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
        Executor, MAX_LIMIT, Params, admins::Admin
    }, services::users, state::AppState
};

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let user = users::get_user(&state.db, id).await.map_err(|e| {
        tracing::error!(target: "get-user", error=?e, executor=executor.id, id, "gotten error while getting user");
        AppError::Internal
    })?.ok_or(AppError::UserNotFound(id))?;
    tracing::debug!(target: "get-user", executor=executor.id, id, "gotten user");
    Ok(Json(user))
}

#[derive(Clone, Serialize)]
pub struct LeaderBoardUser {
    pub id: i64,
    pub name: String,
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
        "select id, name, balance, role, notify_level, updated_at, created_at from users order by id asc limit $1 offset $2")
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