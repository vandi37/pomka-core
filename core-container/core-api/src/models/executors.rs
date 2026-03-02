use sqlx::Type;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Type)]
#[sqlx(type_name = "executor_type", rename_all = "kebab-case")]
pub enum ExecutorType {
    Admin,
    Bot,
    Userbot,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecutorRef {
    Admin(i64),
    Bot(i64),
    UserBot(i64),
}

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::postgres::PgRow;

#[derive(Debug, Clone, FromRow)]
pub struct ExecutorRow {
    pub id: i64,
    pub executor_type: ExecutorType,
    pub admin: Option<i64>,
    pub bot: Option<i64>,
    pub userbot: Option<i64>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Executor {
    pub id: i64,
    pub executor_ref: ExecutorRef,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
impl<'r> FromRow<'r, PgRow> for Executor {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let exec_row: ExecutorRow = ExecutorRow::from_row(row)?;
        Ok(Executor {
            id: exec_row.id,
            executor_ref: match exec_row.executor_type {
                ExecutorType::Admin => ExecutorRef::Admin(
                    exec_row
                        .admin
                        .ok_or(sqlx::Error::Decode(Box::new(ExecutorError)))?,
                ),
                ExecutorType::Bot => ExecutorRef::Bot(
                    exec_row
                        .bot
                        .ok_or(sqlx::Error::Decode(Box::new(ExecutorError)))?,
                ),
                ExecutorType::Userbot => ExecutorRef::UserBot(
                    exec_row
                        .userbot
                        .ok_or(sqlx::Error::Decode(Box::new(ExecutorError)))?,
                ),
            },
            updated_at: exec_row.updated_at,
            created_at: exec_row.created_at,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ExecutorError;

impl Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}
impl Error for ExecutorError {}
