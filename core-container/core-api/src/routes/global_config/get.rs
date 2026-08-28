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

use axum::{Extension, Json, extract::State, response::IntoResponse};
use serde::Serialize;
use sqlx::query;

use crate::{ error::AppError, routes::Executor, state::AppState};

#[derive( Clone, Serialize)]
pub struct Fees {
    admin: f32,
    bot: f32,
    userbot: f32,
    userbot_user_token: f32,
    userbot_owner: f32
}

pub async fn get_fees(
     State(state): State<Arc<AppState>>,
     Extension(executor): Extension<Executor>,
) -> Result<impl IntoResponse, AppError> {
    let fees = query!("select admin_fee, bot_fee, userbot_fee, userbot_user_token_fee, userbot_owner_fee from global_config")
        .fetch_one(&state.db)
        .await.map_err(|e| {
            tracing::error!(target:"get-fees", error=?e, executor=executor.id, "gotten error while getting fees");
            AppError::Internal
        })?;
    let fees = Fees {
        admin: fees.admin_fee,
        bot: fees.bot_fee,
        userbot: fees.userbot_fee,
        userbot_user_token: fees.userbot_user_token_fee,
        userbot_owner: fees.userbot_owner_fee,
    };
    tracing::debug!(target:"get-fees", executor=executor.id, "gotten fees");
    Ok(Json(fees))
}