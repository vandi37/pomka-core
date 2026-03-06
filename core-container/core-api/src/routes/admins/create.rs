use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::query_as;

use crate::{
    error::AppError,
    routes::admins::{Admin, InputAdmin, get::AdminRes},
    state::AppState,
};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(login): Json<InputAdmin>,
) -> Result<impl IntoResponse, AppError> {
    let password = state.password_hasher_service.hash_password(&login.password)
        .or_else(|e| {
            tracing::error!(target: "create-admin", error=?e, username=login.username, creator=admin.id, "gotten error while hashing admin password");
            Err(AppError::Internal)
    })?;
    let res = query_as!(AdminRes ,"insert into admins (username, password, creator) values ($1, $2, $3) returning id, username, creator, updated_at, created_at",
        login.username, password, admin.id)
        .fetch_one(&state.db)
        .await
        .or_else(|e| match e {
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => Err(AppError::AdminUsernameTaken(login.username.clone())),
                e => {
                    tracing::error!(target: "create-admin", error=?e, username=login.username, creator=admin.id, "gotten error while creating admin");
                    Err(AppError::Internal)
                }
            }
        )?;
    tracing::info!(target:"create-admin", username=res.username, id=res.id, creator=res.creator, "created new admin");
    Ok((StatusCode::CREATED, Json(res)))
}
