//! Route definitions for `/boms`.

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/boms` routes (nested under `/api` by the top-level router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/boms", get(handlers::list))
        .route("/boms/{id}", get(handlers::show))
        .route("/boms/product/{product_id}", get(handlers::show_by_product))
        .route("/boms/{id}/items", get(handlers::list_items))
        .route(
            "/boms/{id}/production-runs",
            get(handlers::list_production_runs),
        )
        .route(
            "/boms/{id}/production-runs/{run_id}",
            get(handlers::show_production_run),
        )
        .route(
            "/boms/{id}/production-runs/{run_id}/consumptions",
            get(handlers::list_consumptions),
        )
        .route_layer(RequireScope::new(Scope::BomRead));

    let write_routes = Router::new()
        .route(
            "/boms",
            post(handlers::create).layer(RequireScope::new(Scope::BomCreate)),
        )
        .route(
            "/boms/{id}",
            patch(handlers::update).layer(RequireScope::new(Scope::BomUpdate)),
        )
        .route(
            "/boms/{id}",
            delete(handlers::remove).layer(RequireScope::new(Scope::BomDelete)),
        )
        .route(
            "/boms/{id}/activate",
            post(handlers::activate).layer(RequireScope::new(Scope::BomUpdate)),
        )
        .route(
            "/boms/{id}/items",
            post(handlers::add_item).layer(RequireScope::new(Scope::BomUpdate)),
        )
        .route(
            "/boms/{bom_id}/items/{item_id}",
            patch(handlers::update_item).layer(RequireScope::new(Scope::BomUpdate)),
        )
        .route(
            "/boms/{bom_id}/items/{item_id}",
            delete(handlers::remove_item).layer(RequireScope::new(Scope::BomUpdate)),
        )
        .route(
            "/boms/{id}/production-runs",
            post(handlers::create_production_run).layer(RequireScope::new(Scope::BomUpdate)),
        )
        .route(
            "/boms/{id}/production-runs/{run_id}",
            patch(handlers::update_production_run).layer(RequireScope::new(Scope::BomUpdate)),
        )
        .route(
            "/boms/{id}/production-runs/{run_id}/consumptions",
            post(handlers::add_consumption).layer(RequireScope::new(Scope::BomUpdate)),
        );

    Router::new().merge(read_routes).merge(write_routes)
}
