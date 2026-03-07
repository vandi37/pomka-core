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
    pub allow_produce_stocks: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
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
                .route("/", post(create::create).get(get::get_bots))
                .route(
                    "/{id}",
                    patch(update::update)
                        .get(get::get_bot)
                        .delete(delete::delete_bot),
                )
                .layer(from_fn_with_state(state.clone(), admin_access)),
        )
}
