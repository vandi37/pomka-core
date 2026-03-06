use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;
mod create;
mod delete;
mod get;
mod login;
mod middleware;
mod update;

pub fn admins_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/refresh",
            get(login::refresh)
                .route_layer(from_fn_with_state(state.clone(), middleware::admin_refresh)),
        )
        .route("/login", post(login::login))
        .merge(
            Router::new()
                .route("/{id}", get(get::get_admin).delete(delete::delete_admin))
                .route(
                    "/",
                    post(create::create)
                        .put(update::update_admin)
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
