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
mod update;
pub mod get;
use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::get,
};

use crate::routes::{global_config::{get::get_global_config, update::update_global_config}, middleware::{access, map_executor}};
use crate::{routes::admins::middleware::admin_access, utils::state::AppState};

pub fn global_config_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_global_config)
            .route_layer(from_fn_with_state(state.clone(), map_executor))
            .route_layer(from_fn_with_state(state.clone(), access))
            .patch(update_global_config).route_layer(from_fn_with_state(state, admin_access)))
}
