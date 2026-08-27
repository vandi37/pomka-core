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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod create;
mod delete;
mod get;
mod login;
pub mod middleware;
mod update;

use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};

use crate::{routes::admins::middleware::admin_access, state::AppState};
#[derive(Clone, Serialize)]
pub struct Bot {
    pub id: i64,
}

#[derive(Deserialize, Clone)]
pub struct InputBot {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BotRes {
    pub id: i64,
    pub username: String,
    pub creator: Option<i64>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub fn bots_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/refresh",
            get(login::refresh_bot)
                .route_layer(from_fn_with_state(state.clone(), middleware::bot_refresh)),
        )
        .route("/login", post(login::login_bot))
        .merge(
            Router::new()
                .route("/", post(create::create_bot).get(get::get_bots))
                .route(
                    "/{id}",
                    patch(update::update_bot)
                        .get(get::get_bot)
                        .delete(delete::delete_bot),
                )
                .layer(from_fn_with_state(state.clone(), admin_access)),
        )
}
