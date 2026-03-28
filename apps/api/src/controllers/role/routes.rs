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
        .route_layer(RequireScope::new(Scope::RoleRead));

    let create_routes = Router::new().route(
        "/roles",
        axum::routing::post(handlers::create).layer(RequireScope::new(Scope::RoleCreate)),
    );

    let update_routes = Router::new().route(
        "/roles/{id}",
        axum::routing::patch(handlers::update).layer(RequireScope::new(Scope::RoleUpdate)),
    );

    let delete_routes = Router::new().route(
        "/roles/{id}",
        axum::routing::delete(handlers::remove).layer(RequireScope::new(Scope::RoleDelete)),
    );

    Router::new()
        .merge(read_routes)
        .merge(create_routes)
        .merge(update_routes)
        .merge(delete_routes)
}
