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
    Extension, Json, extract::State, http::StatusCode, response::IntoResponse,
};
use rust_decimal::{Decimal, dec};
use serde::Deserialize;
use sqlx::query;

use crate::{utils::error::AppError, routes::admins::Admin, utils::state::AppState};

#[derive(Deserialize)]
pub struct Fee {
    pub fee: Decimal,
}

pub async fn patch_admin_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(fee): Json<Fee>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.fee.clamp(dec!(0), dec!(1));
    query!("update global_config set admin_fee=cast($1 * cast(scale as decimal(20, 10)) as int);", fee).execute(&state.db).await
        .map_err(|e| {
            tracing::error!(target:"patch-admin-fee", error=?e, by=admin.id, "gotten error while patching admin fee");
            AppError::Internal
        })?;
    tracing::debug!(target:"patch-admin-fee", by=admin.id, "patched admin fee");
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_bot_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(fee): Json<Fee>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.fee.clamp(dec!(0), dec!(1));

    query!("update global_config set bot_fee=cast($1 * cast(scale as decimal(20, 10)) as int);", fee).execute(&state.db).await
        .map_err(|e| {
            tracing::error!(target:"patch-bot-fee", error=?e, by=admin.id, "gotten error while patching bot fee");
            AppError::Internal
        })?;
    tracing::debug!(target:"patch-bot-fee", by=admin.id, "patched bot fee");
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_userbot_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(fee): Json<Fee>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.fee.clamp(dec!(0), dec!(1));

    query!("update global_config set userbot_fee=cast($1 * cast(scale as decimal(20, 10)) as int);", fee).execute(&state.db).await
        .map_err(|e| {
            tracing::error!(target:"patch-userbot-fee", error=?e, by=admin.id, "gotten error while patching userbot fee");
            AppError::Internal
        })?;
    tracing::debug!(target:"patch-userbot-fee", by=admin.id, "patched userbot fee");
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_userbot_user_token_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(fee): Json<Fee>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.fee.clamp(dec!(0), dec!(1));

    query!("update global_config set userbot_user_token_fee=cast($1 * cast(scale as decimal(20, 10)) as int);", fee).execute(&state.db).await
        .map_err(|e| {
            tracing::error!(target:"patch-userbot-user-token-fee", error=?e, by=admin.id, "gotten error while patching userbot user token fee");
            AppError::Internal
        })?;
    tracing::debug!(target:"patch-userbot-user-token-fee", by=admin.id, "patched userbot user token fee");
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_userbot_owner_fee(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(fee): Json<Fee>,
) -> Result<impl IntoResponse, AppError> {
    let fee = fee.fee.clamp(dec!(0), dec!(1));
    query!("update global_config set userbot_owner_fee=cast($1 * cast(scale as decimal(20, 10)) as int);", fee).execute(&state.db).await
        .map_err(|e| {
            tracing::error!(target:"patch-userbot-owner-fee", error=?e, by=admin.id, "gotten error while patching userbot owner fee");
            AppError::Internal
        })?;
    tracing::debug!(target:"patch-userbot-owner-fee", by=admin.id, "patched userbot owner fee");
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct DailyClaim {
    pub daily_claim: i64,
}

pub async fn patch_daily_claim(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(daily_claim): Json<DailyClaim>,
) -> Result<impl IntoResponse, AppError> {
    query!("update global_config set daily_claim=$1", daily_claim.daily_claim).execute(&state.db).await.map_err(|e| {
            tracing::error!(target:"patch-daily-claim", error=?e, by=admin.id, "gotten error while patching daily claim");
            AppError::Internal
        })?;
    tracing::debug!(target:"patch-daily-claim", by=admin.id, "patched daily claim");
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct ResetHour {
    pub reset_hour: i32,
}

pub async fn patch_reset_hour(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(reset_hour): Json<ResetHour>,
) -> Result<impl IntoResponse, AppError> {
    query!("update global_config set reset_hour=$1", reset_hour.reset_hour.clamp(0, 24)).execute(&state.db).await.map_err(|e| {
            tracing::error!(target:"patch-reset-hour", error=?e, by=admin.id, "gotten error while patching reset hour");
            AppError::Internal
        })?;
    tracing::debug!(target:"patch-reset-hour", by=admin.id, "patched reset hour");
    Ok(StatusCode::NO_CONTENT)
}