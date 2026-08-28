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

pub mod admins;
mod bots;
mod claim_daily_reward;
mod global_config;
pub mod middleware;
mod users;

use std::sync::Arc;

use axum::{
    Json, Router,
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    routes::{
        global_config::get::get_fees,
        middleware::{access, logging, map_executor},
    },
    state::AppState,
};

#[derive(Clone, Serialize)]
pub struct TokenResponse {
    pub id: i64,
    pub token: String,
    pub expr: i64,
}

#[derive(Deserialize)]
pub struct Params {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Clone, Serialize)]
pub struct Executor {
    pub id: i64,
}

pub fn router(state: Arc<AppState>) -> Router {
    let cors: CorsLayer = CorsLayer::new().allow_origin(Any).allow_methods(Any);
    Router::new()
        .route("/ping", get(|| async { Json(json!("pong")) }))
        .nest("/admins/", admins::admins_router(state.clone()))
        .nest("/bots/", bots::bots_router(state.clone()))
        .nest(
            "/users/",
            users::users_router(state.clone()).route(
                "/daily-reward",
                post(claim_daily_reward::claim_daily_reward)
                    .route_layer(from_fn_with_state(state.clone(), map_executor))
                    .route_layer(from_fn_with_state(state.clone(), access)),
            ),
        )
        .nest(
            "/global-config/",
            global_config::global_config_router(state.clone()),
        )
        .route(
            "/fees",
            get(get_fees)
                .route_layer(from_fn_with_state(state.clone(), map_executor))
                .route_layer(from_fn_with_state(state.clone(), access)),
        )
        .layer(cors)
        .layer(from_fn(logging))
        .with_state(state)
}

const MAX_LIMIT: i64 = 100;
