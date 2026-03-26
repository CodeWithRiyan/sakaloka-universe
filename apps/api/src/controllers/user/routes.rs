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

    let write_routes = Router::new()
        .route("/users", axum::routing::post(handlers::create))
        .route(
            "/users/{id}",
            axum::routing::patch(handlers::update).delete(handlers::remove),
        )
        .route_layer(RequireScope::new(Scope::UserManage));

    Router::new().merge(read_routes).merge(write_routes)
}
