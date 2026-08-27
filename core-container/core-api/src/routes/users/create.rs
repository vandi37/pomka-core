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
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::query_as;

use crate::{error::AppError, models::users::User, routes::{Executor, users::VALID_USER_NAME}, state::AppState};

#[derive(Deserialize, Clone)]
pub struct CreateUser {
    pub name: String,
}
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(create): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    if !VALID_USER_NAME.contains(&create.name.len()) {
        return Err(AppError::InvalidUserName(create.name))
    }
    let mut tx = state.db.begin().await.map_err(|e|{
        tracing::error!(target: "create-user", error=?e, executor=executor.id, name=create.name, "gotten error while creating transaction");
        AppError::Internal
    })?;
    let res = query_as::<_, User>( "insert into users (name) values ($1) returning id, name, userhandle, balance, role, notify_level, updated_at, created_at")
        .bind(&create.name)
        .fetch_one(tx.as_mut())
        .await
        .map_err(|e|{
            tracing::error!(target: "create-user", error=?e, executor=executor.id, name=create.name, "gotten error while creating user");
            AppError::Internal
        })?;
    tx.commit().await.map_err(|e|{
        tracing::error!(target: "create-user", error=?e, executor=executor.id, id=res.id, name=res.name, "gotten error while committing transaction");
        AppError::Internal
    })?;

    tracing::debug!(target:"create-user", executor=executor.id, id=res.id, name=res.name, "created new user");
    Ok((StatusCode::CREATED, Json(res)))
}
