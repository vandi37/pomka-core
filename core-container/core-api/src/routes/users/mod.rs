use std::ops::RangeInclusive;
use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state, routing::{patch, post}};
pub const VALID_USER_NAME: RangeInclusive<usize> = 1..=64;
use crate::{
    routes::{
        middleware::{access, adapter, map_executor},
        users::{create::create_user, update::{update_user_name, update_user_notify, update_user_role}},
    },
    state::AppState,
};
mod create;
mod update;

pub fn users_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().merge(
        Router::new()
            .route("/", post(create_user)
                .route_layer(from_fn_with_state(state.clone(), adapter)))
            .route("/name", patch(update_user_name))
            .route("/notify", patch(update_user_notify))
            .route("/role", patch(update_user_role))
            .layer(from_fn_with_state(state.clone(), map_executor))
            .layer(from_fn_with_state(state.clone(), access)),
    )
}
