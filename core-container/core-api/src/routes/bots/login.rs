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

pub async fn login_bot(
    State(state): State<Arc<AppState>>,
    Json(login): Json<InputBot>,
) -> Result<impl IntoResponse, AppError> {
    let bot = query!(
        "select id, password from bots where username = $1",
        login.username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!(target: "bot-login", error=?e, username=login.username, "gotten error while getting bot data");
        AppError::Internal
    })?
    .map(|bot|  state
        .password_hasher_service
        .verify_password(&bot.password, &login.password)
        .map_err(|e| {
            tracing::error!(target: "bot-login", error=?e, username=login.username, id=bot.id, "gotten error while verifying bot password");
            AppError::Internal
        })?
        .then_some(bot)
        .ok_or(AppError::Internal))
    .ok_or( AppError::InvalidCredentials)??;
    let (token, expr) =
        create_jwt(bot.id, (), state.tokens_state.bots.refresh.as_bytes(), Duration::days(7)).map_err(|e| {
            tracing::error!(target: "bot-login", error=?e, username=login.username, id=bot.id, "gotten error while creating bot jwt");
            AppError::Internal
        })?;
    tracing::info!(target:"bot-login", id=bot.id, username=login.username, "bot logged in");
    Ok((
        StatusCode::OK,
        Json(TokenResponse {
            id: bot.id,
            token, expr
        }),
    ))
}

pub async fn refresh_bot(
    State(state): State<Arc<AppState>>,
    Extension(bot): Extension<Bot>,
) -> Result<impl IntoResponse, AppError> {
    let (token, expr) =
        create_jwt(bot.id, (), state.tokens_state.bots.access.as_bytes(), Duration::hours(1)).map_err(|e| {
            tracing::error!(target: "bot-refresh", error=?e, id=bot.id, "gotten error while creating access token for bot");
            AppError::Internal
        })?;
    tracing::info!(target: "bot-refresh", id=bot.id, "bot gotten access token");
    Ok((
        StatusCode::OK,
        Json(TokenResponse {
            id: bot.id,
            token, expr
        }),
    ))
}
