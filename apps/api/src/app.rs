//! Application state and router construction.

use std::sync::Arc;

use axum::http::{HeaderValue, Method};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::controllers;
use crate::middleware::auth_middleware;

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
/// At runtime the directory is resolved from the `MIGRATIONS_DIR` environment
/// variable (set in Docker images). When the variable is absent the
/// compile-time `CARGO_MANIFEST_DIR` path is used as a fallback so that
/// `cargo run` during local development still works.
///
/// # Errors
///
/// Returns an error if any migration file cannot be read or executed.
pub async fn run_migrations(db: &sakaloka_data::surreal::SurrealClient) -> anyhow::Result<()> {
    fn legacy_schema_conflict(error: &sakaloka_data::surreal::SurrealError) -> bool {
        let message = error.to_string().to_ascii_lowercase();
        message.contains("already exists") || message.contains("already defined")
    }

    let surql_dir = match std::env::var("MIGRATIONS_DIR") {
        Ok(dir) => std::path::PathBuf::from(dir)
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("Invalid MIGRATIONS_DIR: {e}"))?,
        Err(_) => std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("libs")
            .join("data")
            .join("surql"),
    };

    db.execute_raw("DEFINE TABLE IF NOT EXISTS schema_migration SCHEMALESS;")
        .await
        .map_err(|error| anyhow::anyhow!("failed to initialize migration store: {error}"))?;

    let mut entries: Vec<_> = std::fs::read_dir(&surql_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "surql"))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if db.migration_applied(&name).await.map_err(|error| {
            anyhow::anyhow!("failed to check migration marker for {name}: {error}")
        })? {
            tracing::info!(file = %name, "skipping migration already applied");
            continue;
        }

        let sql = std::fs::read_to_string(&path)?;
        tracing::info!(file = %name, "applying migration");
        match db.execute_raw(&sql).await {
            Ok(()) => {}
            Err(error) if legacy_schema_conflict(&error) => {
                tracing::warn!(
                    file = %name,
                    error = %error,
                    "migration conflicts with existing schema; marking as already applied"
                );
            }
            Err(error) => return Err(error.into()),
        }
        db.mark_migration_applied(&name).await.map_err(|error| {
            anyhow::anyhow!("failed to record migration marker for {name}: {error}")
        })?;
    }

    tracing::info!("all migrations applied");
    Ok(())
}
