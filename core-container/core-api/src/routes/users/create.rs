use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::{query, query_as};

use crate::{error::AppError, models::users::User, routes::{Executor, users::VALID_USER_NAME}, state::AppState};

#[derive(Deserialize, Clone)]
pub struct CreateUser {
    pub name: String,
}
pub const CONTROL_POOL_ADDITION: i64 = 10_000;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<Executor>,
    Json(create): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    if !VALID_USER_NAME.contains(&create.name.len()) {
        return Err(AppError::InvalidUserName(create.name))
    }
    let mut tx = state.db.begin().await.map_err(|e|{
        tracing::error!(target: "create-user", error=?e, executor=executor.id, name=create.name, "gotten error while creating transaction");
        AppError::Internal
    })?;
    let res = query_as::<_, User>( "insert into users (name) values ($1) returning id, name, balance, role, notify_level, updated_at, created_at")
        .bind(&create.name)
        .fetch_one(tx.as_mut())
        .await
        .map_err(|e|{
            tracing::error!(target: "create-user", error=?e, executor=executor.id, name=create.name, "gotten error while creating user");
            AppError::Internal
        })?;
    query!("update global_config set control_pool = control_pool + $1", CONTROL_POOL_ADDITION)
        .execute(tx.as_mut())
        .await.map_err(|e|{
            tracing::error!(target: "create-user", error=?e, executor=executor.id, id=res.id, name=res.name, "gotten error while increasing control pool ");
            AppError::Internal
        })?;
    tx.commit().await.map_err(|e|{
        tracing::error!(target: "create-user", error=?e, executor=executor.id, id=res.id, name=res.name, "gotten error while committing transaction");
        AppError::Internal
    })?;

    tracing::debug!(target:"create-user", executor=executor.id, id=res.id, name=res.name, "created new user");
    Ok((StatusCode::CREATED, Json(res)))
}
