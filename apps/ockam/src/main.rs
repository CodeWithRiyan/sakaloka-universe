// ============================================================
// 🔴 Sakaloka-Universe — apps/ockam (Mars)
// Iron Curtain directives — mandatory on every binary crate.
// ============================================================
#![doc = "🔴 **Mars** — Sakaloka-Universe Ockam secure channel broker.

Establishes end-to-end encrypted channels between all planets so
all inter-planet traffic is encrypted at the transport layer —
in addition to JWT application-layer auth."]
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

use anyhow::Context;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Entry point for 🔴 Mars (Ockam secure channel broker).
///
/// Binds on `0.0.0.0:4000` inside the Docker container.
/// External binding: `127.0.0.1:54000 → 4000` via Docker Compose.
///
/// Connects to Earth, Jupiter, Saturn, and Uranus via
/// Ockam secure channels to provide end-to-end encrypted transport.
///
/// ## Sprint 12 (US-40)
/// Full Ockam node implementation is scheduled for Sprint 12.
/// This scaffold satisfies the workspace compilation requirement.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!(planet = "mars", "🔴 Sakaloka Ockam broker starting");

    let port = std::env::var("OCKAM_PORT").unwrap_or_else(|_| "4000".into());
    info!(port = %port, "🔐 Mars listening — external: 127.0.0.1:54000");

    // TODO(Sprint 12 US-40): Instantiate Ockam node, configure vault,
    // and establish secure channels to Earth ↔ Jupiter, Earth ↔ Saturn,
    // Earth ↔ Uranus. See PRD §5.7 for full acceptance criteria.
    //
    // Example implementation plan:
    //   1. Load OCKAM_VAULT_SECRET from env
    //   2. Create Ockam node with TCP transport listener on port 4000
    //   3. Establish channels to each planet using mTLS identities
    //   4. Route all inter-planet traffic through this broker

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind Mars broker to {addr}"))?;

    info!(%addr, "🔴 Mars Ockam broker is ready");

    // Accept loop — placeholder until Ockam crate integration (Sprint 12).
    loop {
        let (_stream, peer) = listener
            .accept()
            .await
            .context("Failed to accept connection on Mars")?;
        info!(peer = %peer, "🔴 Mars: incoming connection (Ockam tunnel pending Sprint 12)");
    }
}
