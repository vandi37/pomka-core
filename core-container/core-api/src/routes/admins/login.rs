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
use crate::routes::admins::{Admin, InputAdmin};
use crate::tokens::create_jwt;
use crate::{error::AppError, state::AppState};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(login): Json<InputAdmin>,
) -> Result<impl IntoResponse, AppError> {
    let admin = query!(
        "select id, password from admins where username = $1",
        login.username
    )
    .fetch_optional(&state.db)
    .await
    .or_else(|e| {
        tracing::error!(target: "admin-login", error=?e, username=login.username, "gotten error while getting admin data");
        Err(AppError::Internal)
    })?
    .ok_or_else(|| AppError::InvalidCredentials)?;
    state
        .password_hasher_service
        .verify_password(&admin.password, &login.password)
        .or_else(|e| {
            tracing::error!(target: "admin-login", error=?e, username=login.username, id=admin.id, "gotten error while verifying admin password");
            Err(AppError::Internal)
        })?
        .then_some(())
        .ok_or(AppError::Internal)?;
    let token =
        create_jwt(admin.id, (), state.tokens_state.admins.refresh.as_bytes(), Duration::days(7)).or_else(|e| {
            tracing::error!(target: "admin-login", error=?e, username=login.username, id=admin.id, "gotten error while creating admin jwt");
            Err(AppError::Internal)
        })?;
    tracing::info!(target:"admin-login", id=admin.id, username=login.username, "admin logged in");
    Ok((
        StatusCode::OK,
        Json(TokenResponse {
            id: admin.id,
            token,
        }),
    ))
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
) -> Result<impl IntoResponse, AppError> {
    let token =
        create_jwt(admin.id, (), state.tokens_state.admins.access.as_bytes(), Duration::hours(1)).or_else(|e| {
            tracing::error!(target: "admin-refresh", error=?e, id=admin.id, "gotten error while creating access token for admin");
            Err(AppError::Internal)
        })?;
    tracing::info!(target: "admin-refresh", id=admin.id, "admin gotten access token");
    Ok((
        StatusCode::OK,
        Json(TokenResponse {
            id: admin.id,
            token,
        }),
    ))
}
