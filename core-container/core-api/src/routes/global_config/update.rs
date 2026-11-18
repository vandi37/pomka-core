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

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use rust_decimal::{Decimal, dec};
use serde::Deserialize;
use sqlx::query;

use crate::{routes::admins::Admin, utils::error::AppError, utils::state::AppState};

#[derive(Deserialize)]
pub struct Fee {
    pub fee: Decimal,
}

#[derive(Deserialize)]
pub struct UpdateGlobalConfig {
    pub admin_fee: Option<Decimal>,
    pub bot_fee: Option<Decimal>,
    pub userbot_fee: Option<Decimal>,
    pub userbot_user_token_fee: Option<Decimal>,
    pub userbot_owner_fee: Option<Decimal>,
    pub daily_claim: Option<i64>,
    pub reset_hour: Option<i32>,
}

pub async fn update_global_config(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(global_config): Json<UpdateGlobalConfig>,
) -> Result<impl IntoResponse, AppError> {
    if global_config.admin_fee.is_none()
        && global_config.bot_fee.is_none()
        && global_config.userbot_fee.is_none()
        && global_config.userbot_user_token_fee.is_none()
        && global_config.userbot_owner_fee.is_none()
        && global_config.daily_claim.is_none()
        && global_config.reset_hour.is_none()
    {
        return Err(AppError::EmptyPatch);
    }

    let global_config = UpdateGlobalConfig {
        admin_fee: global_config.admin_fee.map(|f| f.clamp(dec!(0), dec!(1))),
        bot_fee: global_config.bot_fee.map(|f| f.clamp(dec!(0), dec!(1))),
        userbot_fee: global_config.userbot_fee.map(|f| f.clamp(dec!(0), dec!(1))),
        userbot_user_token_fee: global_config.userbot_user_token_fee.map(|f| f.clamp(dec!(0), dec!(1))),
        userbot_owner_fee: global_config.userbot_owner_fee.map(|f| f.clamp(dec!(0), dec!(1))),
        daily_claim: global_config.daily_claim,
        reset_hour: global_config.reset_hour.map(|r| r.clamp(0, 24)),
    };
    query!(r#"update global_config set
        admin_fee=coalesce(cast($1 * cast(scale as decimal(20, 10)) as int), admin_fee),
        bot_fee=coalesce(cast($2 * cast(scale as decimal(20, 10)) as int), bot_fee),
        userbot_fee=coalesce(cast($3 * cast(scale as decimal(20, 10)) as int), userbot_fee),
        userbot_user_token_fee=coalesce(cast($4 * cast(scale as decimal(20, 10)) as int), userbot_user_token_fee),
        userbot_owner_fee=coalesce(cast($5 * cast(scale as decimal(20, 10)) as int), userbot_owner_fee),
        daily_claim=coalesce($6, daily_claim),
        reset_hour=coalesce($7, reset_hour);
    "#, 
        global_config.admin_fee, 
        global_config.bot_fee, 
        global_config.userbot_fee, 
        global_config.userbot_user_token_fee, 
        global_config.userbot_owner_fee, 
        global_config.daily_claim,
        global_config.reset_hour,
    ).execute(&state.db).await.map_err(|e| {
            tracing::error!(target:"update-global-config", error=?e, by=admin.id, "gotten error while updating global config");
            AppError::Internal
        })?;

    tracing::info!(target:"update-global-config", by=admin.id, "updated global config");
    Ok(StatusCode::NO_CONTENT)
}