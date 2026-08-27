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

use axum::{Json, extract::{Extension, State}, response::IntoResponse};
use serde::Deserialize;
use sqlx::query_as;

use crate::{error::AppError, models::{executors::ExecutorType, users::{ User, UserRole}}, routes::Executor, services::{executors::get_executor, users}, state::AppState};

#[derive(Clone, Deserialize)]
pub struct RemoveHandle {
    pub id: i64,
    pub by: i64,
}
pub async fn remove_user_handle(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(handle): Json<RemoveHandle>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.db.begin().await.map_err(|e|{
       tracing::error!(target: "remove-user-handle", 
            error=?e, 
            executor=executor.id, 
            by=handle.by, 
            id=handle.id, 
            "gotten error while creating transaction");
        AppError::Internal
    })?;
    let executor_row = get_executor(tx.as_mut(), executor.id).await.map_err(|e| {
        tracing::error!(target: "remove-user-handle", 
            error=?e, 
            executor=executor.id, 
            by=handle.by,
            id=handle.id, 
            "gotten error while getting executor");
        AppError::Internal
    })?.ok_or(AppError::ExecutorNotFound(executor.id))?;

    if executor_row.executor_type == ExecutorType::Userbot {
        Err(AppError::ExecutorForbidden(executor_row.id))?
    }
    let by = users::get_user(tx.as_mut(), handle.by).await.map_err(|e|{
        tracing::error!(target: "remove-user-handle", 
            error=?e, 
            executor=executor.id, 
            by=handle.by,
            id=handle.id,  
            "gotten error while getting by-user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(handle.by))?;

    if by.role < UserRole::User || by.role < UserRole::Moderator && handle.by != handle.id {
        Err(AppError::UserForbidden(by.id))?
    }

    let res = query_as::<_, User>( "update users set userhandle=null where id=$2 returning id, name, userhandle, balance, role, notify_level, updated_at, created_at")
        .bind(handle.id)
        .fetch_optional(tx.as_mut())
        .await
        .map_err(|e: sqlx::Error|{
            tracing::error!(target: "remove-user-handle", 
            error=?e, 
            executor=executor.id, 
            by=handle.by,
            id=handle.id,  
            "gotten error while removing user handle");
        AppError::Internal
        })?.ok_or(AppError::UserNotFound(handle.id))?;

    tx.commit().await.map_err(|e|{
        tracing::error!(target: "remove-user-handle", 
            error=?e, 
            executor=executor.id, 
            by=handle.by,
            id=handle.id,  
            "gotten error while committing transaction");
        AppError::Internal
    })?;
     tracing::debug!(target: "remove-user-handle", 
            executor=executor.id, 
            by=handle.by,
            id=handle.id,  
            "removed user handle");
    Ok(Json(res))
}