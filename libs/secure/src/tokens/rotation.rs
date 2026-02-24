//! Refresh token rotation and JTI blocklist management.

use crate::error::SecureError;
use std::future::Future;

/// Database abstraction for managing the JTI blocklist.
/// Implemented by the data crate.
pub trait JtiStore: Send + Sync {
    /// Checks if a JTI is on the blocklist.
    fn is_jti_blocked(
        &self,
        jti: &str,
    ) -> impl Future<Output = Result<bool, SecureError>> + Send;

    /// Adds a JTI to the blocklist.
    fn block_jti(
        &self,
        jti: &str,
        exp: u64,
    ) -> impl Future<Output = Result<(), SecureError>> + Send;
}

/// Checks whether a given JTI (token ID) has been blocklisted via the injected store.
///
/// # Errors
///
/// Returns [`SecureError::TokenReused`] if the JTI is on the blocklist.
pub async fn check_jti_blocklist<S: JtiStore>(store: &S, jti: &str) -> Result<(), SecureError> {
    if store.is_jti_blocked(jti).await? {
        return Err(SecureError::TokenReused);
    }
    Ok(())
}

