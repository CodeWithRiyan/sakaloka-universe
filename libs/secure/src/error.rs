//! Sakaloka error type for all IAM operations.

use thiserror::Error;

/// All errors produced by `sakaloka-secure`.
///
/// Every function in this crate returns `Result<T, SecureError>` so that
/// callers can pattern-match on failure without `.unwrap()` or `.expect()`.
#[derive(Debug, Error)]
pub enum SecureError {
    /// Password hashing failed.
    #[error("password hashing failed: {0}")]
    HashFailed(String),

    /// Password verification failed (wrong password or corrupted hash).
    #[error("password verification failed")]
    VerifyFailed,

    /// A required environment variable is missing.
    #[error("missing environment variable `{0}`")]
    MissingEnvVar(String),

    /// JWT encoding failed.
    #[error("JWT encoding failed: {0}")]
    JwtEncode(String),

    /// JWT decoding or validation failed.
    #[error("JWT validation failed: {0}")]
    JwtDecode(String),

    /// The `aud` claim does not match the expected target planet.
    #[error("audience mismatch: expected `{expected}`, got `{got}`")]
    AudienceMismatch {
        /// The expected audience value.
        expected: String,
        /// The actual audience value found in the token.
        got: String,
    },

    /// The token has expired.
    #[error("token expired")]
    TokenExpired,

    /// A refresh token was reused after rotation — session terminated.
    #[error("token reused: session terminated")]
    TokenReused,

    /// An email address string failed format validation.
    #[error("invalid email address: `{0}`")]
    InvalidEmail(String),

    /// A username string failed validation rules.
    #[error("invalid username: `{0}`")]
    InvalidUsername(String),

    /// A password string failed complexity requirements.
    #[error("invalid password: {0}")]
    InvalidPassword(String),

    /// A lock inside token store was poisoned.
    #[error("synchronization lock poisoned: {0}")]
    SyncPoisoned(String),

    /// A generic input value failed validation.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
