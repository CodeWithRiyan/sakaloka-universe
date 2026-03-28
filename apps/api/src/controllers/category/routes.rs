//! Route definitions for `/products/categories`.

use axum::{routing::get, Router};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/products/categories` routes (nested under `/api` by the
/// top-level router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/products/categories", get(handlers::list))
        .route("/products/categories/{id}", get(handlers::show))
        .route_layer(RequireScope::new(Scope::CategoryRead));

    let write_routes = Router::new()
        .route(
            "/products/categories",
            axum::routing::post(handlers::create).layer(RequireScope::new(Scope::CategoryCreate)),
        )
        .route(
            "/products/categories/{id}",
            axum::routing::patch(handlers::update).layer(RequireScope::new(Scope::CategoryUpdate)),
        )
        .route(
            "/products/categories/{id}",
            axum::routing::delete(handlers::remove).layer(RequireScope::new(Scope::CategoryDelete)),
        );

    Router::new().merge(read_routes).merge(write_routes)
}
