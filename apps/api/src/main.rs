//! Sakaloka-Universe — Earth: Axum REST API entry point.

use std::sync::Arc;

use sakaloka_api::app::{self, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url =
        std::env::var("SURREALDB_URL").unwrap_or_else(|_| "ws://127.0.0.1:58000".to_string());
    let db_user = std::env::var("SURREALDB_USER").unwrap_or_else(|_| "root".to_string());
    let db_pass = std::env::var("SURREALDB_PASS").unwrap_or_else(|_| "sakaloka-dev".to_string());

    tracing::info!("connecting to SurrealDB at {}", db_url);
    let db = sakaloka_data::surreal::SurrealClient::connect(&db_url).await?;
    db.signin(&db_user, &db_pass).await?;
    tracing::info!("SurrealDB connected");

    app::run_migrations(&db).await?;

    let jwt_keys = sakaloka_secure::jwt::JwtKeys::from_env()?;
    let state = AppState {
        db,
        jwt_keys: Arc::new(jwt_keys),
    };

    let router = app::router(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
