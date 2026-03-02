use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Stock {
    pub id: i64,
    pub owner_id: i64,
    pub capacity: i64,
    pub left_amount: i64,
    pub base: i64,
    pub power: f64,
    pub executor: i64,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
