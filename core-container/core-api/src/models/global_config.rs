use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct GlobalConfig {
    pub id: i32,
    pub control_pool: i64,
    pub updated_at: DateTime<Utc>,
}
