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

use axum::{Json, extract::{Extension, State}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use sqlx::{query, query_as};

use crate::{error::AppError, models::{executors::ExecutorType, users::{ User, UserRole}}, routes::Executor, services::{executors::get_executor, users}, state::AppState};

#[derive(Clone, Deserialize)]
pub struct IdAndBy {
    pub id: i64,
    pub by: i64,
}
pub async fn remove_user_handle(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(handle): Json<IdAndBy>,
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

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(id_and_by): Json<IdAndBy>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.db.begin().await.map_err(|e|{
       tracing::error!(target: "delete-user", 
            error=?e, 
            executor=executor.id, 
            by=id_and_by.by, 
            id=id_and_by.id, 
            "gotten error while creating transaction");
        AppError::Internal
    })?;

    let executor_row = get_executor(tx.as_mut(), executor.id).await.map_err(|e| {
        tracing::error!(target: "delete-user", 
            error=?e, 
            executor=executor.id, 
            by=id_and_by.by, 
            id=id_and_by.id, 
            "gotten error while getting executor");
        AppError::Internal
    })?.ok_or(AppError::ExecutorNotFound(executor.id))?;

    if executor_row.executor_type == ExecutorType::Userbot {
        Err(AppError::ExecutorForbidden(executor_row.id))?
    }

    let by = users::get_user(tx.as_mut(), id_and_by.by).await.map_err(|e|{
        tracing::error!(target: "delete-user", 
            error=?e, 
            executor=executor.id, 
            by=id_and_by.by, 
            id=id_and_by.id, 
            "gotten error while getting by-user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(id_and_by.by))?;

    let user = users::get_user_for_update(tx.as_mut(), id_and_by.id).await.map_err(|e|{
        tracing::error!(target: "delete-user", 
            error=?e, 
            executor=executor.id, 
            by=id_and_by.by, 
            id=id_and_by.id, 
            "gotten error while getting user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(id_and_by.id))?;

    if executor_row.executor_type != ExecutorType::Admin &&
         (
            by.role < UserRole::Admin || 
            user.role == UserRole::Pool || 
            user.role >= by.role
        ) && id_and_by.id != id_and_by.by{
        Err(AppError::UserForbidden(by.id))?
    }

    query!("update users set name=null, userhandle=null, role='deleted', notify_level='no' where id=$1", user.id)
        .execute(tx.as_mut())
        .await
        .map_err(|e: sqlx::Error|{
            tracing::error!(target: "delete-user", 
                error=?e, 
                executor=executor.id, 
                by=id_and_by.by, 
                id=id_and_by.id, 
            "gotten error while deleting user");
        AppError::Internal
        })?;
    let userbots_deleted = query!("delete from userbots where owner_id=$1", user.id)
        .execute(tx.as_mut())
        .await
        .map_err(|e: sqlx::Error|{
            tracing::error!(target: "delete-user", 
                error=?e, 
                executor=executor.id, 
                by=id_and_by.by, 
                id=id_and_by.id, 
            "gotten error while deleting user's userbots");
        AppError::Internal
        })?.rows_affected();
    let user_tokens_deleted = query!("delete from user_tokens where user_id=$1", user.id)
        .execute(tx.as_mut())
        .await
        .map_err(|e: sqlx::Error|{
            tracing::error!(target: "delete-user", 
                error=?e, 
                executor=executor.id, 
                by=id_and_by.by, 
                id=id_and_by.id, 
            "gotten error while deleting user's user tokens");
        AppError::Internal
        })?.rows_affected();
    tx.commit().await.map_err(|e|{
        tracing::error!(target: "delete-user", 
            error=?e, 
            executor=executor.id, 
            by=id_and_by.by, 
            id=id_and_by.id, 
            "gotten error while committing transaction");
        AppError::Internal
    })?;
     tracing::debug!(target: "update-user-role", 
            executor=executor.id, 
            by=id_and_by.by, 
            id=id_and_by.id, 
            userbots_deleted=userbots_deleted,
            user_tokens_deleted=user_tokens_deleted,
            "deleted user");
    Ok(StatusCode::NO_CONTENT)
}