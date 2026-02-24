//! Service token store with proactive cache and rotation.

use crate::error::SecureError;
use crate::jwt::{service_claims::issue_service_token, JwtKeys};
use crate::newtypes::Planet;
use sakaloka_core::constants::SERVICE_TOKEN_TTL_SECS;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

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
    issuing_planet: Planet,
    keys: JwtKeys,
    cache: RwLock<HashMap<String, CachedToken>>,
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
        let keys = JwtKeys::from_env()?;
        Ok(Self {
            issuing_planet,
            keys,
            cache: RwLock::new(HashMap::new()),
        })
    }

    /// Gets a service token for the target planet, using the cache if valid.
    /// Refreshes the token automatically if it expires within 60 seconds.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::JwtEncode`] if signing fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sakaloka_secure::{tokens::store::ServiceTokenStore, newtypes::Planet};
    ///
    /// std::env::set_var("SAKALOKA_JWT_SECRET", "test-secret-at-least-32-chars-long!");
    /// let earth = Planet::new("earth").unwrap();
    /// let jupiter = Planet::new("jupiter").unwrap();
    /// let store = ServiceTokenStore::new(earth).unwrap();
    /// let token = store.get_token(&jupiter, &["db:read"]).unwrap();
    /// ```
    pub fn get_token(
        &self,
        target_planet: &Planet,
        scopes: &[&str],
    ) -> Result<String, SecureError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // 1. Check if token is natively valid in cache
        {
            let cache = self.cache.read().map_err(|e| SecureError::SyncPoisoned(e.to_string()))?;
            if let Some(cached) = cache.get(target_planet.as_str()) {
                if cached.expires_at > now + 60 {
                    return Ok(cached.token.clone());
                }
            }
        }

        // 2. Lock for writing and re-check cache (double-checked locking)
        let mut cache = self.cache.write().map_err(|e| SecureError::SyncPoisoned(e.to_string()))?;
        if let Some(cached) = cache.get(target_planet.as_str()) {
            if cached.expires_at > now + 60 {
                return Ok(cached.token.clone());
            }
        }

        // 3. Issue a new token
        let token = issue_service_token(&self.keys, &self.issuing_planet, target_planet, scopes)?;
        let expires_at = now + SERVICE_TOKEN_TTL_SECS;

        cache.insert(
            target_planet.to_string(),
            CachedToken {
                token: token.clone(),
                expires_at,
            },
        );

        Ok(token)
    }
}
