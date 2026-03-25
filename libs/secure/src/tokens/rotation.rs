//! Refresh token rotation and JTI blocklist management.

use crate::error::SecureError;
use std::future::Future;

/// SHA-256 hash a raw refresh token for safe storage.
///
/// # Examples
///
/// ```rust
/// let hash = sakaloka_secure::tokens::rotation::hash_refresh_token("my-token");
/// assert_eq!(hash.len(), 64); // hex-encoded SHA-256
/// ```
pub fn hash_refresh_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Database abstraction for managing the JTI blocklist.
/// Implemented by the data crate.
pub trait JtiStore: Send + Sync {
    /// Checks if a JTI is on the blocklist.
    fn is_jti_blocked(&self, jti: &str) -> impl Future<Output = Result<bool, SecureError>> + Send;

    /// Adds a JTI to the blocklist.
    fn block_jti(
        &self,
        jti: &str,
        exp: u64,
    ) -> impl Future<Output = Result<(), SecureError>> + Send;
}

/// Metadata for a refresh token captured from the database.
#[derive(Debug, Clone)]
pub struct RefreshTokenRecord {
    /// Associated session ID.
    pub session_id: crate::newtypes::SessionId,
    /// Unix timestamp when this token expires.
    pub expires_at: u64,
    /// Unix timestamp when this token was rotated, if any.
    pub rotated_at: Option<u64>,
}

/// Database abstraction for managing refresh token rotation.
pub trait RefreshStore: Send + Sync {
    /// Looks up a refresh token record by its hash.
    fn find_token(
        &self,
        hash: &str,
    ) -> impl Future<Output = Result<Option<RefreshTokenRecord>, SecureError>> + Send;

    /// Marks a token as rotated and inserts a new one atomically.
    /// Also handles session termination on reuse detection.
    fn rotate_token(
        &self,
        old_hash: &str,
        new_hash: &str,
        session_id: &crate::newtypes::SessionId,
        expires_at: u64,
    ) -> impl Future<Output = Result<(), SecureError>> + Send;

    /// Terminates an entire session due to reuse detection.
    fn terminate_session(
        &self,
        session_id: &crate::newtypes::SessionId,
    ) -> impl Future<Output = Result<(), SecureError>> + Send;
}

/// Checks whether a given JTI (token ID) has been blocklisted via the injected store.
pub async fn check_jti_blocklist<S: JtiStore>(store: &S, jti: &str) -> Result<(), SecureError> {
    if store.is_jti_blocked(jti).await? {
        return Err(SecureError::TokenReused);
    }
    Ok(())
}

/// Rotates a refresh token, enforcing reuse detection and session termination.
///
/// # Errors
/// - [`SecureError::TokenExpired`] if the token is past its expiry.
/// - [`SecureError::TokenReused`] if the token has already been rotated.
/// - [`SecureError::JwtDecode`] if the token format is invalid.
pub async fn rotate_refresh_token<S: RefreshStore>(
    store: &S,
    old_token: &str,
) -> Result<(String, RefreshTokenRecord), SecureError> {
    // 1. Hash the incoming token (SHA256 is sufficient for high-entropy tokens)
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(old_token.as_bytes());
    let old_hash = format!("{:x}", hasher.finalize());

    // 2. Lookup record
    let record = store
        .find_token(&old_hash)
        .await?
        .ok_or_else(|| SecureError::JwtDecode("Invalid refresh token".to_string()))?;

    // 3. Check for reuse
    if record.rotated_at.is_some() {
        // REUSE DETECTED! Terminate the entire session.
        store.terminate_session(&record.session_id).await?;
        return Err(SecureError::TokenReused);
    }

    // 4. Check expiry
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| SecureError::SyncPoisoned(e.to_string()))?
        .as_secs();

    if record.expires_at < now {
        return Err(SecureError::TokenExpired);
    }

    // 5. Generate new token and hash it
    let new_token = crate::newtypes::TokenId::new().to_string();
    let mut hasher = Sha256::new();
    hasher.update(new_token.as_bytes());
    let new_hash = format!("{:x}", hasher.finalize());

    // 6. Perform the rotation in the store
    // Default TTL for refresh token is 7 days (604800 secs)
    let new_expiry = now + 604_800;

    store
        .rotate_token(&old_hash, &new_hash, &record.session_id, new_expiry)
        .await?;

    Ok((new_token, record))
}
