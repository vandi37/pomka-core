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

use crate::{ routes::Executor, services::global_config::{self, DailyClaim, Fees}, utils::{error::AppError, state::AppState}};

#[derive(Serialize)]
struct GlobalConfig {
    #[serde(flatten)]
    fees: Fees,
    #[serde(flatten)]
    daily_claim: DailyClaim,
}

pub async fn get_global_config(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
) -> Result<impl IntoResponse, AppError> {
    let fees = global_config::get_fees(&state.db).await.map_err(|e| {
            tracing::error!(target:"get-global-config", error=?e, executor=executor.id, "gotten error while getting fees");
            AppError::Internal
        })?;
    
    let daily_claim = global_config::get_daily_claim(&state.db).await.map_err(|e| {
            tracing::error!(target:"get-global-config", error=?e, executor=executor.id, "gotten error while getting daily claim");
            AppError::Internal
        })?;

    tracing::debug!(target:"get-global-config", executor=executor.id, "gotten global config");
    Ok(Json(GlobalConfig{fees, daily_claim}))
}
