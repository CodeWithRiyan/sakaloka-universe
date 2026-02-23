//! Application router — all routes registered here.

use axum::{routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::health::health_handler;

/// Builds the Axum router with all registered routes and middleware.
///
/// Currently registered routes (Sprint 1):
/// - `GET /health` — liveness / readiness check for Uptime Kuma
pub fn build_router() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
