use tauri::Manager;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init(app_handle: &tauri::AppHandle) {
    let log_dir = app_handle
        .path()
        .app_log_dir()
        .unwrap_or_else(|_| std::env::temp_dir().join("kateb"));

    let _ = std::fs::create_dir_all(&log_dir);

    let file_appender = rolling::daily(&log_dir, "kateb.log");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,kateb_lib=debug"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_target(false)
                .compact(),
        )
        .with(
            fmt::layer()
                .with_writer(file_appender)
                .with_target(true)
                .with_ansi(false),
        )
        .init();

    tracing::info!("Logging initialized, log dir: {:?}", log_dir);
}
