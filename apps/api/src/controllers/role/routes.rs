//! Route definitions for `/roles`.

use axum::{routing::get, Router};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/roles` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/roles", get(handlers::list))
        .route("/roles/permissions", get(handlers::permissions))
        .route("/roles/{id}", get(handlers::show))
        .route_layer(RequireScope::new(Scope::UserRead));

    let write_routes = Router::new()
        .route("/roles", axum::routing::post(handlers::create))
        .route(
            "/roles/{id}",
            axum::routing::patch(handlers::update).delete(handlers::remove),
        )
        .route_layer(RequireScope::new(Scope::UserManage));

    Router::new().merge(read_routes).merge(write_routes)
}
