use std::sync::Arc;

use axum::{Json, extract::{Extension, State}, response::IntoResponse};
use serde::Deserialize;
use sqlx::query_as;

use crate::{error::AppError, models::{executors::ExecutorType, users::{NotifyLevel, User, UserRole}}, routes::{Executor, users::VALID_USER_NAME}, services::{executors::get_executor, users}, state::AppState};

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

    let res = query_as::<_, User>( "update users set name=$1 where id=$2 returning id, name, balance, role, notify_level, updated_at, created_at")
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

    let res = query_as::<_, User>( "update users set notify_level=$1 where id=$2 returning id, name, balance, role, notify_level, updated_at, created_at")
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

    let user = users::get_user(tx.as_mut(), role.id).await.map_err(|e|{
        tracing::error!(target: "update-user-role", 
            error=?e, 
            executor=executor.id, 
            by=role.by, 
            id=role.id, 
            role=%role.role,
            "gotten error while getting user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(role.id))?;

    if role.role == UserRole::Pool || role.role >= by.role || by.role < UserRole::Moderator || user.role == UserRole::Pool || user.role >= by.role {
        Err(AppError::UserForbidden(by.id))?
    }

    let res = query_as::<_, User>( "update users set role=$1 where id=$2 returning id, name, balance, role, notify_level, updated_at, created_at")
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