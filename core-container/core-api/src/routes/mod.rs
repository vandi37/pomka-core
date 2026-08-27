pub mod middleware;
pub mod admins;
mod bots;
mod users;

use std::sync::Arc;

use axum::{Json, Router, middleware::from_fn, routing::get};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

use crate::{routes::middleware::logging, state::AppState};


#[derive(Clone, Serialize)]
pub struct TokenResponse {
    pub id: i64,
    pub token: String,
    pub expr: i64
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
        .nest("/users/", users::users_router(state.clone()))
        .layer(cors)
        .layer(from_fn(logging))
        .with_state(state)
}

const MAX_LIMIT: i64 = 100;