use std::ops::RangeInclusive;
use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};
pub const VALID_USER_NAME: RangeInclusive<usize> = 1..=64;
use crate::{
    routes::{
        admins::middleware::admin_access,
        middleware::{access, adapter, map_executor},
        users::{
            create::create_user,
            get::{get_user, get_users, users_leaderboard},
            update::{update_user_name, update_user_notify, update_user_role},
        },
    },
    state::AppState,
};
mod create;
mod get;
mod update;

pub fn users_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .merge(
            Router::new()
                .route(
                    "/",
                    post(create_user).route_layer(from_fn_with_state(state.clone(), adapter)),
                )
                .route("/name", patch(update_user_name))
                .route("/notify", patch(update_user_notify))
                .route("/role", patch(update_user_role))
                .route("/leaderboard", get(users_leaderboard))
                .route("/{id}", get(get_user))
                .layer(from_fn_with_state(state.clone(), map_executor))
                .layer(from_fn_with_state(state.clone(), access)),
        )
        .route(
            "/",
            get(get_users).route_layer(from_fn_with_state(state.clone(), admin_access)),
        )
}
