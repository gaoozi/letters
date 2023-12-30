use std::env;

use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

pub fn setup() -> Vec<WorkerGuard> {
    let log_env = env::var("RUST_LOG").expect("Can't found 'RUST_LOG' env.");

    let mut guards = Vec::new();
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_env.into());

    let console_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stderr);

    let file_appender = RollingFileAppender::builder()
        .max_log_files(5)
        .rotation(Rotation::DAILY)
        .filename_prefix("letters")
        .filename_suffix("log")
        .build("./logs")
        .expect("initializing rolling file appender failed");
    let (non_blocking_appender, file_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_target(true)
        .with_ansi(false)
        .with_level(true)
        .with_writer(non_blocking_appender)
        .with_filter(tracing_subscriber::filter::LevelFilter::TRACE);
    guards.push(file_guard);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .with(env_filter)
        .init();

    guards
}
