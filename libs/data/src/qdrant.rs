//! Qdrant (Uranus) vector search client abstraction.
//!
//! Always request scoped keys from `sakaloka_secure::keys::vault::ApiKeyVault`.
//! Never use the Qdrant master API key in application code.

use thiserror::Error;

/// Errors produced by the Qdrant client layer.
#[derive(Debug, Error)]
pub enum QdrantError {
    /// Connection to Qdrant failed.
    #[error("Qdrant connection failed: {0}")]
    Connection(String),
    /// A vector upsert operation failed.
    #[error("Qdrant upsert error: {0}")]
    Upsert(String),
    /// A search query failed.
    #[error("Qdrant search error: {0}")]
    Search(String),
}
