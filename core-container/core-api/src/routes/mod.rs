mod middleware;
mod admins;

use std::sync::Arc;

use axum::{Json, Router, middleware::from_fn, response::IntoResponse, routing::get};
use serde::{Serialize};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

use crate::{error::AppError, routes::middleware::logging, state::AppState};


#[derive(Clone, Serialize)]
pub struct TokenResponse {
    pub id: i64,
    pub token: String
}

pub fn router(state: Arc<AppState>) -> Router {
    let cors: CorsLayer = CorsLayer::new().allow_origin(Any).allow_methods(Any);
    Router::new()
        .route("/ping", get(|| async { Json(json!("pong")) }))
        .nest("/admin/", admins::admins_router(state.clone()))
        .layer(cors)
        .layer(from_fn(logging))
        .with_state(state)
}
