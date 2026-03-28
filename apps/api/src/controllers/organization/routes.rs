//! Route definitions for `/organizations`.

use axum::{
    routing::{get, post},
    Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;

use super::handlers;

/// Registers all `/organizations` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/organizations", get(handlers::list))
        .route("/organizations/current", get(handlers::current))
        .route("/organizations/{id}", get(handlers::show))
        .route_layer(RequireScope::new(Scope::OrganizationRead));

    let create_routes = Router::new().route(
        "/organizations",
        post(handlers::create).layer(RequireScope::new(Scope::OrganizationCreate)),
    );

    let select_routes = Router::new().route(
        "/organizations/select",
        post(handlers::select).layer(RequireScope::new(Scope::OrganizationSelect)),
    );

    let update_routes = Router::new().route(
        "/organizations/{id}",
        axum::routing::patch(handlers::update).layer(RequireScope::new(Scope::OrganizationUpdate)),
    );

    let delete_routes = Router::new().route(
        "/organizations/{id}",
        axum::routing::delete(handlers::remove).layer(RequireScope::new(Scope::OrganizationDelete)),
    );

    Router::new()
        .merge(read_routes)
        .merge(create_routes)
        .merge(select_routes)
        .merge(update_routes)
        .merge(delete_routes)
}
