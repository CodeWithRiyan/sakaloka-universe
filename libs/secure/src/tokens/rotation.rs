//! Refresh token rotation and JTI blocklist management.

use crate::error::SecureError;

/// Checks whether a given JTI (token ID) has been blocklisted.
///
/// In a full implementation this queries the `jti_blocklist` SurrealDB table.
/// Returns `Ok(false)` in the scaffold — Sprint 3 wires the real DB check.
///
/// # Errors
///
/// Returns [`SecureError::TokenReused`] if the JTI is on the blocklist.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::tokens::rotation::check_jti_blocklist;
///
/// // In the scaffold, no JTI is blocklisted yet.
/// let result = check_jti_blocklist("some-jti-uuid");
/// assert!(result.is_ok());
/// ```
pub fn check_jti_blocklist(jti: &str) -> Result<(), SecureError> {
    // TODO(Sprint 3): query `jti_blocklist` SurrealDB table.
    // For scaffolding, no JTIs are blocked yet.
    let _ = jti;
    Ok(())
}
