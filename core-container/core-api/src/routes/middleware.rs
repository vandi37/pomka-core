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

use axum::{
    extract::{Request, State, Extension},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use sqlx::query;
use std::{sync::Arc, time::Instant};

use crate::{
    auth_prefix::AuthPrefix, error::AppError, models::executors::ExecutorRef, routes::Executor, state::AppState, tokens::validate_jwt, userbot::get_userbot
};

pub async fn logging(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let response = next.run(req).await;

    let status = response.status().as_u16();
    let latency = start.elapsed();

    if status >= 500 {
        tracing::error!(target:"request", method = %method, path = %path, status, latency = ?latency, "request error");
    } else {
        tracing::info!(target: "request", method = %method, path = %path, status, latency = ?latency, "request");
    }

    response
}
pub const ADAPTER_TOKEN: &'static str = "X-Adapter-Token";

pub async fn adapter(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    (Utc::now().timestamp()
        > state
            .tokens_state
            .adapter_tokens
            .verify(
                req.headers()
                    .get(ADAPTER_TOKEN)
                    .ok_or(AppError::InvalidAdapterToken)?
                    .to_str()
                    .ok()
                    .ok_or(AppError::InvalidAdapterToken)?,
            )
            .ok_or(AppError::InvalidAdapterToken)?)
    .then_some(())
    .ok_or(AppError::InvalidAdapterToken)?;
    Ok(next.run(req).await)
}

pub async fn access(
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
        (AuthPrefix::AdminAccess, token) => {
            let claims = validate_jwt::<()>(token, state.tokens_state.admins.access.as_bytes())
                .or(Err(AppError::InvalidToken))?;
            query!("select id from admins where id = $1", claims.sub).fetch_optional(&state.db)
            .await.map_err(|e| {
                tracing::error!(target: "auth", error=?e, id=claims.sub, "error selecting admin");
                AppError::Internal
            })?.ok_or(AppError::AdminNotFound(claims.sub))?;
            req.extensions_mut().insert(ExecutorRef::Admin(claims.sub));
            tracing::info!(target:"auth", id=claims.sub, "gotten access token from admin");
            Ok(next.run(req).await)
        }
        (AuthPrefix::BotAccess, token) => {
            let claims = validate_jwt::<()>(token, state.tokens_state.bots.access.as_bytes())
                .or(Err(AppError::InvalidToken))?;
            query!("select id from bots where id = $1", claims.sub).fetch_optional(&state.db)
            .await.map_err(|e| {
                tracing::error!(target: "auth", error=?e, id=claims.sub, "error selecting bot");
                AppError::Internal
            })?.ok_or(AppError::BotNotFound(claims.sub))?;
            req.extensions_mut().insert(ExecutorRef::Bot(claims.sub));
            tracing::info!(target:"auth", id=claims.sub, "gotten access token from bot");
            Ok(next.run(req).await)
        }
        (AuthPrefix::Userbot, token ) => {
            let (relevancy, id) = get_userbot(token, &state.tokens_state.userbots).ok_or(AppError::InvalidToken)?;
            (query!("select relevancy from userbots where id = $1", id)
                .fetch_optional(&state.db)
                .await.map_err(|e| {
                tracing::error!(target: "auth", error=?e, id, "error selecting userbot");
                AppError::Internal
            })?.ok_or(AppError::UserbotNotFound(id))?.relevancy == relevancy).then_some(()).ok_or(AppError::InvalidToken)?;
            req.extensions_mut().insert(ExecutorRef::Userbot(id));
            tracing::info!(target:"auth", id, "gotten token from userbot");
            Ok(next.run(req).await)
        }
        _ => Err(AppError::InvalidToken),
    }
}


pub async fn map_executor(
    State(state): State<Arc<AppState>>,
    Extension(executor): Extension<ExecutorRef>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let id = match executor {
        ExecutorRef::Admin(id) => query!("select id from executors where admin = $1", id)
                .fetch_one(&state.db)
                .await.map_err(|e| {
                tracing::error!(target: "map-executor", error=?e, id, "error selecting executor by admin id");
                AppError::Internal
            })?.id,
        ExecutorRef::Bot(id) => query!("select id from executors where bot = $1", id)
                .fetch_one(&state.db)
                .await.map_err(|e| {
                tracing::error!(target: "map-executor", error=?e, id, "error selecting executor by bot id");
                AppError::Internal
            })?.id,
        ExecutorRef::Userbot(id) => query!("select id from executors where userbot = $1", id)
                .fetch_one(&state.db)
                .await.map_err(|e| {
                tracing::error!(target: "map-executor", error=?e, id, "error selecting executor by userbot id");
                AppError::Internal
            })?.id,
    };
    req.extensions_mut().insert(Executor{id});
    Ok(next.run(req).await)
}