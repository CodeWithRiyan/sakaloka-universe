#![doc = "Sakaloka-Universe — Earth: Loco.rs REST API server library."]
#![deny(clippy::all)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// Loco application hooks.
pub mod app;
/// HTTP controllers (route handlers).
pub mod controllers;
/// Authentication and authorization middleware.
pub mod middleware;
/// SeaORM entity models.
pub mod models;
/// Response DTOs and view helpers.
pub mod views;
