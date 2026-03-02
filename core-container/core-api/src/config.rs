use std::{env, str::FromStr};

use tracing::Level;

use crate::logger::LoggerConfig;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub host: String,
    pub port: u16,
    pub logger: LoggerConfig,
    pub admin: AdminConfig,
    pub tokens: TokenConfig,
}
#[derive(Clone, Debug)]
pub struct AdminConfig {
    pub username: String,
    pub password: String,
}
#[derive(Clone, Debug)]
pub struct TokenConfig {
    pub admins_refresh: String,
    pub admins_access: String,
    pub bots_refresh: String,
    pub bots_access: String,
    pub userbots: String,
    pub user_tokens: String,
}
impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: get_env("DATABASE_URL", "postgres://postgres:postgres@localhost/app"),
            redis_url: get_env("REDIS_URL", "docker pull redis:latest"),
            host: get_env("HOST", "0.0.0.0"),
            port: get_env("PORT", "3000").parse().unwrap_or(3000),
            logger: LoggerConfig {
                log_path: get_env("LOG_PATH", "./logs").into(),
                console_level: Level::from_str(&get_env("LOG_LEVEL", "debug"))
                    .unwrap_or(Level::DEBUG),
            },
            admin: AdminConfig {
                username: get_env("ADMIN_USERNAME", "admin"),
                password: get_env("ADMIN_PASSWORD", "password"),
            },
            tokens: TokenConfig {
                admins_refresh: get_env("SECRET_ADMINS_REFRESH", "admins refresh"),
                admins_access: get_env("SECRET_ADMINS_ACCESS", "admins access"),
                bots_refresh: get_env("SECRET_BOTS_REFRESH", "bot refresh"),
                bots_access: get_env("SECRET_BOT_ACCESS", "bots access"),
                userbots: get_env("SECRET_USERBOTS", "bebebebebebebebebebebebebebebebe"),
                user_tokens: get_env("SECRET_USER_TOKENS", "bebebebebebebebebebebebebebebebe"),
            },
        }
    }
}

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
