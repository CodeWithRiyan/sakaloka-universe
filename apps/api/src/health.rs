//! Health check handler — US-03.
//!
//! Returns a JSON response so Uptime Kuma and Docker health checks
//! can confirm Earth is alive and ready.

use axum::{http::StatusCode, response::IntoResponse, Json};

/// Health report body.
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    /// Status code (e.g. "ok").
    pub status: String,
    /// Planet name.
    pub planet: String,
    /// Service name.
    pub service: String,
    /// Package version.
    pub version: String,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "System is healthy", body = HealthResponse)
    )
)]
pub async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok".into(),
            planet: "earth".into(),
            service: "sakaloka-api".into(),
            version: env!("CARGO_PKG_VERSION").into(),
        }),
    )
}
