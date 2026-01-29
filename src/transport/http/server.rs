use crate::app::App;
use crate::transport::http::routes;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;

pub async fn run<F>(addr: &str, app: App, shutdown: F) -> std::io::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let app = Router::new()
        .route("/healthz", get(routes::health_check))
        .route("/check", post(routes::check))
        .with_state(app);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await
}
