
use std::fmt::{Display};

use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Type, Serialize, Deserialize, PartialOrd)]
#[sqlx(type_name = "user_role", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum UserRole {
    Pool,
    Blocked,
    Deleted,
    User,
    Moderator,
    Admin,
    Owner,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pool => write!(f, "pool"),
            Self::Blocked => write!(f, "blocked"),
            Self::Deleted => write!(f, "deleted"),
            Self::User => write!(f, "user"),
            Self::Moderator => write!(f, "moderator"),
            Self::Admin => write!(f, "admin"),
            Self::Owner => write!(f, "owner")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Type, Serialize, Deserialize, PartialOrd)]
#[sqlx(type_name = "notify_level", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum NotifyLevel {
    No,
    Default,
    All,
}

impl Display for NotifyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No => write!(f, "no"),
            Self::Default => write!(f, "default"),
            Self::All => write!(f, "all")
        }
    }
}

use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive( Clone, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub userhandle: Option<String>,
    pub balance: i64,
    pub role: UserRole,
    pub notify_level: NotifyLevel,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
