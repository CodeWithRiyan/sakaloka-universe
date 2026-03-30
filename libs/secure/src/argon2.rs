//! Argon2id password hashing and verification.
//!
//! This is the **only** module allowed to import `argon2`. No other
//! crate in the workspace may perform password hashing.
//!
//! Both [`hash_password`] and [`verify_password`] run the CPU-intensive
//! Argon2id algorithm on Tokio's blocking thread-pool via
//! [`tokio::task::spawn_blocking`], so calling them from an async
//! handler will **not** starve the executor.

use crate::error::SecureError;
use crate::newtypes::Password;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};

/// Tuned Argon2id parameters for VPS workloads.
///
/// These match the seed hash in the migration files:
/// `m = 19 456 KiB` (~19 MB), `t = 2` iterations, `p = 1` lane.
///
/// The defaults shipped by the `argon2` crate (`m = 65 536`, `t = 3`)
/// are designed for high-end servers and take ~1–2 s *per call* on a
/// small VPS, blocking the async executor the entire time.
fn argon2_instance() -> Argon2<'static> {
    // SAFETY: These are hardcoded constants known to be valid at compile time.
    // Argon2 spec requires m >= 8*p (19456 >= 8), t >= 1, p >= 1.
    #[allow(clippy::expect_used)]
    let params = Params::new(19_456, 2, 1, None).expect("hardcoded Argon2 params are valid");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// Hashes a [`Password`] using Argon2id with a random salt.
///
/// The computation runs on Tokio's blocking thread-pool so it never
/// blocks the async executor.
///
/// # Errors
///
/// Returns [`SecureError::HashFailed`] if the Argon2 algorithm encounters
/// an unexpected error, or if the blocking task panics.
///
/// # Examples
///
/// ```rust
/// # tokio_test::block_on(async {
/// use sakaloka_secure::{argon2::hash_password, newtypes::Password};
///
/// let pw = Password::new("correct-horse-battery-staple").unwrap();
/// let hash = hash_password(&pw).await.unwrap();
/// assert!(hash.starts_with("$argon2id$"));
/// # });
/// ```
pub async fn hash_password(password: &Password) -> Result<String, SecureError> {
    let pw_bytes = password.as_str().to_owned();
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = argon2_instance();
        argon2
            .hash_password(pw_bytes.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| SecureError::HashFailed(e.to_string()))
    })
    .await
    .map_err(|e| SecureError::HashFailed(format!("blocking task failed: {e}")))?
}

/// Verifies a plaintext [`Password`] against a stored Argon2id PHC hash.
///
/// The computation runs on Tokio's blocking thread-pool so it never
/// blocks the async executor. The comparison is constant-time to prevent
/// timing attacks.
///
/// # Errors
///
/// Returns [`SecureError::VerifyFailed`] if the password does not match
/// or the hash string is malformed.
///
/// # Examples
///
/// ```rust
/// # tokio_test::block_on(async {
/// use sakaloka_secure::{argon2::{hash_password, verify_password}, newtypes::Password};
///
/// let pw = Password::new("correct-horse-battery-staple").unwrap();
/// let hash = hash_password(&pw).await.unwrap();
/// let ok = verify_password(&pw, &hash).await.unwrap();
/// assert!(ok);
/// # });
/// ```
pub async fn verify_password(password: &Password, hash: &str) -> Result<bool, SecureError> {
    let pw_bytes = password.as_str().to_owned();
    let hash_owned = hash.to_owned();
    tokio::task::spawn_blocking(move || {
        let parsed = PasswordHash::new(&hash_owned).map_err(|_| SecureError::VerifyFailed)?;
        let ok = argon2_instance()
            .verify_password(pw_bytes.as_bytes(), &parsed)
            .is_ok();
        Ok(ok)
    })
    .await
    .map_err(|_| SecureError::VerifyFailed)?
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_and_verify_roundtrip() {
        let pw = Password::new("secure-sakaloka-password").unwrap();
        let hash = hash_password(&pw).await.unwrap();

        // Assert it starts with the correct PHC prefix
        assert!(hash.starts_with("$argon2id$"));

        // Assert it verifies correctly
        let ok = verify_password(&pw, &hash).await.unwrap();
        assert!(ok);
    }

    #[tokio::test]
    async fn test_verify_rejects_wrong_password() {
        let pw = Password::new("correct-password").unwrap();
        let wrong_pw = Password::new("wrong-password").unwrap();

        let hash = hash_password(&pw).await.unwrap();

        let ok = verify_password(&wrong_pw, &hash).await.unwrap();
        assert!(!ok);
    }

    #[tokio::test]
    async fn test_verify_rejects_malformed_hash() {
        let pw = Password::new("correct-password").unwrap();
        let result = verify_password(&pw, "not-a-hash").await;
        assert!(matches!(result, Err(SecureError::VerifyFailed)));
    }
}
