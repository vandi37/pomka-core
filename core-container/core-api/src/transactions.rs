use redis::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, PgTransaction, Postgres, query, query_as};

use crate::{models::{executors::{ExecutorRow, ExecutorType}, transactions::Transaction, users::{NotifyLevel, User, UserRole}}, services::{executors, users}};

pub struct Pay {
    pub sender: i64, 
    pub receiver: i64, 
    pub amount: i64, 
    pub executor: i64,
    pub data: Option<Value>,
    pub r#type: String,
    pub allowed: bool
}

pub enum TxError{
    Sqlx(sqlx::Error),
    Redis(redis::RedisError),
    Serde(serde_json::Error),
    NegativeAmount,
    UserNotFound(i64),
    ExecutorNotFound(i64),
    InsufficientFunds,
    Forbidden
}

pub async fn pay_tx<'a>(
    pay: Pay,
    tx: &mut PgTransaction<'a>,
    client: &Client
) -> Result<Transaction, TxError> {
    if pay.amount < 0 {
        return Err(TxError::NegativeAmount)
    }
    let exec = executors::get_executor(tx.as_mut(),pay.executor)
        .await.map_err(TxError::Sqlx)?.ok_or(TxError::ExecutorNotFound(pay.executor))?;
    
    let sender = users::get_user(tx.as_mut(), pay.sender)
        .await
        .map_err(TxError::Sqlx)?
        .ok_or(TxError::UserNotFound(pay.sender))?;
    let receiver = users::get_user(tx.as_mut(), pay.receiver)
        .await
        .map_err(TxError::Sqlx)?
        .ok_or(TxError::UserNotFound(pay.receiver))?;

     if let Some(u) = exec.userbot {
        let userbot = query!("select id, owner_id from userbots where id = $1", u)
        .fetch_one(tx.as_mut())
        .await.map_err(TxError::Sqlx)?;
        if userbot.owner_id != sender.id && !pay.allowed {
            return Err(TxError::Forbidden)
        }
    };
    if exec.executor_type != ExecutorType::Admin && sender.role == UserRole::Pool {
        return Err(TxError::Forbidden)
    }
    if sender.balance < pay.amount && sender.role != UserRole::Pool {
        return Err(TxError::InsufficientFunds)
    }
    query!("update users set balance = balance - $1 where id = $2", pay.amount, pay.sender)
        .execute(tx.as_mut()).await.map_err(TxError::Sqlx)?;
    query!("update users set balance = balance + $1 where id = $2", pay.amount, pay.receiver)
        .execute(tx.as_mut()).await.map_err(TxError::Sqlx)?;

    let res = query_as!(Transaction, r#"insert into transactions (sender_id, receiver_id, amount, executor, data, type)
        values ($1, $2, $3, $4, $5, $6)
        returning id, sender_id, receiver_id, amount, executor, data, type, updated_at, created_at"#,
        pay.sender,
        pay.receiver,
        pay.amount,
        exec.id,
        pay.data,
        pay.r#type
    ).fetch_one(tx.as_mut()).await.map_err(TxError::Sqlx)?;
    if sender.notify_level == NotifyLevel::All {
        publish_transaction(client, &res, sender.id).await?;
    }
    if receiver.notify_level == NotifyLevel::All {
        publish_transaction(client, &res, receiver.id).await?;
    }
    Ok(res)
}

pub async fn pay(pay: Pay, pool: &PgPool, client: &Client) -> Result<Transaction, TxError> {
    let mut tx = pool.begin().await.map_err(TxError::Sqlx)?;
    let res = pay_tx(pay, &mut tx, client).await?;
    tx.commit().await.map_err(TxError::Sqlx)?;
    Ok(res)
}

use redis::AsyncCommands;

pub const TX_STREAM: &'static str = "transactions";
pub const TX_CREATED_EVENT: &'static str = "transaction-created";

pub async fn publish_transaction(client: &Client, tx: &Transaction, user_id: i64 ) -> Result<(), TxError> {
    let mut conn = client.get_connection_manager().await.map_err(TxError::Redis)?;

    let payload = serde_json::to_string(tx)
        .map_err(TxError::Serde)?;
    conn.xadd(
        TX_STREAM,
        "*",
        &[
            ("event", TX_CREATED_EVENT), 
            ("user-id", &user_id.to_string()),
            ("payload", &payload)
        ],
    )
    .await.map_err(TxError::Redis)
}