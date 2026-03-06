use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    Json,
    extract::{Extension, State},
};
use chrono::Duration;
use sqlx::query;

use crate::routes::TokenResponse;
use crate::routes::bots::{Bot, InputBot};
use crate::tokens::create_jwt;
use crate::{error::AppError, state::AppState};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(login): Json<InputBot>,
) -> Result<impl IntoResponse, AppError> {
    let bot = query!(
        "select id, password from bots where username = $1",
        login.username
    )
    .fetch_optional(&state.db)
    .await
    .or_else(|e| {
        tracing::error!(target: "bot-login", error=?e, username=login.username, "gotten error while getting bot data");
        Err(AppError::Internal)
    })?
    .ok_or_else(|| AppError::InvalidCredentials)?;
    state
        .password_hasher_service
        .verify_password(&bot.password, &login.password)
        .or_else(|e| {
            tracing::error!(target: "bot-login", error=?e, username=login.username, id=bot.id, "gotten error while verifying bot password");
            Err(AppError::Internal)
        })?
        .then_some(())
        .ok_or(AppError::Internal)?;
    let token =
        create_jwt(bot.id, (), state.tokens_state.bots.refresh.as_bytes(), Duration::days(7)).or_else(|e| {
            tracing::error!(target: "bot-login", error=?e, username=login.username, id=bot.id, "gotten error while creating bot jwt");
            Err(AppError::Internal)
        })?;
    tracing::info!(target:"bot-login", id=bot.id, username=login.username, "bot logged in");
    Ok((
        StatusCode::OK,
        Json(TokenResponse {
            id: bot.id,
            token,
        }),
    ))
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Extension(bot): Extension<Bot>,
) -> Result<impl IntoResponse, AppError> {
    let token =
        create_jwt(bot.id, (), state.tokens_state.bots.access.as_bytes(), Duration::hours(1)).or_else(|e| {
            tracing::error!(target: "bot-refresh", error=?e, id=bot.id, "gotten error while creating access token for bot");
            Err(AppError::Internal)
        })?;
    tracing::info!(target: "bot-refresh", id=bot.id, "bot gotten access token");
    Ok((
        StatusCode::OK,
        Json(TokenResponse {
            id: bot.id,
            token,
        }),
    ))
}
