use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Admin {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub creator: Option<i64>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
