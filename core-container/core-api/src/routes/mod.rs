mod middleware;
mod admins;
mod bots;

use std::sync::Arc;

use axum::{Json, Router, middleware::from_fn, routing::get};
use serde::{Serialize};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

use crate::{routes::middleware::logging, state::AppState};


#[derive(Clone, Serialize)]
pub struct TokenResponse {
    pub id: i64,
    pub token: String
}

pub fn router(state: Arc<AppState>) -> Router {
    let cors: CorsLayer = CorsLayer::new().allow_origin(Any).allow_methods(Any);
    Router::new()
        .route("/ping", get(|| async { Json(json!("pong")) }))
        .nest("/admins/", admins::admins_router(state.clone()))
        .nest("/bots/", bots::bots_router(state.clone()))
        .layer(cors)
        .layer(from_fn(logging))
        .with_state(state)
}
