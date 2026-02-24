// ============================================================
// 🌌 Sakaloka-Universe — libs/core
// Iron Curtain directives — mandatory on every lib.rs
// ============================================================
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! # sakaloka-core
//!
//! Shared domain types, traits, and constants for the entire
//! Sakaloka-Universe. This crate is the single source of truth
//! for all domain primitives shared across planets.
//!
//! ## Design rules
//! - No business logic lives here — only types and traits.
//! - No async runtime dependency — this crate must be `no_std` compatible
//!   in the future for Mercury (RTIC firmware) via feature flags.

pub mod constants;
pub mod models;
pub mod traits;
pub mod types;
