//! Health check handler — US-03.
//!
//! Returns a JSON response so Uptime Kuma and Docker health checks
//! can confirm Earth is alive and ready.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/// Handles `GET /health`.
///
/// Returns `200 OK` with a JSON body confirming the planet name and status.
/// This endpoint intentionally has no auth requirement — it is the only
/// unauthenticated surface on Earth.
pub async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "planet": "earth",
            "service": "sakaloka-api",
            "version": env!("CARGO_PKG_VERSION"),
        })),
    )
}
