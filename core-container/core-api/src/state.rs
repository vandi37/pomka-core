use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::{hash::PasswordHasherService, tokens::TokensState};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub password_hasher_service: PasswordHasherService,
    pub tokens_state: TokensState,
}
