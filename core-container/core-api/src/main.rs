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
mod transactions;
mod services;
mod claim_daily_reward;

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

    let redis = Client::open(config.redis_url).unwrap_or_else(|e| {
        tracing::error!(target:"setup", "gotten error: {e}");
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
        daily_claim: config.daily_claim,
        reset_hour_utc: config.reset_hour_utc,
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
