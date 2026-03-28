//! Route definitions for `/users`.

use axum::{routing::get, Router};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/users` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/users", get(handlers::list))
        .route("/users/{id}", get(handlers::show))
        .route_layer(RequireScope::new(Scope::UserRead));

    let create_routes = Router::new().route(
        "/users",
        axum::routing::post(handlers::create).layer(RequireScope::new(Scope::UserCreate)),
    );

    let update_routes = Router::new().route(
        "/users/{id}",
        axum::routing::patch(handlers::update).layer(RequireScope::new(Scope::UserUpdate)),
    );

    let delete_routes = Router::new().route(
        "/users/{id}",
        axum::routing::delete(handlers::remove).layer(RequireScope::new(Scope::UserDelete)),
    );

    Router::new()
        .merge(read_routes)
        .merge(create_routes)
        .merge(update_routes)
        .merge(delete_routes)
}
