use crate::app::App;
use crate::config::runtime::RuntimeConfig;
use crate::core::fixed_window::FixedWindowLimiter;
use crate::core::limiter::LimiterImpl;
use crate::transport::http;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

mod app;
mod config;
mod core;
mod transport;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let stop = CancellationToken::new();

    let limiter = LimiterImpl::FixedWindow(FixedWindowLimiter::new());
    limiter.cleanup_task(Duration::from_secs(10), stop.clone());

    let cfg = RuntimeConfig::new();
    let app = App { limiter, cfg };

    let shutdown = {
        let stop = stop.clone();
        async move {
            shutdown_signal().await;
            stop.cancel();
        }
    };

    http::server::run(app, shutdown).await
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
