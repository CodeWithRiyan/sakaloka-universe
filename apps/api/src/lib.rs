#![doc = "Sakaloka-Universe — Earth: Axum REST API server library."]
#![deny(clippy::all)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// Application state and router construction.
pub mod app;
/// HTTP controllers (route handlers).
pub mod controllers;
/// OpenAPI document definition.
pub mod docs;
/// Unified API error type.
pub mod error;
/// Shared helpers used across controllers.
pub mod helpers;
/// Authentication and authorization middleware.
pub mod middleware;
/// Response DTOs and view helpers.
pub mod views;
