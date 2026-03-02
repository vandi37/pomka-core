use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state, routing::{get, post}};

use crate::state::AppState;
mod middleware;
mod login;

pub fn admins_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
    .merge( Router::new()
        .layer(from_fn_with_state(state.clone(), middleware::admin_access))
    )
    .merge(Router::new()
        .route("/refresh", get(login::refresh))
        .layer(from_fn_with_state(state.clone(), middleware::admin_refresh))
)   
    .route("/login", post(login::login))
}

#[derive(Clone)]
pub struct Admin(pub i64);