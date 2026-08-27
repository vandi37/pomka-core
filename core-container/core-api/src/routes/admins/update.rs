// The Pomka Ecosystem Core Source Code
// Copyright (C) 2026 Lev (Leo) Kondukov (aka DiceBarrel, Barrel, Vandi)
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::query_as;

use crate::{
    error::AppError,
    routes::admins::{Admin, AdminRes},
    state::AppState,
};

#[derive(Deserialize, Clone)]
pub struct UpdateAdmin {
    pub username: Option<String>,
    pub password: Option<String>,
}

pub async fn update_admin(
    State(state): State<Arc<AppState>>,
    Extension(admin): Extension<Admin>,
    Json(login): Json<UpdateAdmin>,
) -> Result<impl IntoResponse, AppError> {
    if login.username.is_none() && login.password.is_none() {
        Err(AppError::EmptyPatch)?
    }
    let password = login.password.and_then(|p| Some(state.password_hasher_service.hash_password(&p)
        .map_err(|e| {
            tracing::error!(target: "update-admin", error=?e, id=admin.id, "gotten error while hashing admin password");
            AppError::Internal
    }))).transpose()?;
    let res = query_as!(AdminRes, r#"
update admins 
set
    username=coalesce($1,username), 
    password=coalesce($2,password) 
where id=$3 returning id, username, creator, updated_at, created_at"#, login.username, password, admin.id)
        .fetch_one(&state.db)
        .await
        .map_err(|e|match (e, login.username) {
                (sqlx::Error::Database(db_err), Some(u)) if db_err.is_unique_violation() => AppError::AdminUsernameTaken(u),
                (e, _) => {
                    tracing::error!(target: "update-admin", error=?e, id=admin.id, "gotten error while updating admin");
                    AppError::Internal
                }
    })?;
    tracing::info!(target: "update-admin", username=res.username, id=res.id, "updated user");
    Ok((StatusCode::OK, Json(res)))
}
