//! Application state and router construction.

use std::sync::Arc;

use axum::http::{HeaderValue, Method};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::controllers;
use crate::middleware::auth_middleware;

/// Shared application state injected into every handler.
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL client.
    pub db: sakaloka_data::postgres::PgClient,
    /// JWT signing/verification keys.
    pub jwt_keys: Arc<sakaloka_secure::jwt::JwtKeys>,
}

/// Builds the top-level Axum router with all routes and middleware.
pub fn router(state: AppState) -> Router {
    let allowed_origins = std::env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173,http://localhost:1420".to_string());
    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .max_age(std::time::Duration::from_secs(3600));

    // Public routes — no auth required (login, register, refresh).
    let public_routes = Router::new().merge(controllers::auth::public_routes());

    // Protected routes — require valid JWT via auth_middleware.
    let protected_routes = Router::new()
        .merge(controllers::auth::protected_routes())
        .merge(controllers::bom::routes())
        .merge(controllers::product::routes())
        .merge(controllers::brand::routes())
        .merge(controllers::category::routes())
        .merge(controllers::user::routes())
        .merge(controllers::role::routes())
        .merge(controllers::organization::routes())
        .merge(controllers::pos::routes())
        .merge(controllers::stock::routes())
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let api_routes = Router::new().merge(public_routes).merge(protected_routes);

    Router::new()
        .route("/health", axum::routing::get(health))
        .merge(Scalar::with_url("/scalar", crate::docs::ApiDoc::openapi()))
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

/// Health-check endpoint returning status and build version.
async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "version": env!("SAKALOKA_VERSION"),
    }))
}

/// Runs embedded SQLx migrations against PostgreSQL.
///
/// # Errors
///
/// Returns an error if any migration fails.
pub async fn run_migrations(db: &sakaloka_data::postgres::PgClient) -> anyhow::Result<()> {
    db.run_migrations()
        .await
        .map_err(|e| anyhow::anyhow!("migration failed: {e}"))?;
    tracing::info!("all migrations applied");
    Ok(())
}
