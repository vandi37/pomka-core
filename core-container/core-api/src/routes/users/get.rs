use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Path, Query, State},
    response::IntoResponse,
};
use serde::Serialize;
use sqlx::{Postgres, query_as};

use crate::{
    error::AppError, models::users::User, routes::{
        Executor, MAX_LIMIT, Params, admins::Admin, users::{USER_HANDLE_REGEX, VALID_USER_HANDLE}
    }, services::users, state::AppState
};

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Path(id): Path<i64>,
) -> Result<Json<User>, AppError> {
    let user = users::get_user(&state.db, id).await.map_err(|e| {
        tracing::error!(target: "get-user", error=?e, executor=executor.id, id, "gotten error while getting user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(id))?;
    tracing::debug!(target: "get-user", executor=executor.id, id, "gotten user");
    Ok(Json(user))
}

pub async fn get_user_by_handle(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Path(userhandle): Path<String>,
) -> Result<Json<User>, AppError> {
     if !VALID_USER_HANDLE.contains(&userhandle.len()) || USER_HANDLE_REGEX.is_match(&userhandle){
        return Err(AppError::InvalidUserHandle(userhandle))
    }
    let user = users::get_user_by_handle(&state.db, &userhandle).await.map_err(|e| {
        tracing::error!(target: "get-user-by-handle", error=?e, executor=executor.id, userhandle, "gotten error while getting user by handle");
        AppError::Internal
    })?.ok_or(AppError::UserNotFoundByHandle(userhandle.clone()))?;
    tracing::debug!(target: "get-user-by-handle", executor=executor.id, userhandle, "gotten user by handle");
    Ok(Json(user))
}


pub async fn get_user_dispatcher(
    state: State<Arc<AppState>>,
    extention: Extension<Executor>,
    Path(identifier): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(id) = identifier.parse::<i64>() {
        get_user(state, extention, Path(id)).await
    } else {
        get_user_by_handle(state, extention, Path(identifier)).await
    }
}


#[derive(Clone, Serialize)]
pub struct LeaderBoardUser {
    pub id: i64,
    pub name: Option<String>,
    pub balance: i64,
}


pub async fn users_leaderboard(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Query(mut params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    params.limit = params.limit.min(MAX_LIMIT).max(1);
    params.offset = params.offset.max(1);
    let users = query_as!(LeaderBoardUser, "select id, name, balance from users order by balance desc limit $1 offset $2", params.limit, params.offset)
        .fetch_all(&state.db)
        .await.map_err(|e| {
            tracing::error!(target:"users-leaderboard", error=?e, executor=executor.id, limit=params.limit, offset=params.offset, "gotten error while getting users leaderboard");
            AppError::Internal
        })?;
    tracing::debug!(target:"users-leaderboard", executor=executor.id, limit=params.limit, offset=params.offset, len=users.len(), "gotten users leaderboard");
    Ok(Json(users))
}

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Query(mut params): Query<Params>
) -> Result<impl IntoResponse, AppError> {
    params.limit = params.limit.min(MAX_LIMIT).max(1);
    params.offset = params.offset.max(1);
    let users = query_as::<Postgres, User>(
        "select id, name, userhandle, balance, role, notify_level, updated_at, created_at from users order by id asc limit $1 offset $2")
        .bind(params.limit)
        .bind(params.offset)
        .fetch_all(&state.db)
        .await.map_err(|e| {
            tracing::error!(target:"get-users", error=?e, by=admin.id, limit=params.limit, offset=params.offset, "gotten error while getting users");
            AppError::Internal
        })?;
    tracing::debug!(target:"get-users", by=admin.id, limit=params.limit, offset=params.offset, len=users.len(), "gotten users");
    Ok(Json(users))
}