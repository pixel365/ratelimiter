use crate::app::App;
use crate::config::cli::Protocol;
use crate::config::helpers::invalid_cfg;
use crate::transport::http::routes;
use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run<F>(app: App, shutdown: F) -> std::io::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let Some(protocols) = app.cfg.protocol.as_ref() else {
        return Err(invalid_cfg("protocol is not finalized"));
    };

    if !protocols.contains(&Protocol::Http) {
        tracing::info!("HTTP protocol is disabled");
        return Ok(());
    }

    let Some(host) = app.cfg.http_host else {
        return Err(invalid_cfg("http_host is not finalized"));
    };

    let Some(port) = app.cfg.http_port else {
        return Err(invalid_cfg("http_port is not finalized"));
    };

    let addr = SocketAddr::new(host, port);

    let router = Router::new()
        .route("/healthz", get(routes::health_check))
        .route("/check", post(routes::check))
        .with_state(app);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown)
        .await
}
