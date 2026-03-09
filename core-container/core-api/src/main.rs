mod config;
mod error;
mod hash;
mod logger;
mod models;
mod routes;
mod state;
mod tokens;
mod auth_prefix;
mod userbot;

use redis::Client;
use std::{net::SocketAddr, process::exit, sync::Arc};

use config::Config;
use state::AppState;

use sqlx::postgres::PgPoolOptions;

use crate::{hash::PasswordHasherService, logger::init_logger, routes::router, tokens::TokensState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    let _guards = init_logger(config.logger);
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|e| {
            tracing::error!(target:"setup", "gotten error: {e}");
            exit(1)
        });

    let client = Client::open(config.redis_url).unwrap_or_else(|e| {
        tracing::error!(target:"setup", "gotten error: {e}");
        exit(1)
    });

    let redis = client.get_connection_manager().await.unwrap_or_else(|e| {
        tracing::error!(target:"setup","gotten error: {e}");
        exit(1)
    });
    let password_hasher_service = PasswordHasherService::new().unwrap_or_else(|e| {
        tracing::error!(target:"setup","gotten error: {e}");
        exit(1)
    });
    let password = password_hasher_service
        .hash_password(&config.admin.password)
        .unwrap_or_else(|e| {
            tracing::error!(target:"setup","gotten error: {e}");
            exit(1)
        });
    sqlx::query!(
        "insert into admins (username, password) values ($1, $2) on conflict (username) do update set password=$2",
        config.admin.username, password
    ).fetch_all(&db).await.unwrap_or_else(|e| {
        tracing::error!(target:"setup","gotten error: {e}");
        exit(1)
    });
    let tokens_state = TokensState::try_from(config.tokens).unwrap_or_else(|e| {
        tracing::error!(target:"setup","gotten error: {e:?}");
        exit(1);
    });
    let state = Arc::new(AppState {
        db,
        redis,
        password_hasher_service,
        tokens_state,
    });
    let app = router(state);

    let addr = SocketAddr::new(
        config.host.parse().unwrap_or_else(|e| {
            tracing::error!(target:"setup","gotten error: {e}");
            exit(1)
        }),
        config.port,
    );

    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap_or_else(|e| {
                tracing::error!(target:"setup","gotten error: {e}");
                exit(1)
            }),
        app,
    )
    .await
    .unwrap_or_else(|e| {
        tracing::error!(target:"setup","gotten error: {e}");
        exit(1)
    });
}
