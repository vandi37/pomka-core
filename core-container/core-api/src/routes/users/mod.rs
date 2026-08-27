// The Pomka Ecosystem Core Source Code
// Copyright (C) 2026 Lev (Leo) Kondukov (aka DiceBarrel, Barrel, Vandi)
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ops::RangeInclusive;
use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};
use lazy_regex::{Lazy, Regex, lazy_regex};
pub const VALID_USER_NAME: RangeInclusive<usize> = 1..=64;
pub const VALID_USER_HANDLE: RangeInclusive<usize> = 3..=32;
pub static USER_HANDLE_REGEX: Lazy<Regex> = lazy_regex!("^[a-z][a-z0-9-]*[a-z0-9]$"i);

use crate::{
    routes::{
        admins::middleware::admin_access, middleware::{access, adapter, map_executor}, users::{
            create::create_user, delete::remove_user_handle, get::{get_user_dispatcher, get_users, users_leaderboard}, update::{update_user_handle, update_user_name, update_user_notify, update_user_role},
        },
    }, state::AppState,
};
mod create;
mod get;
mod update;
mod delete;
pub fn users_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .merge(
            Router::new()
                .route(
                    "/",
                    post(create_user).route_layer(from_fn_with_state(state.clone(), adapter)),
                )
                .route("/name", patch(update_user_name))
                .route("/userhandle", patch(update_user_handle).delete(remove_user_handle))
                .route("/notify", patch(update_user_notify))
                .route("/role", patch(update_user_role))
                .route("/leaderboard", get(users_leaderboard))
                .route("/{identifier}", get(get_user_dispatcher))
                .layer(from_fn_with_state(state.clone(), map_executor))
                .layer(from_fn_with_state(state.clone(), access)),
        )
        .route(
            "/",
            get(get_users).route_layer(from_fn_with_state(state.clone(), admin_access)),
        )
}
