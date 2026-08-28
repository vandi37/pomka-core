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

use crate::{ error::AppError, routes::Executor, services::global_config, state::AppState};

pub async fn get_fees(
     State(state): State<Arc<AppState>>,
     Extension(executor): Extension<Executor>,
) -> Result<impl IntoResponse, AppError> {
    let fees = global_config::get_fees(&state.db).await.map_err(|e| {
            tracing::error!(target:"get-fees", error=?e, executor=executor.id, "gotten error while getting fees");
            AppError::Internal
        })?;

    tracing::debug!(target:"get-fees", executor=executor.id, "gotten fees");
    Ok(Json(fees))
}