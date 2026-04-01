//! HTTP controllers (route handlers) for the Sakaloka API.
//!
//! Controller boundary standard:
//!
//! - `routes.rs` owns HTTP path registration, method binding, and route-level
//!   guards such as [`sakaloka_secure::rbac::guard::RequireScope`].
//! - `handlers.rs` owns HTTP request parsing, response shaping, and orchestration
//!   of application services / data access.
//! - `helpers.rs` is reserved for controller-local glue that should not leak as
//!   shared cross-domain behavior.
//!
//! Shared behavior that is reused across domains belongs in:
//!
//! - [`crate::helpers`] for API-layer helpers
//! - `libs/*` for domain, data, or security concerns
//!
//! This keeps the modular-monolith boundary predictable as the API grows.

pub mod auth;
pub mod bom;
pub mod brand;
pub mod category;
pub mod organization;
pub mod pos;
pub mod product;
pub mod role;
pub mod stock;
pub mod user;
