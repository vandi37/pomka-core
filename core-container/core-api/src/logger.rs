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

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct LoggerConfig {
    pub log_path: PathBuf,
    pub console_level: tracing::Level,
}
pub struct LoggerGuards {
    pub _debug: tracing_appender::non_blocking::WorkerGuard,
    pub _info: tracing_appender::non_blocking::WorkerGuard,
    pub _error: tracing_appender::non_blocking::WorkerGuard,
}
use tracing_appender::rolling;
use tracing_subscriber::Layer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger(config: LoggerConfig) -> LoggerGuards {
    let log_path = config.log_path;

    let debug_file = rolling::daily(log_path.join("debug"), "debug.log");
    let info_file = rolling::daily(log_path.join("info"), "info.log");
    let error_file = rolling::daily(log_path.join("error"), "error.log");

    let (debug_writer, debug_guard) = tracing_appender::non_blocking(debug_file);
    let (info_writer, info_guard) = tracing_appender::non_blocking(info_file);
    let (error_writer, error_guard) = tracing_appender::non_blocking(error_file);

    let console_layer = fmt::layer()
        .with_target(false)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .pretty()
        .with_filter(EnvFilter::from_default_env().add_directive(config.console_level.into()));

    let debug_layer = fmt::layer()
        .json()
        .with_writer(debug_writer)
        .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);

    let info_layer = fmt::layer()
        .json()
        .with_writer(info_writer)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    let error_layer = fmt::layer()
        .json()
        .with_writer(error_writer)
        .with_filter(tracing_subscriber::filter::LevelFilter::ERROR);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(debug_layer)
        .with(info_layer)
        .with(error_layer)
        .init();
    LoggerGuards {
        _debug: debug_guard,
        _info: info_guard,
        _error: error_guard,
    }
}
