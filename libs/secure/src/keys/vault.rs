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
/// ```rust,no_run
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
    /// Returns [`SecureError::MissingEnvVar`] if the vault was not properly initialized.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sakaloka_secure::keys::vault::{ApiKeyVault, QdrantScope};
    ///
    /// std::env::set_var("QDRANT_MASTER_KEY", "my-master-key");
    /// let vault = ApiKeyVault::from_env().unwrap();
    /// // Neptune only gets write-only access.
    /// let neptune_key = vault.get_qdrant_key(QdrantScope::WriteOnly).unwrap();
    /// ```
    pub fn get_qdrant_key(&self, scope: QdrantScope) -> Result<String, SecureError> {
        // TODO(Sprint 4b): provision real scoped keys via Qdrant REST API.
        // For scaffolding, scope is encoded as a prefix on the master key.
        let prefix = match scope {
            QdrantScope::ReadWrite => "rw",
            QdrantScope::WriteOnly => "wo",
            QdrantScope::ReadOnly => "ro",
        };
        Ok(format!("{}:{}", prefix, self.master_key))
    }
}
