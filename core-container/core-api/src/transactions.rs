use redis::Client;
use serde_json::Value;
use sqlx::{PgPool, PgTransaction, query, query_as};
use uuid::Uuid;

use crate::{
    models::{
        transactions::Transaction,
        users::{NotifyLevel, UserRole},
    },
    services::{executors, users},
};

pub struct Pay {
    pub sender: i64,
    pub receiver: i64,
    pub amount: i64,
    pub executor: i64,
    pub data: Option<Value>,
    pub r#type: String,
    pub allowed: bool,
    pub idempotency_key: Option<Uuid>,
}

pub enum TxError {
    Sqlx(sqlx::Error),
    Redis(redis::RedisError),
    Serde(serde_json::Error),
    NegativeAmount,
    UserNotFound(i64),
    ExecutorNotFound(i64),
    InsufficientFunds,
    Forbidden(i64),
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

    let (first_id, second_id) = if pay.sender < pay.receiver {
        (pay.sender, pay.receiver)
    } else {
        (pay.receiver, pay.sender)
    };
    let first = users::get_user_for_update(tx.as_mut(), first_id)
        .await
        .map_err(TxError::Sqlx)?
        .ok_or(TxError::UserNotFound(first_id))?;

    let second = users::get_user_for_update(tx.as_mut(), second_id)
        .await
        .map_err(TxError::Sqlx)?
        .ok_or(TxError::UserNotFound(second_id))?;

    let (sender, receiver) = if first_id == pay.sender{
        (first, second)
    } else {
        (second, first)
    };
    
    if let Some(u) = exec.userbot {
        if sender.role == UserRole::Pool {
            return Err(TxError::Forbidden(exec.id));
        }
        let userbot = query!("select id, owner_id from userbots where id = $1", u)
            .fetch_one(tx.as_mut())
            .await
            .map_err(TxError::Sqlx)?;
        if userbot.owner_id != sender.id && !pay.allowed {
            return Err(TxError::Forbidden(exec.id));
        }
    };
    if sender.balance < pay.amount && sender.role != UserRole::Pool {
        return Err(TxError::InsufficientFunds);
    }
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
        pay.amount,
        pay.receiver
    )
    .execute(tx.as_mut())
    .await
    .map_err(TxError::Sqlx)?;

    let res = query_as!(Transaction, r#"insert into transactions (idempotency_key, sender_id, receiver_id, amount, executor, data, type)
        values ($1, $2, $3, $4, $5, $6, $7)
        returning id, idempotency_key, sender_id, receiver_id, amount, executor, data, type, updated_at, created_at"#,
        pay.idempotency_key,
        pay.sender,
        pay.receiver,
        pay.amount,
        exec.id,
        pay.data,
        pay.r#type
    ).fetch_one(tx.as_mut()).await.map_err(TxError::Sqlx)?;
    
    Ok((res, sender.notify_level, receiver.notify_level))
}

pub async fn pay(pay: Pay, pool: &PgPool, client: &Client) -> Result<Transaction, TxError> {
    let (sender_id, receiver_id) = (pay.sender, pay.receiver);
    let mut tx = pool.begin().await.map_err(TxError::Sqlx)?;
    let (res, sender_notify_level, receiver_notify_level) = pay_tx(pay, &mut tx).await?;
    tx.commit().await.map_err(TxError::Sqlx)?;
    notify_about_transaction(client, sender_id, sender_notify_level, receiver_id, receiver_notify_level, &res).await?;
    Ok(res)
}

pub async fn notify_about_transaction( client: &Client,sender_id: i64, sender_notify_level: NotifyLevel, receiver_id:i64, receiver_notify_level: NotifyLevel, tx: &Transaction) -> Result<(), TxError> {
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
) -> Result<(), TxError> {
    let mut conn = client
        .get_connection_manager()
        .await
        .map_err(TxError::Redis)?;

    let payload = serde_json::to_string(tx).map_err(TxError::Serde)?;
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
    .map_err(TxError::Redis)
}
