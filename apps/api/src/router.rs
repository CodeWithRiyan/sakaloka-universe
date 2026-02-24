//! Application router — all routes registered here.

use axum::{middleware, routing::get, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::auth;
use crate::entity;
use crate::health::health_handler;
use crate::state::AppState;

/// Builds the Axum router with all registered routes and middleware.
///
/// Currently registered routes:
/// - `GET /health` — liveness / readiness check for Uptime Kuma
/// - `POST /auth/login` — authenticate and issue tokens
/// - `/entity/` — protected routes enforcing `RequireScope`
pub fn build_router(state: AppState) -> Router {
    let auth_routes = auth::router();

    // Entity routes require an active connection + JWT claims logic,
    // so we apply auth_middleware globally to this scope.
    let entity_routes = entity::router().route_layer(middleware::from_fn_with_state(
        state.clone(),
        crate::middleware::auth_middleware,
    ));

    Router::new()
        .route("/health", get(health_handler))
        .nest("/auth", auth_routes)
        .nest("/entity", entity_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
