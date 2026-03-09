use std::sync::Arc;

use axum::{
    Json, extract::{Extension, State}, http::StatusCode, response::IntoResponse
};
use serde::Deserialize;
use sqlx::query_as;

use crate::{
    error::AppError,
    routes::{admins::Admin, bots::BotRes},
    state::AppState,
};
#[derive(Deserialize, Clone)]
pub struct CreateBot {
    pub username: String,
    pub password: String,
    pub allow_produce_stocks: bool,
}

pub async fn create_bot(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(login): Json<CreateBot>,
) -> Result<impl IntoResponse, AppError> {
    let password = state.password_hasher_service.hash_password(&login.password)
        .map_err(|e| {
            tracing::error!(target: "create-bot", error=?e, username=login.username, creator=admin.id, "gotten error while hashing bot password");
            AppError::Internal
    })?;
    let res = query_as!(BotRes ,"insert into bots (username, password, creator, allow_produce_stocks) values ($1, $2, $3, $4) returning id, username, creator, allow_produce_stocks, updated_at, created_at",
        login.username, password, admin.id, login.allow_produce_stocks)
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => AppError::BotUsernameTaken(login.username.clone()),
                e => {
                    tracing::error!(target: "create-bot", error=?e, username=login.username, creator=admin.id, "gotten error while creating bot");
                    AppError::Internal
                }
            }
        )?;
    tracing::info!(target:"create-bot", username=res.username, id=res.id, creator=res.creator, "created new bot");
    Ok((StatusCode::CREATED, Json(res)))
}
