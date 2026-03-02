use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, extract::{Extension, State}};
use chrono::Duration;
use serde::Deserialize;
use sqlx::{query};

use crate::routes::TokenResponse;
use crate::routes::admins::Admin;
use crate::tokens::create_jwt;
use crate::{error::AppError, state::AppState};

#[derive(Deserialize, Clone)]
pub struct LoginAdmin {
    pub username: String,
    pub password: String,
}
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(login): Json<LoginAdmin>,
) -> Result<impl IntoResponse, AppError> {
    let admin = query!(
        "select id, password from admins where username = $1",
        login.username
    )
    .fetch_optional(&state.db)
    .await
    .or_else(|e| {
        tracing::error!("got error in login {e}");
        Err(AppError::Internal)
    })?
    .ok_or_else(|| AppError::InvalidCredentials)?;
    state
        .password_hasher_service
        .verify_password(&admin.password, &login.password)
        .or_else(|e| {
            tracing::error!("got error in login {e}");
            Err(AppError::Internal)
        })?
        .then_some(())
        .ok_or(AppError::Internal)?;

    let token =
        create_jwt(admin.id, (), state.tokens_state.admins.refresh.as_bytes(), Duration::days(7)).or_else(|e| {
            tracing::error!("got error in login {e}");
            Err(AppError::Internal)
        })?;

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
    Extension(admin): Extension<Admin>
)-> Result<impl IntoResponse, AppError> {
    let token =
        create_jwt(admin.0, (), state.tokens_state.admins.access.as_bytes(), Duration::hours(1)).or_else(|e| {
            tracing::error!("got error in login {e}");
            Err(AppError::Internal)
        })?;

    Ok((
        StatusCode::OK,
        Json(TokenResponse {
            id: admin.0,
            token,
        }),
    ))
}