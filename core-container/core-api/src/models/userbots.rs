use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Userbot {
    pub id: i64,
    pub owner_id: i64,
    pub relevancy: i64,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
