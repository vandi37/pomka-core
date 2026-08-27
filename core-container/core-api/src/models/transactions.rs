use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub idempotency_key: Option<Uuid>,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub amount: i64,
    pub executor: i64,
    pub data: Option<Value>,
    pub r#type: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
