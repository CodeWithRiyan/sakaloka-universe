//! Route definitions for `/inventory/pos-stock`.

use axum::{
    routing::{get, post},
    Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/inventory/pos-stock` routes (nested under `/api` by the
/// top-level router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/inventory/pos-stock", get(handlers::list))
        .route("/inventory/pos-stock/low-stock", get(handlers::low_stock))
        .route("/inventory/pos-stock/{id}", get(handlers::show))
        .route("/inventory/pos-stock/{id}/history", get(handlers::history));

    let write_routes = Router::new()
        .route(
            "/inventory/pos-stock/products/{product_id}/adjust",
            post(handlers::adjust),
        )
        .route_layer(RequireScope::new(Scope::EntityWrite));

    Router::new().merge(read_routes).merge(write_routes)
}
