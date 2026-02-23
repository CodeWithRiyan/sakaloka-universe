// ============================================================
// 🌌 Sakaloka-Universe — libs/ai
// Iron Curtain directives — mandatory on every lib.rs
// ============================================================
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! # sakaloka-ai
//!
//! Local Burn AI inference engine for Sakaloka-Universe.
//!
//! ## Planet: 🔮 Neptune
//! Neptune has **no HTTP surface**. It communicates exclusively via
//! Eclipse Zenoh (Saturn). This crate provides the embedding pipeline
//! used by Neptune.
//!
//! ## Embedding pipeline
//! Jupiter record saved → Zenoh event → Neptune embeds
//! → Zenoh event → Uranus upserts vector
//!
//! ## Rules
//! - Model is loaded from a local file path — never downloaded at runtime.
//! - `embed()` always returns `Result<Vec<f32>, AiError>`.
//! - No HTTP listener — zero network exposure for this crate.

pub mod embed;
pub mod error;
pub mod model;
