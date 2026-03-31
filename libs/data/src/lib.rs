// ============================================================
// Sakaloka-Universe — libs/data
// Iron Curtain directives — mandatory on every lib.rs
// ============================================================
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! # sakaloka-data
//!
//! Database and messaging client abstractions for Sakaloka-Universe.
//!
//! ## Backends
//! - **PostgreSQL** (default) — production database via SQLx
//! - **SurrealDB** — legacy, behind `surrealdb-backend` feature flag
//! - **Qdrant** — vector search, behind `qdrant` feature flag
//! - **Zenoh** — pub/sub messaging, behind `zenoh` feature flag
//!
//! ## Rules
//! - Always use the authenticated client — never embed raw credentials.
//! - Use parameterized queries — never interpolate user input into SQL.

/// PostgreSQL client module (default database backend).
pub mod postgres;

#[cfg(feature = "qdrant")]
/// Qdrant vector search client.
pub mod qdrant;
#[cfg(feature = "surrealdb-backend")]
/// Legacy SurrealDB client module.
pub mod surreal;
#[cfg(feature = "zenoh")]
/// Eclipse Zenoh pub/sub messaging client.
pub mod zenoh;
