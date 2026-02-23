//! AI error types for sakaloka-ai.

use thiserror::Error;

/// All errors produced by the `sakaloka-ai` crate.
#[derive(Debug, Error)]
pub enum AiError {
    /// The model file could not be found at the specified path.
    #[error("model file not found at `{0}`")]
    ModelNotLoaded(String),
    /// The forward inference pass failed.
    #[error("inference failed: {0}")]
    InferenceFailed(String),
    /// The input text was empty or too long.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
