//! Application router — all routes registered here.

use axum::{routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::state::AppState;
use crate::health::health_handler;
use crate::auth;

/// Builds the Axum router with all registered routes and middleware.
///
/// Currently registered routes (Sprint 3):
/// - `GET /health` — liveness / readiness check for Uptime Kuma
/// - `POST /auth/login` — authenticate and issue tokens
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .nest("/auth", auth::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
