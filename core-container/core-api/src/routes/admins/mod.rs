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