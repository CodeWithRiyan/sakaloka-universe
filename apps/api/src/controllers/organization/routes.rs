//! Route definitions for `/organizations`.

use axum::{
    routing::{get, post},
    Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/organizations` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/organizations", get(handlers::list))
        .route("/organizations/current", get(handlers::current))
        .route("/organizations/{id}", get(handlers::show))
        .route_layer(RequireScope::new(Scope::EntityRead));

    let write_routes = Router::new()
        .route("/organizations", post(handlers::create))
        .route("/organizations/select", post(handlers::select))
        .route(
            "/organizations/{id}",
            axum::routing::patch(handlers::update).delete(handlers::remove),
        )
        .route_layer(RequireScope::new(Scope::EntityWrite));

    Router::new().merge(read_routes).merge(write_routes)
}
