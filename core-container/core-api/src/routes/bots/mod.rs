use serde::{Deserialize, Serialize};

mod middleware;
mod login;

use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};

use crate::state::AppState;
#[derive(Clone, Serialize)]
pub struct Bot {
    pub id: i64,
}

#[derive(Deserialize, Clone)]
pub struct InputBot {
    pub username: String,
    pub password: String,
}

pub fn bots_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/refresh",
            get(login::refresh)
                .route_layer(from_fn_with_state(state.clone(), middleware::bot_refresh)),
        )
        .route("/login", post(login::login))
        .merge(
            Router::new()
                .layer(from_fn_with_state(state.clone(), middleware::bot_access)),
        )
}
