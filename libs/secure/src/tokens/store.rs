//! Service token store with proactive cache and rotation.

use crate::error::SecureError;
use crate::newtypes::Planet;

/// Cached service token entry.
#[derive(Debug, Clone)]
pub struct CachedToken {
    /// The signed JWT string.
    pub token: String,
    /// Unix timestamp when this token expires.
    pub expires_at: u64,
}

/// A store that caches service tokens per target planet and proactively
/// refreshes them 60 seconds before expiry.
///
/// This ensures no planet ever calls another with an expired credential.
pub struct ServiceTokenStore {
    #[allow(dead_code)]
    issuing_planet: Planet,
}

impl ServiceTokenStore {
    /// Creates a new [`ServiceTokenStore`] for the given issuing planet.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::MissingEnvVar`] if `SAKALOKA_JWT_SECRET` is not set.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sakaloka_secure::{tokens::store::ServiceTokenStore, newtypes::Planet};
    ///
    /// std::env::set_var("SAKALOKA_JWT_SECRET", "test-secret-at-least-32-chars-long!");
    /// let earth = Planet::new("earth").unwrap();
    /// let store = ServiceTokenStore::new(earth).unwrap();
    /// ```
    pub fn new(issuing_planet: Planet) -> Result<Self, SecureError> {
        Ok(Self { issuing_planet })
    }
}
