// The Pomka Ecosystem Core Source Code
// Copyright (C) 2026 Lev (Leo) Kondukov (aka DiceBarrel, Barrel, Vandi)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use redis::Client;
use serde_json::Value;
use sqlx::{PgPool, PgTransaction, query, query_as};
use uuid::Uuid;

use crate::{
    models::{
        transactions::Transaction,
        users::{NotifyLevel, UserRole},
    },
    services::{executors, global_config::get_fees, users},
};

pub struct Pay {
    pub sender: i64,
    pub receiver: i64,
    pub amount: i64,
    pub executor: i64,
    pub data: Option<Value>,
    pub r#type: String,
    pub allowed: TxAllowance,
    pub idempotency_key: Option<Uuid>,
}

pub enum TxAllowance {
    OneTimeAllowed,
    UserTokenAllowed,
    NotAllowed, // or allowed as the userbot owner
}

#[derive( Debug)]
pub enum TxError {
    Sqlx(sqlx::Error),
    NegativeAmount,
    UserNotFound(i64),
    ExecutorNotFound(i64),
    InsufficientFunds,
    Forbidden(i64),
    Overflow,
    DuplicateIdempotencyKey,
}

#[derive(Debug)]
pub enum SendingError {
    Redis(redis::RedisError),
    Serde(serde_json::Error),
}
// Don't forget to notify the user with notify_about_transaction() after the database transaction is committed
pub async fn pay_tx<'a>(
    pay: Pay,
    tx: &mut PgTransaction<'a>,
) -> Result<(Transaction, NotifyLevel, NotifyLevel), TxError> {
    if pay.amount < 0 {
        return Err(TxError::NegativeAmount);
    }
    let exec = executors::get_executor(tx.as_mut(), pay.executor)
        .await
        .map_err(TxError::Sqlx)?
        .ok_or(TxError::ExecutorNotFound(pay.executor))?;

    let sender = users::get_user_for_update(tx.as_mut(), pay.sender)
        .await
        .map_err(TxError::Sqlx)?
        .ok_or(TxError::UserNotFound(pay.sender))?;

    let receiver = users::get_user(tx.as_mut(), pay.receiver)
        .await
        .map_err(TxError::Sqlx)?
        .ok_or(TxError::UserNotFound(pay.receiver))?;

    let fees = get_fees(tx.as_mut()).await.map_err(TxError::Sqlx)?;
    let mut fee = fees.bot;
    if let Some(_) = exec.admin {
        fee = fees.admin
    }

    if let Some(u) = exec.userbot {
        if sender.role == UserRole::Pool {
            return Err(TxError::Forbidden(exec.id));
        }
        let userbot = query!("select id, owner_id from userbots where id = $1", u)
            .fetch_one(tx.as_mut())
            .await
            .map_err(TxError::Sqlx)?;
        match pay.allowed {
            TxAllowance::NotAllowed if userbot.owner_id == sender.id => fee = fees.userbot_owner,
            TxAllowance::OneTimeAllowed => fee = fees.userbot,
            TxAllowance::UserTokenAllowed => fee = fees.userbot_user_token,
            _ => return Err(TxError::Forbidden(exec.id)),
        };
    };
    if sender.balance < pay.amount && sender.role != UserRole::Pool {
        return Err(TxError::InsufficientFunds);
    }

    let received_amount = pay
        .amount
        .checked_mul((fees.scale - fee) as i64)
        .and_then(|v| v.checked_div(fees.scale as i64))
        .ok_or(TxError::Overflow)?;

    query!(
        "update users set balance = balance - $1 where id = $2",
        pay.amount,
        pay.sender
    )
    .execute(tx.as_mut())
    .await
    .map_err(TxError::Sqlx)?;

    query!(
        "update users set balance = balance + $1 where id = $2",
        received_amount,
        pay.receiver
    )
    .execute(tx.as_mut())
    .await
    .map_err(TxError::Sqlx)?;

    query!(
        r#"update users set balance = balance + $1 
        where id in 
            (select id from users 
                where role = 'pool' limit 1
            );"#,
        pay.amount - received_amount,
    )
    .execute(tx.as_mut())
    .await
    .map_err(TxError::Sqlx)?;

    let res = query_as!(Transaction, r#"insert into transactions (idempotency_key, sender_id, receiver_id, amount, executor, data, type)
        values ($1, $2, $3, $4, $5, $6, $7)
        on conflict (idempotency_key) do nothing
        returning id, idempotency_key, sender_id, receiver_id, amount, executor, data, type, updated_at, created_at"#,
        pay.idempotency_key,
        pay.sender,
        pay.receiver,
        pay.amount,
        exec.id,
        pay.data,
        pay.r#type
    ).fetch_optional(tx.as_mut())
        .await
        .map_err(TxError::Sqlx)?
        .ok_or(TxError::DuplicateIdempotencyKey)?;

    Ok((res, sender.notify_level, receiver.notify_level))
}

pub async fn pay(
    pay: Pay,
    pool: &PgPool,
    client: &Client,
) -> Result<(Transaction, Option<SendingError>), TxError> {
    let (sender_id, receiver_id) = (pay.sender, pay.receiver);
    let mut tx = pool.begin().await.map_err(TxError::Sqlx)?;
    let (res, sender_notify_level, receiver_notify_level) = pay_tx(pay, &mut tx).await?;
    tx.commit().await.map_err(TxError::Sqlx)?;
    let err = notify_about_transaction(
        client,
        sender_id,
        sender_notify_level,
        receiver_id,
        receiver_notify_level,
        &res,
    )
    .await
    .err();
    Ok((res, err))
}

pub async fn notify_about_transaction(
    client: &Client,
    sender_id: i64,
    sender_notify_level: NotifyLevel,
    receiver_id: i64,
    receiver_notify_level: NotifyLevel,
    tx: &Transaction,
) -> Result<(), SendingError> {
    if sender_notify_level == NotifyLevel::All {
        publish_transaction(client, &tx, sender_id).await?;
    }
    if receiver_notify_level == NotifyLevel::All {
        publish_transaction(client, &tx, receiver_id).await?;
    }
    Ok(())
}
use redis::AsyncCommands;

pub const TX_STREAM: &'static str = "transactions";
pub const TX_CREATED_EVENT: &'static str = "transaction-created";

pub async fn publish_transaction(
    client: &Client,
    tx: &Transaction,
    user_id: i64,
) -> Result<(), SendingError> {
    let mut conn = client
        .get_connection_manager()
        .await
        .map_err(SendingError::Redis)?;

    let payload = serde_json::to_string(tx).map_err(SendingError::Serde)?;
    conn.xadd(
        TX_STREAM,
        "*",
        &[
            ("event", TX_CREATED_EVENT),
            ("user-id", &user_id.to_string()),
            ("payload", &payload),
        ],
    )
    .await
    .map_err(SendingError::Redis)
}
