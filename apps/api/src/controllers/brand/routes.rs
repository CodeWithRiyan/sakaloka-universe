//! Route definitions for `/products/brands`.

use axum::{routing::get, Router};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/products/brands` routes (nested under `/api` by the
/// top-level router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/products/brands", get(handlers::list))
        .route("/products/brands/{id}", get(handlers::show))
        .route_layer(RequireScope::new(Scope::BrandRead));

    let write_routes = Router::new()
        .route(
            "/products/brands",
            axum::routing::post(handlers::create).layer(RequireScope::new(Scope::BrandCreate)),
        )
        .route(
            "/products/brands/{id}",
            axum::routing::patch(handlers::update).layer(RequireScope::new(Scope::BrandUpdate)),
        )
        .route(
            "/products/brands/{id}",
            axum::routing::delete(handlers::remove).layer(RequireScope::new(Scope::BrandDelete)),
        );

    Router::new().merge(read_routes).merge(write_routes)
}
