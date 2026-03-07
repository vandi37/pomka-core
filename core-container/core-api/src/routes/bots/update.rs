use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::query_as;

use crate::{
    error::AppError,
    routes::{admins::Admin, bots::BotRes},
    state::AppState,
};
#[derive(Deserialize, Clone)]
pub struct UpdateBot {
    pub username: Option<String>,
    pub password: Option<String>,
    pub allow_produce_stocks: Option<bool>,
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(id): Path<i64>,
    Json(bot): Json<UpdateBot>,
) -> Result<impl IntoResponse, AppError> {
    if bot.username.is_none() && bot.password.is_none() && bot.allow_produce_stocks.is_none() {
        Err(AppError::EmptyPatch)?
    }
    let password = bot.password.and_then(|p| Some(state.password_hasher_service.hash_password(&p)
        .or_else(|e| {
            tracing::error!(target: "update-bot", error=?e, id, by=admin.id, "gotten error while hashing bot password");
            Err(AppError::Internal)
    }))).transpose()?;
    let res = query_as!(BotRes ,r#"
update bots
set
    username = coalesce($1, username), 
    password = coalesce($2, password), 
    allow_produce_stocks = coalesce($3, allow_produce_stocks)
where id = $4
returning id, username, creator, allow_produce_stocks, updated_at, created_at"#,
        bot.username, password, bot.allow_produce_stocks, id)
        .fetch_optional(&state.db)
        .await
        .or_else(|e| match (e, bot.username) {
               (sqlx::Error::Database(db_err), Some(u)) if db_err.is_unique_violation() => Err(AppError::BotUsernameTaken(u)),
                (e, _) => {
                    tracing::error!(target: "update-bot", error=?e, creator=admin.id, "gotten error while updating bot");
                    Err(AppError::Internal)
                }
            }
        )?
        .ok_or(AppError::BotNotFound(id))?;
    tracing::info!(target:"update-bot", username=res.username, id=res.id, creator=res.creator, by=admin.id, "updated bot");
    Ok((StatusCode::OK, Json(res)))
}
