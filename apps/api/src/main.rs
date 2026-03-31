//! Sakaloka-Universe — Earth: Axum REST API entry point.

use sakaloka_api::app::{self, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").map_err(|_| {
        anyhow::anyhow!(
            "DATABASE_URL must be set — use .env file locally or set in Docker environment"
        )
    })?;

    tracing::info!("connecting to PostgreSQL");
    let db = sakaloka_data::postgres::PgClient::connect(&database_url).await?;
    tracing::info!("PostgreSQL connected");

    app::run_migrations(&db).await?;

    let jwt_keys = sakaloka_secure::jwt::JwtKeys::from_env()?;
    let state = AppState {
        db,
        jwt_keys: std::sync::Arc::new(jwt_keys),
    };

    let router = app::router(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
