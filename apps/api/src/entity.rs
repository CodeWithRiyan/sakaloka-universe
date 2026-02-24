//! Dummy entity routes to test RBAC and JWT Auth.

use axum::{
    routing::{get, post},
    Json, Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::state::AppState;

/// The entity router module with protected endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(read_handler).route_layer(RequireScope::new(Scope::EntityRead)),
        )
        .route(
            "/",
            post(write_handler).route_layer(RequireScope::new(Scope::EntityWrite)),
        )
}

async fn read_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": "Entity read successfully"
    }))
}

async fn write_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": "Entity written successfully"
    }))
}
