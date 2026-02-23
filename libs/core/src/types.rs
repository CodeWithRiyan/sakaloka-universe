//! Shared domain types for Sakaloka-Universe.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A planet identifier used in JWT audience and Zenoh topic routing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlanetId(String);

impl PlanetId {
    /// Creates a new [`PlanetId`] from a string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_core::types::PlanetId;
    ///
    /// let id = PlanetId::new("earth");
    /// assert_eq!(id.as_str(), "earth");
    /// ```
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    /// Returns the string representation of this planet identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PlanetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A correlation/request ID used for distributed tracing across planets.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestId(Uuid);

impl RequestId {
    /// Generates a new random [`RequestId`] using UUID v4.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_core::types::RequestId;
    ///
    /// let id = RequestId::new();
    /// // Each call produces a unique ID.
    /// assert_ne!(id, RequestId::new());
    /// ```
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
