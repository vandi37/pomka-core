// The Pomka Ecosystem Core Source Code
// Copyright (C) 2026 Lev (Leo) Kondukov (aka DiceBarrel, Barrel, Vandi)
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt::Display;

use sqlx::{Type, postgres::PgRow};

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
pub struct ExecutorRow {
    pub id: i64,
    pub executor_type: ExecutorType,
    pub admin: Option<i64>,
    pub bot: Option<i64>,
    pub userbot: Option<i64>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl ExecutorRow {
    pub fn get_ref(&self) -> Option<ExecutorRef> {
        match self.executor_type {
            ExecutorType::Admin => self.admin.map(|a| ExecutorRef::Admin(a)),
            ExecutorType::Bot => self.bot.map(|b| ExecutorRef::Bot(b)),
            ExecutorType::Userbot => self.bot.map(|u| ExecutorRef::Userbot(u)),
        }
    }
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
        let row = ExecutorRow::from_row(row)?;
        Ok(Executor {
            id: row.id,
            executor_ref: row
                .get_ref()
                .ok_or(sqlx::Error::Decode(Box::new(ExecutorError)))?,
            updated_at: row.updated_at,
            created_at: row.created_at,
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

impl std::error::Error for ExecutorError {}
