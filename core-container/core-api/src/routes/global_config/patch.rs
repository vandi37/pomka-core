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

use axum::{Extension, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use sqlx::query;

use crate::{error::AppError, routes::admins::Admin, state::AppState};

pub async fn patch_admin_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(fee): Path<f32>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.max(1.0).min(0.0);
    query!("update global_config set admin_fee=$1;", fee).execute(&state.db).await
         .map_err(|e| {
            tracing::error!(target:"patch-admin-fee", error=?e, by=admin.id, "gotten error while patching admin fee");
            AppError::Internal
        })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_bot_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(fee): Path<f32>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.max(1.0).min(0.0);
    query!("update global_config set bot_fee=$1;", fee).execute(&state.db).await
         .map_err(|e| {
            tracing::error!(target:"patch-bot-fee", error=?e, by=admin.id, "gotten error while patching bot fee");
            AppError::Internal
        })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_userbot_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(fee): Path<f32>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.max(1.0).min(0.0);
    query!("update global_config set userbot_fee=$1;", fee).execute(&state.db).await
         .map_err(|e| {
            tracing::error!(target:"patch-userbot-fee", error=?e, by=admin.id, "gotten error while patching userbot fee");
            AppError::Internal
        })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_userbot_user_token_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(fee): Path<f32>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.max(1.0).min(0.0);
    query!("update global_config set userbot_user_token_fee=$1;", fee).execute(&state.db).await
         .map_err(|e| {
            tracing::error!(target:"patch-userbot-user-token-fee", error=?e, by=admin.id, "gotten error while patching userbot user token fee");
            AppError::Internal
        })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_userbot_owner_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(fee): Path<f32>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.max(1.0).min(0.0);
    query!("update global_config set userbot_owner_fee=$1;", fee).execute(&state.db).await
         .map_err(|e| {
            tracing::error!(target:"patch-userbot-owner-fee", error=?e, by=admin.id, "gotten error while patching userbot owner fee");
            AppError::Internal
        })?;
    Ok(StatusCode::NO_CONTENT)
}