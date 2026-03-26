//! Route definitions for `/products`.

use axum::{routing::get, Router};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/products` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/products", get(handlers::list))
        .route("/products/{id}", get(handlers::show))
        .route(
            "/products",
            axum::routing::post(handlers::create).layer(RequireScope::new(Scope::EntityWrite)),
        )
        .route(
            "/products/{id}",
            axum::routing::patch(handlers::update).layer(RequireScope::new(Scope::EntityWrite)),
        )
        .route(
            "/products/{id}",
            axum::routing::delete(handlers::remove).layer(RequireScope::new(Scope::EntityDelete)),
        )
}
