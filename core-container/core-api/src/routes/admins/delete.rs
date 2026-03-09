use std::sync::Arc;

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::query;

use crate::{error::AppError, routes::admins::Admin, state::AppState};

pub async fn delete_admin(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let res = query!("delete from admins where id = $1", id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(target:"delete-admin", error=?e, id, by=admin.id, "gotten error while deleting admin");
            AppError::Internal
        })?;
    if res.rows_affected() == 0 {
        Err(AppError::AdminNotFound(id))?
    }
    tracing::info!(target:"delete-admin", id, by=admin.id, "deleted admin");
    Ok(StatusCode::NO_CONTENT)
}
