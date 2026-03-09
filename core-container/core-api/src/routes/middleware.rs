use axum::{extract::{Request, State}, http::{HeaderName, response}, middleware::Next, response::Response};
use chrono::{DateTime, Utc};
use std::{sync::Arc, time::Instant};

use crate::{error::AppError, state::AppState};

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

pub async fn adapter(State(state): State<Arc<AppState>>, req: Request, next: Next) -> Result<Response, AppError> {
    (Utc::now().timestamp() > state.tokens_state.adapter_tokens.verify(req.headers().get(ADAPTER_TOKEN)
    .ok_or(AppError::InvalidAdapterToken)?
    .to_str().ok().ok_or(AppError::InvalidAdapterToken)?).ok_or(AppError::InvalidAdapterToken)?
    ).then_some(()).ok_or(AppError::InvalidAdapterToken)?;
    Ok(next.run(req).await)
}