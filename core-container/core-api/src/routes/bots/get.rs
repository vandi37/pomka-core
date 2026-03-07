use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::query_as;

use crate::{error::AppError, routes::{Params, admins::Admin, bots::BotRes}, state::AppState};

pub async fn get_bot(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let bot = query_as!(BotRes, "select id, username, creator, allow_produce_stocks, updated_at, created_at from bots where id = $1", id)
        .fetch_optional(&state.db)
        .await
        .or_else(|e| {
            tracing::error!(target:"get-bot", error=?e, id, by=admin.id, "gotten error while getting bot");
            Err(AppError::Internal)
        })?
        .ok_or(AppError::BotNotFound(id))?;
    tracing::debug!(target:"get-bot", id, by=admin.id, "gotten bot");
    Ok((StatusCode::OK, Json(bot)))
}

pub async fn get_bots(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, AppError> {
    let bots = query_as!(BotRes, "select id, username, creator, allow_produce_stocks, updated_at, created_at from bots order by id asc limit $1 offset $2", params.limit, params.offset)
        .fetch_all(&state.db)
        .await
        .or_else(|e| {
              tracing::error!(target:"get-bots", error=?e, by=admin.id, limit=params.limit, offset=params.offset, "gotten error while getting bots");
            Err(AppError::Internal)
        })?;
    tracing::debug!(target:"get-bots", by=admin.id, limit=params.limit, offset=params.offset, len=bots.len(), "gotten bots");
    Ok((StatusCode::OK, Json(bots)))
}
