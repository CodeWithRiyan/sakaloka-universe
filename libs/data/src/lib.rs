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
//! - **PostgreSQL** — production database via SQLx
//!
//! ## Rules
//! - Always use the authenticated client — never embed raw credentials.
//! - Use parameterized queries — never interpolate user input into SQL.

/// PostgreSQL client module.
pub mod postgres;
