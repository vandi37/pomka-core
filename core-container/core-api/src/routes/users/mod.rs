use std::ops::RangeInclusive;
use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state, routing::post};
pub const VALID_USER_NAME: RangeInclusive<usize> = 1..=64;
use crate::{
    routes::{
        middleware::{access, adapter, map_executor},
        users::create::create_user,
    },
    state::AppState,
};
mod create;

pub fn users_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().merge(
        Router::new()
            .route("/", post(create_user)
                .route_layer(from_fn_with_state(state.clone(), adapter)))
            .layer(from_fn_with_state(state.clone(), map_executor))
            .layer(from_fn_with_state(state.clone(), access)),
    )
}
