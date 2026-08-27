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
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::query;

use crate::{error::AppError, routes::admins::Admin, state::AppState};

pub async fn delete_bot(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let res = query!("delete from bots where id = $1", id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(target:"delete-bot", error=?e, id, by=admin.id, "gotten error while deleting bot");
            AppError::Internal
        })?;
    if res.rows_affected() == 0 {
        Err(AppError::BotNotFound(id))?
    }
    tracing::info!(target:"delete-bot", id, by=admin.id, "deleted bot");
    Ok(StatusCode::NO_CONTENT)
}
