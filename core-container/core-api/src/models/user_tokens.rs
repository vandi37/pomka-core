use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserToken {
    pub id: i64,
    pub user_id: i64,
    pub amount: Option<i64>,
    pub till: Option<DateTime<Utc>>,
    pub used: i64,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
