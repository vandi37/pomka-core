use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum UserRole {
    Pool,
    Blocked,
    User,
    Moderator,
    Admin,
    Owner,
}

#[derive(Debug, Clone, Copy, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "notify_level", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum NotifyLevel {
    No,
    Default,
    All,
}

use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub balance: i64,
    pub role: UserRole,
    pub notify_level: NotifyLevel,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
