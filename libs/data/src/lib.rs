// ============================================================
// 🌌 Sakaloka-Universe — libs/data
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
//! ## Planets served
//! - 🪐 **Jupiter** — SurrealDB graph/document database client
//! - 🔵 **Uranus** — Qdrant vector search client
//! - 🪐 **Saturn** — Eclipse Zenoh pub/sub messaging client
//!
//! ## Rules
//! - Always use the authenticated client — never embed raw credentials.
//! - All SurrealDB schema is defined in `.surql` files under `db/migrations/`.
//! - Tables use `SCHEMAFULL` — never create tables ad hoc.
//! - All Zenoh event handlers must be idempotent.

pub mod qdrant;
pub mod surreal;
pub mod zenoh;
