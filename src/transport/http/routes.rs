use crate::core::limiter::Limiter;
use crate::core::types::{CheckInput, CheckOutput};
use crate::transport::http::types::ErrorResponse;
use crate::App;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

pub async fn health_check() -> Result<&'static str, ()> {
    Ok("OK")
}

pub async fn check(
    State(app): State<App>,
    Json(req): Json<CheckInput>,
) -> Result<Json<CheckOutput>, (StatusCode, Json<ErrorResponse>)> {
    req.validate(256).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: e.as_str() }),
        )
    })?;

    let decision = app.limiter.check(req);
    Ok(Json(CheckOutput {
        allowed: decision.allowed,
        remaining: decision.remaining,
        reset_ms: decision.reset_ms,
    }))
}
