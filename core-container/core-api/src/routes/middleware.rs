use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn logging(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    // Call the next middleware/handler
    let response = next.run(req).await;

    let status = response.status().as_u16();
    let latency = start.elapsed();

    if status >= 500 {
        tracing::error!(method = %method, path = %path, status, latency = ?latency, "request error");
    } else {
        tracing::info!(method = %method, path = %path, status, latency = ?latency, "request");
    }

    response
}
