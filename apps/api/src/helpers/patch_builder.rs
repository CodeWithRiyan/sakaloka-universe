//! Builder for partial-update JSON patch objects.
//!
//! Replaces repetitive `if let Some(x) { map.insert(...) }` blocks in update
//! handlers with a fluent API.

/// A builder that accumulates optional field updates into a
/// [`serde_json::Value::Object`].
///
/// # Examples
///
/// ```rust,ignore
/// let patch = PatchBuilder::new()
///     .set_string("name", payload.name)
///     .set_string("email", payload.email)
///     .set_bool("is_active", payload.is_active)
///     .build();
/// ```
pub struct PatchBuilder {
    map: serde_json::Map<String, serde_json::Value>,
}

impl PatchBuilder {
    /// Create a new, empty patch builder.
    pub fn new() -> Self {
        Self {
            map: serde_json::Map::new(),
        }
    }

    /// Insert a string field if `value` is `Some`.
    pub fn set_string(mut self, key: &str, value: Option<String>) -> Self {
        if let Some(v) = value {
            self.map
                .insert(key.to_string(), serde_json::Value::String(v));
        }
        self
    }

    /// Insert an `i64` field if `value` is `Some`.
    pub fn set_i64(mut self, key: &str, value: Option<i64>) -> Self {
        if let Some(v) = value {
            self.map
                .insert(key.to_string(), serde_json::Value::Number(v.into()));
        }
        self
    }

    /// Insert a boolean field if `value` is `Some`.
    pub fn set_bool(mut self, key: &str, value: Option<bool>) -> Self {
        if let Some(v) = value {
            self.map.insert(key.to_string(), serde_json::Value::Bool(v));
        }
        self
    }

    /// Insert an arbitrary [`serde_json::Value`] if `value` is `Some`.
    pub fn set_value(mut self, key: &str, value: Option<serde_json::Value>) -> Self {
        if let Some(v) = value {
            self.map.insert(key.to_string(), v);
        }
        self
    }

    /// Insert an `f64` field if `value` is `Some`.
    pub fn set_f64(mut self, key: &str, value: Option<f64>) -> Self {
        if let Some(v) = value {
            if let Some(n) = serde_json::Number::from_f64(v) {
                self.map
                    .insert(key.to_string(), serde_json::Value::Number(n));
            }
        }
        self
    }

    /// Returns `true` when no fields have been set.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Consume the builder and return the patch as a [`serde_json::Value`].
    pub fn build(self) -> serde_json::Value {
        serde_json::Value::Object(self.map)
    }
}

impl Default for PatchBuilder {
    fn default() -> Self {
        Self::new()
    }
}
