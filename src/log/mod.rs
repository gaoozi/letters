use tracing_appender::{rolling::{RollingFileAppender, Rotation}, non_blocking::WorkerGuard};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt, fmt, Layer};

pub fn setup() -> Vec<WorkerGuard> {
    let mut guards = Vec::new();
    let env_filter = EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "letters=debug,tower_http=debug,axum::rejection=trace".into());

    let console_layer = tracing_subscriber::fmt::layer().pretty().with_writer(std::io::stderr);

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
