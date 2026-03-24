//! Custom middleware for authentication, logging, and claim extraction.

/// JWT authentication middleware.
pub mod auth;

pub use auth::auth_middleware;
