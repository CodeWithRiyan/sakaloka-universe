// ============================================================
// 🌌 Sakaloka-Universe — libs/secure
// Iron Curtain directives — mandatory on every lib.rs
// ============================================================
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! # sakaloka-secure
//!
//! **Single source of truth for all Identity & Access Management (IAM)**
//! in the Sakaloka-Universe.
//!
//! ## What lives here
//! - Argon2id password hashing and verification
//! - JWT signing and validation (User JWTs + Service JWTs)
//! - RBAC roles and scope enforcement middleware
//! - **Permission normaliser** — converts DB `role.permissions` JSON to canonical
//!   JWT scope strings (`rbac::permission`)
//! - Service token store with proactive cache + rotation
//! - Scoped Qdrant API key vault
//! - Newtype wrappers for validated user inputs
//!
//! ## What NEVER lives here
//! This crate must never depend on HTTP frameworks or route definitions.
//! IAM policy is enforced at the framework layer in `apps/api`.
//!
//! ## Iron Curtain
//! Zero `.unwrap()`, zero `.expect()`, zero `unsafe`. All errors
//! propagate via `Result<T, SecureError>`.

pub mod argon2;
pub mod error;
pub mod jwt;
pub mod keys;
pub mod newtypes;
pub mod rbac;
pub mod tokens;

pub use crate::newtypes::{
    EmailAddress, Password, Planet, ServiceName, SessionId, TokenId, UserId, Username,
};
