//! Newtype wrappers for validated user inputs.
//!
//! Raw `String` and `&str` values **must never** enter business logic or
//! database queries directly. Every external input is wrapped in a validated
//! newtype at the boundary. Construction always returns `Result<T, SecureError>`.

use crate::error::SecureError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A validated user identifier (SurrealDB record ID format: `user:<ulid>`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    /// Creates a new [`UserId`] from a raw string.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::InvalidUsername`] if the string is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::UserId;
    ///
    /// let id = UserId::new("user:01JKXYZ").unwrap();
    /// assert_eq!(id.as_str(), "user:01JKXYZ");
    /// ```
    pub fn new(raw: &str) -> Result<Self, SecureError> {
        if raw.is_empty() {
            return Err(SecureError::InvalidUsername(
                "user ID cannot be empty".to_string(),
            ));
        }
        Ok(Self(raw.to_string()))
    }

    /// Returns the inner string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A unique token identifier (JWT `jti` claim), used to prevent replay attacks.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenId(Uuid);

impl TokenId {
    /// Generates a new random [`TokenId`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::TokenId;
    ///
    /// let id = TokenId::new();
    /// assert_ne!(id, TokenId::new());
    /// ```
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the UUID value.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TokenId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TokenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A unique session identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    /// Generates a new random [`SessionId`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::SessionId;
    ///
    /// let id = SessionId::new();
    /// assert_ne!(id, SessionId::new());
    /// ```
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a [`SessionId`] from a raw string (e.g. from SurrealDB record ID).
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::InvalidInput`] if the raw value is not a valid UUID.
    pub fn new_with_raw(raw: &str) -> Result<Self, SecureError> {
        Uuid::parse_str(raw.trim_start_matches("session:"))
            .map(Self)
            .map_err(|_| SecureError::InvalidInput(format!("Invalid session ID: {raw}")))
    }

    /// Returns the SurrealDB-prefixed session ID string.
    ///
    /// Note: this allocates a new `String` on every call.
    pub fn to_record_id_string(&self) -> String {
        format!("session:{}", self.0)
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A validated planet name (e.g. `"earth"`, `"jupiter"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Planet(String);

impl Planet {
    /// Creates a new [`Planet`] identifier.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::InvalidUsername`] if the string is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::Planet;
    ///
    /// let p = Planet::new("earth").unwrap();
    /// assert_eq!(p.audience(), "sakaloka:earth");
    /// ```
    pub fn new(name: &str) -> Result<Self, SecureError> {
        if name.is_empty() {
            return Err(SecureError::InvalidUsername(
                "planet name cannot be empty".to_string(),
            ));
        }
        Ok(Self(name.to_lowercase()))
    }

    /// Returns the JWT audience string for this planet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::Planet;
    ///
    /// let p = Planet::new("jupiter").unwrap();
    /// assert_eq!(p.audience(), "sakaloka:jupiter");
    /// ```
    pub fn audience(&self) -> String {
        format!("sakaloka:{}", self.0)
    }

    /// Returns the raw planet name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A validated service name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceName(String);

impl ServiceName {
    /// Creates a new validated [`ServiceName`].
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::InvalidUsername`] if the string is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::ServiceName;
    ///
    /// let sn = ServiceName::new("sakaloka-api").unwrap();
    /// assert_eq!(sn.as_str(), "sakaloka-api");
    /// ```
    pub fn new(name: &str) -> Result<Self, SecureError> {
        if name.is_empty() {
            return Err(SecureError::InvalidUsername(
                "service name cannot be empty".to_string(),
            ));
        }
        Ok(Self(name.to_string()))
    }

    /// Returns the raw service name string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ServiceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A validated email address.
///
/// Stores the email in lowercase. Validation is a basic format check —
/// email deliverability verification is out of scope for this crate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Parses and validates an email address string.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::InvalidEmail`] if the string does not contain
    /// exactly one `@` with non-empty local and domain parts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::EmailAddress;
    ///
    /// let email = EmailAddress::new("User@Example.COM").unwrap();
    /// assert_eq!(email.as_str(), "user@example.com");
    /// ```
    pub fn new(raw: &str) -> Result<Self, SecureError> {
        let lower = raw.trim().to_lowercase();
        let parts: Vec<&str> = lower.splitn(2, '@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() || !parts[1].contains('.')
        {
            return Err(SecureError::InvalidEmail(raw.to_string()));
        }
        Ok(Self(lower))
    }

    /// Returns the normalized (lowercase) email string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A validated username (3–64 chars, alphanumeric + underscore/hyphen).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    /// Creates a validated [`Username`].
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::InvalidUsername`] if the username is shorter than
    /// 3 characters, longer than 64 characters, or contains invalid characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::Username;
    ///
    /// let u = Username::new("riyan_dev").unwrap();
    /// assert_eq!(u.as_str(), "riyan_dev");
    /// ```
    pub fn new(raw: &str) -> Result<Self, SecureError> {
        let trimmed = raw.trim();
        if trimmed.len() < 3 {
            return Err(SecureError::InvalidUsername(
                "username must be at least 3 characters".to_string(),
            ));
        }
        if trimmed.len() > 64 {
            return Err(SecureError::InvalidUsername(
                "username must be at most 64 characters".to_string(),
            ));
        }
        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(SecureError::InvalidUsername(
                "username may only contain letters, digits, _ and -".to_string(),
            ));
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the validated username string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A raw, unverified password string — cleared on drop.
///
/// This type holds the plaintext only long enough to hash it.
/// It is **never** stored, serialized, or logged.
#[derive(Debug)]
pub struct Password(String);

impl Password {
    /// Creates a [`Password`] from raw input, enforcing minimum complexity.
    ///
    /// # Errors
    ///
    /// Returns [`SecureError::InvalidPassword`] if the password is shorter
    /// than 12 characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::newtypes::Password;
    ///
    /// let pw = Password::new("correct-horse-battery-staple").unwrap();
    /// assert_eq!(pw.as_str().len(), 28);
    /// ```
    pub fn new(raw: &str) -> Result<Self, SecureError> {
        if raw.len() < 12 {
            return Err(SecureError::InvalidPassword(
                "password must be at least 12 characters".to_string(),
            ));
        }
        Ok(Self(raw.to_string()))
    }

    /// Exposes the raw password string for hashing only.
    ///
    /// This must only be called by `libs/secure::argon2` — nowhere else.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Drop for Password {
    fn drop(&mut self) {
        // Zero out the password string in memory before dropping.
        // Best-effort: Rust's allocator may have already moved the backing buffer.
        self.0.clear();
    }
}
