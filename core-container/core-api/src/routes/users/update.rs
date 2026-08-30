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

use crate::{error::AppError, models::{executors::ExecutorType, users::{NotifyLevel, User, UserRole}}, routes::{Executor, users::{USER_HANDLE_REGEX, VALID_USER_HANDLE, VALID_USER_NAME}}, services::{executors::get_executor, users}, state::AppState};

#[derive(Clone, Deserialize)]
pub struct UpdateName {
    pub id: i64,
    pub by: i64,
    pub name: String,
}

pub async fn update_user_name(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(name): Json<UpdateName>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.db.begin().await.map_err(|e|{
       tracing::error!(target: "update-user-name", 
            error=?e, 
            executor=executor.id, 
            name=name.name, 
            by=name.by, 
            id=name.id, 
            "gotten error while creating transaction");
        AppError::Internal
    })?;
    let executor_row = get_executor(tx.as_mut(), executor.id).await.map_err(|e| {
        tracing::error!(target: "update-user-name", 
            error=?e, 
            executor=executor.id, 
            name=name.name, 
            by=name.by, 
            id=name.id, 
            "gotten error while getting executor");
        AppError::Internal
    })?.ok_or(AppError::ExecutorNotFound(executor.id))?;

    if executor_row.executor_type == ExecutorType::Userbot {
        Err(AppError::ExecutorForbidden(executor_row.id))?
    }
    if !VALID_USER_NAME.contains(&name.name.len()) {
        return Err(AppError::InvalidUserName(name.name))
    }
    let by = users::get_user(tx.as_mut(), name.by).await.map_err(|e|{
        tracing::error!(target: "update-user-name", 
            error=?e, 
            executor=executor.id, 
            name=name.name, 
            by=name.by, 
            id=name.id, 
            "gotten error while getting by-user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(name.by))?;

    if by.role < UserRole::User || by.role < UserRole::Moderator && name.by != name.id {
        Err(AppError::UserForbidden(by.id))?
    }

    let res = query_as::<_, User>( "update users set name=$1 where id=$2 returning id, name, userhandle, balance, role, notify_level, updated_at, created_at")
        .bind(&name.name)
        .bind(name.id)
        .fetch_optional(tx.as_mut())
        .await
        .map_err(|e: sqlx::Error|{
            tracing::error!(target: "update-user-name", 
            error=?e, 
            executor=executor.id, 
            name=name.name, 
            by=name.by, 
            id=name.id, 
            "gotten error while updating user name");
        AppError::Internal
        })?.ok_or(AppError::UserNotFound(name.id))?;
    tx.commit().await.map_err(|e|{
        tracing::error!(target: "update-user-name", 
            error=?e, 
            executor=executor.id, 
            name=name.name, 
            by=name.by, 
            id=name.id, 
            "gotten error while committing transaction");
        AppError::Internal
    })?;
     tracing::debug!(target: "update-user-name", 
            executor=executor.id, 
            name=name.name, 
            by=name.by, 
            id=name.id, 
            "updated user name");
    Ok(Json(res))
}

#[derive(Clone, Deserialize)]
pub struct UpdateHandle {
    pub id: i64,
    pub by: i64,
    pub userhandle: String,
}

pub async fn update_user_handle(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(handle): Json<UpdateHandle>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.db.begin().await.map_err(|e|{
       tracing::error!(target: "update-user-handle", 
            error=?e, 
            executor=executor.id, 
            userhandle=handle.userhandle, 
            by=handle.by, 
            id=handle.id, 
            "gotten error while creating transaction");
        AppError::Internal
    })?;
    let executor_row = get_executor(tx.as_mut(), executor.id).await.map_err(|e| {
        tracing::error!(target: "update-user-handle", 
            error=?e, 
            executor=executor.id, 
            userhandle=handle.userhandle, 
            by=handle.by,
            id=handle.id, 
            "gotten error while getting executor");
        AppError::Internal
    })?.ok_or(AppError::ExecutorNotFound(executor.id))?;

    if executor_row.executor_type == ExecutorType::Userbot {
        Err(AppError::ExecutorForbidden(executor_row.id))?
    }
    if !VALID_USER_HANDLE.contains(&handle.userhandle.len()) || USER_HANDLE_REGEX.is_match(&handle.userhandle){
        return Err(AppError::InvalidUserHandle(handle.userhandle))
    }
    let by = users::get_user(tx.as_mut(), handle.by).await.map_err(|e|{
        tracing::error!(target: "update-user-handle", 
            error=?e, 
            executor=executor.id, 
            userhandle=handle.userhandle, 
            by=handle.by,
            id=handle.id,  
            "gotten error while getting by-user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(handle.by))?;

    if by.role < UserRole::User || by.role < UserRole::Moderator && handle.by != handle.id {
        Err(AppError::UserForbidden(by.id))?
    }
    users::get_user_for_update(&state.db, handle.id).await.map_err(|e|{
            tracing::error!(target: "update-user-handle", 
            error=?e, 
            executor=executor.id, 
            userhandle=handle.userhandle, 
            by=handle.by,
            id=handle.id,  
            "gotten error while getting user");
        AppError::Internal
    })?;

    let res = query_as::<_, User>( r#"
        update users set userhandle=$1 where id=$2 
        on conflict (userhandle) do nothing
        returning id, name, userhandle, balance, role, notify_level, updated_at, created_at"#
    )
        .bind(&handle.userhandle)
        .bind(handle.id)
        .fetch_optional(tx.as_mut())
        .await
        .map_err(|e: sqlx::Error|{
            tracing::error!(target: "update-user-handle", 
            error=?e, 
            executor=executor.id, 
            userhandle=handle.userhandle, 
            by=handle.by,
            id=handle.id,  
            "gotten error while updating user handle");
        AppError::Internal
        })?.ok_or(AppError::UserhandleTaken(handle.userhandle.clone()))?;

    tx.commit().await.map_err(|e|{
        tracing::error!(target: "update-user-handle", 
            error=?e, 
            executor=executor.id, 
            userhandle=handle.userhandle, 
            by=handle.by,
            id=handle.id,  
            "gotten error while committing transaction");
        AppError::Internal
    })?;
     tracing::debug!(target: "update-user-handle", 
            executor=executor.id, 
            userhandle=handle.userhandle, 
            by=handle.by,
            id=handle.id,  
            "updated user handle");
    Ok(Json(res))
}



#[derive(Clone, Deserialize)]
pub struct UpdateNotify {
    pub id: i64,
    pub level: NotifyLevel
}

pub async fn update_user_notify(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(notify): Json<UpdateNotify>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.db.begin().await.map_err(|e|{
       tracing::error!(target: "update-user-notify", 
            error=?e, 
            executor=executor.id, 
            id=notify.id,
            level=%notify.level,
            "gotten error while creating transaction");
        AppError::Internal
    })?;
    let executor_row = get_executor(tx.as_mut(), executor.id).await.map_err(|e| {
       tracing::error!(target: "update-user-notify", 
            error=?e, 
            executor=executor.id, 
            id=notify.id,
            level=%notify.level,
            "gotten error while getting executor");
        AppError::Internal
    })?.ok_or(AppError::ExecutorNotFound(executor.id))?;

    if executor_row.executor_type == ExecutorType::Userbot {
        Err(AppError::ExecutorForbidden(executor_row.id))?
    }

    let res = query_as::<_, User>( "update users set notify_level=$1 where id=$2 returning id, name, userhandle, balance, role, notify_level, updated_at, created_at")
        .bind(&notify.level)
        .bind(notify.id)
        .fetch_optional(tx.as_mut())
        .await
        .map_err(|e: sqlx::Error|{
            tracing::error!(target: "update-user-notify", 
                error=?e, 
                executor=executor.id, 
                id=notify.id,
                level=%notify.level,
                "gotten error while updating user notify level");
            AppError::Internal
        })?.ok_or(AppError::UserNotFound(notify.id))?;
    tx.commit().await.map_err(|e|{
       tracing::error!(target: "update-user-notify", 
            error=?e, 
            executor=executor.id, 
            id=notify.id,
            level=%notify.level,
            "gotten error while committing transaction");
        AppError::Internal
    })?;
     tracing::debug!(target: "update-user-notify", 
        executor=executor.id, 
        id=notify.id,
        level=%notify.level,
        "updated user notify level");
    Ok(Json(res))
}

#[derive(Clone, Deserialize)]
pub struct UpdateRole {
    pub id: i64,
    pub by: i64,
    pub role: UserRole,
}

pub async fn update_user_role(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(role): Json<UpdateRole>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = state.db.begin().await.map_err(|e|{
       tracing::error!(target: "update-user-role", 
            error=?e, 
            executor=executor.id, 
            by=role.by, 
            id=role.id, 
            role=%role.role,
            "gotten error while creating transaction");
        AppError::Internal
    })?;
    let executor_row = get_executor(tx.as_mut(), executor.id).await.map_err(|e| {
        tracing::error!(target: "update-user-role", 
            error=?e, 
            executor=executor.id, 
            by=role.by, 
            id=role.id, 
            role=%role.role,
            "gotten error while getting executor");
        AppError::Internal
    })?.ok_or(AppError::ExecutorNotFound(executor.id))?;

    if executor_row.executor_type == ExecutorType::Userbot {
        Err(AppError::ExecutorForbidden(executor_row.id))?
    }

    let by = users::get_user(tx.as_mut(), role.by).await.map_err(|e|{
        tracing::error!(target: "update-user-role", 
            error=?e, 
            executor=executor.id, 
            by=role.by, 
            id=role.id, 
            role=%role.role,
            "gotten error while getting by-user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(role.by))?;

    let user = users::get_user_for_update(tx.as_mut(), role.id).await.map_err(|e|{
        tracing::error!(target: "update-user-role", 
            error=?e, 
            executor=executor.id, 
            by=role.by, 
            id=role.id, 
            role=%role.role,
            "gotten error while getting user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(role.id))?;

    if executor_row.executor_type != ExecutorType::Admin &&
         ( role.role == UserRole::Pool || 
            role.role >= by.role || 
            by.role < UserRole::Moderator || 
            user.role == UserRole::Pool || 
            user.role >= by.role
        ) {
        Err(AppError::UserForbidden(by.id))?
    }

    let res = query_as::<_, User>( "update users set role=$1 where id=$2 returning id, name, userhandle, balance, role, notify_level, updated_at, created_at")
        .bind(&role.role)
        .bind(user.id)
        .fetch_one(tx.as_mut())
        .await
        .map_err(|e: sqlx::Error|{
            tracing::error!(target: "update-user-role", 
                error=?e, 
                executor=executor.id, 
                by=role.by, 
                id=role.id, 
                role=%role.role,
            "gotten error while updating user role");
        AppError::Internal
        })?;
    tx.commit().await.map_err(|e|{
        tracing::error!(target: "update-user-role", 
            error=?e, 
            executor=executor.id, 
            by=role.by, 
            id=role.id, 
            role=%role.role,
            "gotten error while committing transaction");
        AppError::Internal
    })?;
     tracing::debug!(target: "update-user-role", 
            executor=executor.id, 
            by=role.by, 
            id=role.id, 
            role=%role.role,
            "updated user role");
    Ok(Json(res))
}