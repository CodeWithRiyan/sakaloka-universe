// ============================================================
// 🌍 Sakaloka-Universe — apps/api (Earth)
// Entry point — anyhow is allowed only here (binary crate).
// libs/ use thiserror. Never use .unwrap() or .expect() outside tests.
// ============================================================
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![forbid(unsafe_code)]

mod health;
mod router;
#[cfg(test)]
mod tests;

use anyhow::Context;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Application entry point for 🌍 Earth (Sakaloka API server).
///
/// Binds on `0.0.0.0:3000` inside the Docker container.
/// The external binding `127.0.0.1:53000 → 3000` is managed by Docker Compose.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured tracing. Use RUST_LOG env var to control level.
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!(planet = "earth", "🌍 Sakaloka API starting");

    let app = router::build_router();
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind to {addr}"))?;

    info!(addr, "🌍 Earth is listening — external: 127.0.0.1:53000");

    axum::serve(listener, app)
        .await
        .context("API server crashed")?;

    Ok(())
}
