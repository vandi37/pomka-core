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
mod patch;
pub mod get;
use std::sync::Arc;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::patch,
};

use crate::{routes::{admins::middleware::admin_access, global_config::patch::{patch_admin_fee, patch_bot_fee, patch_userbot_fee, patch_userbot_owner_fee, patch_userbot_user_token_fee}}, state::AppState};


pub fn global_config_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{fee}", patch(patch_admin_fee)
            .patch(patch_bot_fee)
            .patch(patch_userbot_fee)
            .patch(patch_userbot_user_token_fee)
            .patch(patch_userbot_owner_fee)
        )
        .layer(from_fn_with_state(state, admin_access))
}
