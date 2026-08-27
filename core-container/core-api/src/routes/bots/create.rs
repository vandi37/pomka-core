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
    Json, extract::{Extension, State}, http::StatusCode, response::IntoResponse
};
use sqlx::query_as;

use crate::{
    error::AppError,
    routes::{admins::Admin, bots::{BotRes, InputBot}},
    state::AppState,
};

pub async fn create_bot(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(login): Json<InputBot>,
) -> Result<impl IntoResponse, AppError> {
    let password = state.password_hasher_service.hash_password(&login.password)
        .map_err(|e| {
            tracing::error!(target: "create-bot", error=?e, username=login.username, creator=admin.id, "gotten error while hashing bot password");
            AppError::Internal
    })?;
    let res = query_as!(BotRes ,"insert into bots (username, password, creator) values ($1, $2, $3) returning id, username, creator, updated_at, created_at",
        login.username, password, admin.id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => AppError::BotUsernameTaken(login.username.clone()),
                e => {
                    tracing::error!(target: "create-bot", error=?e, username=login.username, creator=admin.id, "gotten error while creating bot");
                    AppError::Internal
                }
            }
        )?;
    tracing::info!(target:"create-bot", username=res.username, id=res.id, creator=res.creator, "created new bot");
    Ok((StatusCode::CREATED, Json(res)))
}
