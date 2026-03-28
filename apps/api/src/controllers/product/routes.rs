//! Route definitions for `/products`.

use axum::{routing::get, Router};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/products` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/products", get(handlers::list))
        .route("/products/{id}", get(handlers::show))
        .route_layer(RequireScope::new(Scope::ProductRead));

    let write_routes = Router::new()
        .route(
            "/products",
            axum::routing::post(handlers::create).layer(RequireScope::new(Scope::ProductCreate)),
        )
        .route(
            "/products/{id}",
            axum::routing::patch(handlers::update).layer(RequireScope::new(Scope::ProductUpdate)),
        )
        .route(
            "/products/{id}",
            axum::routing::delete(handlers::remove).layer(RequireScope::new(Scope::ProductDelete)),
        );

    Router::new().merge(read_routes).merge(write_routes)
}
