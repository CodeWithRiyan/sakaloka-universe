//! SurrealDB (Jupiter) client abstraction.
//!
//! Always use the authenticated client from this module.
//! Never embed raw SurrealDB credentials in application code.

use thiserror::Error;

/// Errors produced by the SurrealDB client layer.
#[derive(Debug, Error)]
pub enum SurrealError {
    /// Connection to SurrealDB failed.
    #[error("SurrealDB connection failed: {0}")]
    Connection(String),
    /// A query execution error.
    #[error("SurrealDB query error: {0}")]
    Query(String),
}
