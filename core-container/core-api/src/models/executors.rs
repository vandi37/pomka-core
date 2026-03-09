use sqlx::Type;

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
    Userbot(i64),
}

use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Executor {
    pub id: i64,
    pub executor_type: ExecutorType,
    pub admin: Option<i64>,
    pub bot: Option<i64>,
    pub userbot: Option<i64>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Executor {
    pub fn get_ref(&self) -> Option<ExecutorRef> {
        match self.executor_type {
            ExecutorType::Admin => self.admin.map(|a| ExecutorRef::Admin(a)),
            ExecutorType::Bot => self.bot.map(|b| ExecutorRef::Bot(b)),
            ExecutorType::Userbot => self.bot.map(|u| ExecutorRef::Userbot(u))
        }
    }
}