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
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::state::AppState;
mod create;
mod delete;
mod get;
mod login;
pub mod middleware;
mod update;

pub fn admins_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/refresh",
            get(login::refresh_admin)
                .route_layer(from_fn_with_state(state.clone(), middleware::admin_refresh)),
        )
        .route("/login", post(login::login_admin))
        .merge(
            Router::new()
                .route("/{id}", get(get::get_admin).delete(delete::delete_admin))
                .route(
                    "/",
                    post(create::create_admin)
                        .patch(update::update_admin)
                        .get(get::get_admins),
                )
                .layer(from_fn_with_state(state.clone(), middleware::admin_access)),
        )
}

#[derive(Clone, Serialize)]
pub struct Admin {
    pub id: i64,
}

#[derive(Deserialize, Clone)]
pub struct InputAdmin {
    pub username: String,
    pub password: String,
}


#[derive(Serialize)]
pub struct AdminRes {
    pub id: i64,
    pub username: String,
    pub creator: Option<i64>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}