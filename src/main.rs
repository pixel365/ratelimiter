use crate::app::App;
use crate::config::runtime::RuntimeConfig;
use crate::core::fixed_window::FixedWindowLimiter;
use crate::core::limiter::LimiterImpl;
use crate::transport::http;
use std::time::Duration;

mod app;
mod config;
mod core;
mod transport;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let limiter = LimiterImpl::FixedWindow(FixedWindowLimiter::new());
    limiter.cleanup_task(Duration::from_secs(10));

    let cfg = RuntimeConfig::new();
    let app = App { limiter, cfg };

    http::server::run(app, shutdown_signal()).await
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.unwrap();
    }
}
