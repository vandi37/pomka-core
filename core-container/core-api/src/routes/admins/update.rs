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

pub async fn update_admin(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(login): Json<InputAdmin>,
) -> Result<impl IntoResponse, AppError> {
    let password = state.password_hasher_service.hash_password(&login.password)
        .or_else(|e| {
            tracing::error!(target: "update-admin", error=?e, username=login.username, id=admin.id, "gotten error while hashing admin password");
            Err(AppError::Internal)
    })?;
    let res = query_as!(AdminRes, "update admins set username=$1, password=$2 where id=$3 returning id, username, creator, updated_at, created_at", login.username, password, admin.id)
        .fetch_one(&state.db)
        .await
        .or_else(|e|match e {
                sqlx::Error::Database(db_err) if db_err.is_unique_violation() => Err(AppError::AdminUsernameTaken(login.username.clone())),
                e => {
                    tracing::error!(target: "update-admin", error=?e, username=login.username, id=admin.id, "gotten error while updating admin");
                    Err(AppError::Internal)
                }
    })?;
    tracing::info!(target: "update-admin", username=res.username, id=res.id, "updated user");
    Ok((StatusCode::OK, Json(res)))
}
