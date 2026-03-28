//! Route definitions for `/pos`.

use axum::{routing::get, Router};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/pos` routes (nested under `/api` by the top-level router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/pos/menu", get(handlers::menu))
        .route("/pos/orders", get(handlers::list_orders))
        .route("/pos/orders/active", get(handlers::active_orders))
        .route("/pos/orders/history", get(handlers::order_history))
        .route("/pos/orders/{id}", get(handlers::show_order))
        .route_layer(RequireScope::new(Scope::PosRead));

    let create_routes = Router::new().route(
        "/pos/orders",
        axum::routing::post(handlers::create_order).layer(RequireScope::new(Scope::PosCreate)),
    );

    let update_routes = Router::new().route(
        "/pos/orders/{id}",
        axum::routing::patch(handlers::update_order).layer(RequireScope::new(Scope::PosUpdate)),
    );

    Router::new()
        .merge(read_routes)
        .merge(create_routes)
        .merge(update_routes)
}
