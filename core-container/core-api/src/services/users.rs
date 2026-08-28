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



use sqlx::{Executor, Postgres, query, query_as};

use crate::{models::users::User};

pub async fn get_user<'e>(executor: impl Executor<'e, Database = Postgres>, id: i64) -> Result<Option<User>, sqlx::Error> {
    query_as::<Postgres, User>(
        "select id, name, userhandle, balance, role, notify_level, updated_at, created_at from users where id = $1"
    )
            .bind(id)
            .fetch_optional(executor)
            .await
}

pub async fn get_user_for_update<'e>(executor: impl Executor<'e, Database = Postgres>, id: i64) -> Result<Option<User>, sqlx::Error> {
    query_as::<Postgres, User>(
        "select id, name, userhandle, balance, role, notify_level, updated_at, created_at from users where id = $1 for update "
    )
            .bind(id)
            .fetch_optional(executor)
            .await
}

pub async fn get_user_by_handle<'e>(executor: impl Executor<'e, Database = Postgres>, userhandle: &str) -> Result<Option<User>, sqlx::Error> {
    query_as::<Postgres, User>(
        "select id, name, userhandle, balance, role, notify_level, updated_at, created_at from users where userhandle = $1"
    )
            .bind(userhandle)
            .fetch_optional(executor)
            .await
}
pub async fn get_pool_id<'e>(executor: impl Executor<'e, Database = Postgres>) -> Result<i64, sqlx::Error> {
    query!("select id from users where role = 'pool'")
        .fetch_one(executor)
        .await
        .and_then(|res| Ok(res.id))
}