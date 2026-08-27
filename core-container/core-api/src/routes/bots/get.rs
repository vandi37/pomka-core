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
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::query_as;

use crate::{error::AppError, routes::{MAX_LIMIT, Params, admins::Admin, bots::BotRes}, state::AppState};

pub async fn get_bot(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let bot = query_as!(BotRes, "select id, username, creator, updated_at, created_at from bots where id = $1", id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(target:"get-bot", error=?e, id, by=admin.id, "gotten error while getting bot");
            AppError::Internal
        })?
        .ok_or(AppError::BotNotFound(id))?;
    tracing::debug!(target:"get-bot", id, by=admin.id, "gotten bot");
    Ok((StatusCode::OK, Json(bot)))
}

pub async fn get_bots(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Query(mut params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    params.limit = params.limit.min(MAX_LIMIT).max(1);
    params.offset = params.offset.max(1);
    let bots = query_as!(BotRes, "select id, username, creator, updated_at, created_at from bots order by id asc limit $1 offset $2", params.limit, params.offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
              tracing::error!(target:"get-bots", error=?e, by=admin.id, limit=params.limit, offset=params.offset, "gotten error while getting bots");
            AppError::Internal
        })?;
    tracing::debug!(target:"get-bots", by=admin.id, limit=params.limit, offset=params.offset, len=bots.len(), "gotten bots");
    Ok((StatusCode::OK, Json(bots)))
}
