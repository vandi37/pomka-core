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
