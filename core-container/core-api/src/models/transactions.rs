use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Type)]
#[sqlx(type_name = "tx_type", rename_all = "kebab-case")]
pub enum TxType {
    CreateUser,
    Transfer,
    StockPayment,
    Moderation,
    Zero,
}

use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub amount: i64,
    pub executor: i64,
    pub data: Option<Value>,
    pub r#type: TxType,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
