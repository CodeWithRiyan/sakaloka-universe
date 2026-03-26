//! Route definitions for `/auth`.

use axum::{
    routing::{get, post},
    Router,
};

use crate::app::AppState;

use super::handlers;

/// Public `/auth` routes — no JWT required.
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(handlers::login))
        .route("/auth/register", post(handlers::register))
        .route("/auth/refresh", post(handlers::refresh))
}

/// Protected `/auth` routes — require valid JWT.
pub fn protected_routes() -> Router<AppState> {
    Router::new().route("/auth/profile", get(handlers::profile))
}
