use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::query_as;

use crate::{error::AppError, routes::{Params, admins::{Admin, AdminRes}}, state::AppState};

pub async fn get_admin(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let res = query_as!(AdminRes, "select id, username, creator, updated_at, created_at from admins where id = $1", id)
        .fetch_optional(&state.db)
        .await
        .or_else(|e| {
            tracing::error!(target:"get-admin", error=?e, id, by=admin.id, "gotten error while getting admin");
            Err(AppError::Internal)
        })?
        .ok_or(AppError::AminNotFound(id))?;
    tracing::debug!(target:"get-admin", id, by=admin.id, "gotten admin");

    Ok((StatusCode::OK, Json(res)))
}



pub async fn get_admins(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let admins = query_as!(AdminRes, "select id, username, creator, updated_at, created_at from admins order by id asc limit $1 offset $2", params.limit, params.offset)
        .fetch_all(&state.db)
        .await
        .or_else(|e| {
              tracing::error!(target:"get-admins", error=?e, by=admin.id, limit=params.limit, offset=params.offset, "gotten error while getting admins");
            Err(AppError::Internal)
        })?;
    tracing::debug!(target:"get-admin", by=admin.id, limit=params.limit, offset=params.offset, len=admins.len(), "gotten admins");
    Ok((StatusCode::OK, Json(admins)))
}
