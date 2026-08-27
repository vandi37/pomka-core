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

use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::query_as;

use crate::{
    error::AppError,
    routes::{admins::Admin, bots::BotRes},
    state::AppState,
};
#[derive(Deserialize, Clone)]
pub struct UpdateBot {
    pub username: Option<String>,
    pub password: Option<String>,
}

pub async fn update_bot(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(id): Path<i64>,
    Json(bot): Json<UpdateBot>,
) -> Result<impl IntoResponse, AppError> {
    if bot.username.is_none() && bot.password.is_none() {
        Err(AppError::EmptyPatch)?
    }
    let password = bot.password.and_then(|p| Some(state.password_hasher_service.hash_password(&p)
        .map_err(|e| {
            tracing::error!(target: "update-bot", error=?e, id, by=admin.id, "gotten error while hashing bot password");
            AppError::Internal
    }))).transpose()?;
    let res = query_as!(BotRes ,r#"
update bots
set
    username = coalesce($1, username), 
    password = coalesce($2, password)
where id = $3
returning id, username, creator, updated_at, created_at"#,
        bot.username, password, id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| match (e, bot.username) {
               (sqlx::Error::Database(db_err), Some(u)) if db_err.is_unique_violation() => AppError::BotUsernameTaken(u),
                (e, _) => {
                    tracing::error!(target: "update-bot", error=?e, creator=admin.id, "gotten error while updating bot");
                    AppError::Internal
                }
            }
        )?
        .ok_or(AppError::BotNotFound(id))?;
    tracing::info!(target:"update-bot", username=res.username, id=res.id, creator=res.creator, by=admin.id, "updated bot");
    Ok((StatusCode::OK, Json(res)))
}
