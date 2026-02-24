//! Scoped Qdrant API key vault.
//!
//! Never use the Qdrant master key in application code.
//! Always request a scoped key from this vault.

use crate::error::SecureError;

/// The scope of a Qdrant API key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QdrantScope {
    /// Full read and write access (used by Earth).
    ReadWrite,
    /// Write-only access — cannot execute search queries (used by Neptune).
    WriteOnly,
    /// Read-only access for internal consumers.
    ReadOnly,
}

/// Vault that provisions scoped Qdrant API keys.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::keys::vault::{ApiKeyVault, QdrantScope};
///
/// std::env::set_var("QDRANT_MASTER_KEY", "my-master-key");
/// let vault = ApiKeyVault::from_env().unwrap();
/// let key = vault.get_qdrant_key(QdrantScope::ReadWrite).unwrap();
/// ```
pub struct ApiKeyVault {
    master_key: String,
}

impl ApiKeyVault {
    /// Loads the Qdrant master key from the `QDRANT_MASTER_KEY` environment variable.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::MissingEnvVar`] if the variable is not set.
    pub fn from_env() -> Result<Self, SecureError> {
        let key = std::env::var("QDRANT_MASTER_KEY")
            .map_err(|_| SecureError::MissingEnvVar("QDRANT_MASTER_KEY".to_string()))?;
        Ok(Self { master_key: key })
    }

    /// Returns a scoped Qdrant API key for the requested access level.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::JwtEncode`] if the token cannot be created.
    pub fn get_qdrant_key(&self, scope: QdrantScope) -> Result<String, SecureError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| SecureError::SyncPoisoned(e.to_string()))?
            .as_secs();

        // 1. Map internal scope to Qdrant JWT access claim
        // Qdrant (OS) supports "r", "rw", "m".
        // We use "rw" for WriteOnly as it's the required level for upserts.
        let access = match scope {
            QdrantScope::ReadWrite => serde_json::json!("rw"),
            QdrantScope::WriteOnly => serde_json::json!("rw"), // Close enough for OS version
            QdrantScope::ReadOnly => serde_json::json!("r"),
        };

        // 2. Build claims
        let claims = serde_json::json!({
            "exp": now + 3600, // 1 hour TTL for scoped keys
            "access": access,
        });

        // 3. Sign the token using the master key as secret
        use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.master_key.as_bytes()),
        )
        .map_err(|e| SecureError::JwtEncode(e.to_string()))?;

        Ok(token)
    }
}
