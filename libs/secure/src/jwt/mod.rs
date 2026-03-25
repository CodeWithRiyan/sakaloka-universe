//! JWT signing and validation for User JWTs and Service JWTs.
//!
//! This module is the **only** place allowed to call
//! `jsonwebtoken::encode` / `jsonwebtoken::decode` in the entire workspace.

pub mod service_claims;
pub mod user_claims;

use crate::error::SecureError;
use jsonwebtoken::{DecodingKey, EncodingKey};

/// Loaded JWT keys derived from the `SAKALOKA_JWT_SECRET` environment variable.
///
/// Both User JWTs and Service JWTs are signed with the same HS256 secret.
/// The `aud` claim differentiates tokens for each planet.
pub struct JwtKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtKeys {
    /// Loads the JWT secret from the `SAKALOKA_JWT_SECRET` environment variable.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::MissingEnvVar`] if the variable is not set or empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::jwt::JwtKeys;
    ///
    /// std::env::set_var("SAKALOKA_JWT_SECRET", "test-secret-at-least-32-chars-long!");
    /// let keys = JwtKeys::from_env().unwrap();
    /// ```
    pub fn from_env() -> Result<Self, SecureError> {
        let secret = std::env::var("SAKALOKA_JWT_SECRET")
            .map_err(|_| SecureError::MissingEnvVar("SAKALOKA_JWT_SECRET".to_string()))?;
        if secret.is_empty() {
            return Err(SecureError::MissingEnvVar(
                "SAKALOKA_JWT_SECRET".to_string(),
            ));
        }
        Ok(Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
        })
    }

    /// Returns a reference to the encoding key.
    pub(crate) fn encoding(&self) -> &EncodingKey {
        &self.encoding
    }

    /// Returns a reference to the decoding key.
    pub(crate) fn decoding(&self) -> &DecodingKey {
        &self.decoding
    }
}

/// Returns the current UNIX timestamp in seconds.
///
/// Falls back to 0 if `SystemTime` fails (should not occur on real hardware).
pub(crate) fn current_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
