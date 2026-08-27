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

use sqlx::{Executor, Postgres, query_as};

use crate::models::executors::ExecutorRow;

pub async fn get_executor<'e>(executor: impl Executor<'e, Database = Postgres>, id: i64) -> Result<Option<ExecutorRow>, sqlx::Error> {
     query_as::<_, ExecutorRow>(
        "select id, executor_type, admin, bot, userbot, updated_at, created_at from executors where id = $1"
    )
        .bind(id)
        .fetch_optional(executor)
        .await
}