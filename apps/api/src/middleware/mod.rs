//! Custom middleware for authentication, logging, and claim extraction.

pub mod auth;

pub use auth::auth_middleware;
