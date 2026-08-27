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

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use sqlx::query;

use crate::{
    auth_prefix::AuthPrefix, error::AppError, routes::bots::Bot, state::AppState, tokens::validate_jwt
};

pub async fn bot_access(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    match req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth| AuthPrefix::cut_prefix(auth))
        .ok_or(AppError::InvalidToken)?
    {
        (AuthPrefix::BotAccess, token) => {
            let claims = validate_jwt::<()>(token, state.tokens_state.bots.access.as_bytes())
                .or(Err(AppError::InvalidToken))?;
            query!("select id from bots where id=$1", claims.sub).fetch_optional(&state.db)
            .await.map_err(|e| {
                tracing::error!(target: "bots-auth", error=?e, id=claims.sub, "error selecting bot");
                AppError::Internal
            })?.ok_or(AppError::BotNotFound(claims.sub))?;
            req.extensions_mut().insert(Bot { id: claims.sub });
            tracing::info!(target:"bot-auth", id=claims.sub, "gotten access token from bot");
            Ok(next.run(req).await)
        }
        _ => Err(AppError::InvalidToken),
    }
}

pub async fn bot_refresh(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    match req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth| AuthPrefix::cut_prefix(auth))
        .ok_or(AppError::InvalidToken)?
    {
        (AuthPrefix::BotRefresh, token) => {
            let claims = validate_jwt::<()>(token, state.tokens_state.bots.refresh.as_bytes())
                .or(Err(AppError::InvalidToken))?;
            query!("select id from bots where id=$1", claims.sub).fetch_optional(&state.db)
            .await.map_err(|e| {
                tracing::error!(target: "bot-auth", error=?e, id=claims.sub, "error selecting bot");
                AppError::Internal
            })?.ok_or(AppError::BotNotFound(claims.sub))?;
            req.extensions_mut().insert(Bot { id: claims.sub });
            tracing::info!(target:"bot-auth", id=claims.sub, "gotten refresh token from bot");
            Ok(next.run(req).await)
        }
        _ => Err(AppError::InvalidToken),
    }
}
