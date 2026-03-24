//! Application state and router construction.

use std::sync::Arc;

use axum::http::Method;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::controllers;

/// Shared application state injected into every handler.
#[derive(Clone)]
pub struct AppState {
    /// SurrealDB client (Jupiter).
    pub db: sakaloka_data::surreal::SurrealClient,
    /// JWT signing/verification keys.
    pub jwt_keys: Arc<sakaloka_secure::jwt::JwtKeys>,
}

/// Builds the top-level Axum router with all routes and middleware.
pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(3600));

    let api_routes = Router::new()
        .merge(controllers::auth::routes())
        .merge(controllers::product::routes())
        .merge(controllers::brand::routes())
        .merge(controllers::category::routes())
        .merge(controllers::user::routes())
        .merge(controllers::role::routes())
        .merge(controllers::organization::routes())
        .merge(controllers::pos::routes())
        .merge(controllers::stock::routes());

    Router::new()
        .route("/health", axum::routing::get(health))
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

/// Health-check endpoint.
async fn health() -> &'static str {
    "ok"
}

/// Applies SurrealQL migrations from the `libs/data/surql/` directory.
///
/// # Errors
///
/// Returns an error if any migration file cannot be read or executed.
pub async fn run_migrations(db: &sakaloka_data::surreal::SurrealClient) -> anyhow::Result<()> {
    let surql_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("libs")
        .join("data")
        .join("surql");

    let mut entries: Vec<_> = std::fs::read_dir(&surql_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "surql"))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        let sql = std::fs::read_to_string(&path)?;
        tracing::info!(file = %name, "applying migration");
        db.execute_raw(&sql).await?;
    }

    tracing::info!("all migrations applied");
    Ok(())
}
