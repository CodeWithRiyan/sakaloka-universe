//! Argon2id password hashing and verification.
//!
//! This is the **only** module allowed to import `argon2`. No other
//! crate in the workspace may perform password hashing.

use crate::error::SecureError;
use crate::newtypes::Password;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Hashes a [`Password`] using Argon2id with a random salt.
///
/// The resulting PHC string can be stored directly in the database.
/// **Never** store the raw password.
///
/// # Errors
///
/// Returns [`SecureError::HashFailed`] if the Argon2 algorithm encounters
/// an unexpected error (e.g. internal parameter issue).
///
/// # Examples
///
/// ```rust,no_run
/// use sakaloka_secure::{argon2::hash_password, newtypes::Password};
///
/// let pw = Password::new("correct-horse-battery-staple").unwrap();
/// let hash = hash_password(&pw).unwrap();
/// assert!(hash.starts_with("$argon2id$"));
/// ```
pub fn hash_password(password: &Password) -> Result<String, SecureError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_str().as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| SecureError::HashFailed(e.to_string()))
}

/// Verifies a plaintext [`Password`] against a stored Argon2id PHC hash.
///
/// This comparison is constant-time to prevent timing attacks.
///
/// # Errors
///
/// Returns [`SecureError::VerifyFailed`] if the password does not match
/// or the hash string is malformed.
///
/// # Examples
///
/// ```rust,no_run
/// use sakaloka_secure::{argon2::{hash_password, verify_password}, newtypes::Password};
///
/// let pw = Password::new("correct-horse-battery-staple").unwrap();
/// let hash = hash_password(&pw).unwrap();
/// let ok = verify_password(&pw, &hash).unwrap();
/// assert!(ok);
/// ```
pub fn verify_password(password: &Password, hash: &str) -> Result<bool, SecureError> {
    let parsed = PasswordHash::new(hash).map_err(|_| SecureError::VerifyFailed)?;
    let ok = Argon2::default()
        .verify_password(password.as_str().as_bytes(), &parsed)
        .is_ok();
    Ok(ok)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_roundtrip() {
        let pw = Password::new("secure-sakaloka-password").unwrap();
        let hash = hash_password(&pw).unwrap();
        
        // Assert it starts with the correct PHC prefix
        assert!(hash.starts_with("$argon2id$"));
        
        // Assert it verifies correctly
        let ok = verify_password(&pw, &hash).unwrap();
        assert!(ok);
    }

    #[test]
    fn test_verify_rejects_wrong_password() {
        let pw = Password::new("correct-password").unwrap();
        let wrong_pw = Password::new("wrong-password").unwrap();
        
        let hash = hash_password(&pw).unwrap();
        
        let ok = verify_password(&wrong_pw, &hash).unwrap();
        assert!(!ok);
    }

    #[test]
    fn test_verify_rejects_malformed_hash() {
        let pw = Password::new("correct-password").unwrap();
        let result = verify_password(&pw, "not-a-hash");
        assert!(matches!(result, Err(SecureError::VerifyFailed)));
    }
}
