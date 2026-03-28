//! Permission normalisation helpers.
//!
//! Converts a stored `role.permissions` JSON object into the canonical scope
//! strings that are embedded in a User JWT.  This is the **single conversion
//! path** for user token issuance; the RBAC matrix in `matrix.rs` is reserved
//! for non-user (service-to-service) contexts only.
//!
//! ## Expected DB shape
//!
//! ```json
//! {
//!   "entity": ["read", "write"],
//!   "user":   ["read", "manage"]
//! }
//! ```
//!
//! Each `module + action` pair maps to a canonical scope string, e.g.
//! `entity + read → entity:read`.  Unknown modules or actions are **silently
//! ignored** so that future additions to the permission vocabulary do not break
//! existing tokens.

use serde_json::Value;

/// The set of `(module, action)` pairs that map to a known [`Scope`].
const KNOWN_PERMISSIONS: &[(&str, &str, &str)] = &[
    ("entity", "read", "entity:read"),
    ("entity", "write", "entity:write"),
    ("entity", "delete", "entity:delete"),
    ("search", "read", "search:read"),
    ("user", "read", "user:read"),
    ("user", "manage", "user:manage"),
    ("db", "read", "db:read"),
    ("db", "write", "db:write"),
    ("db", "admin", "db:admin"),
    ("zenoh", "publish", "zenoh:publish"),
    ("zenoh", "subscribe", "zenoh:subscribe"),
];

/// Returns `true` if the given `module:action` scope string is known.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::is_known_scope;
///
/// assert!(is_known_scope("entity:read"));
/// assert!(!is_known_scope("unknown:permission"));
/// ```
pub fn is_known_scope(scope: &str) -> bool {
    KNOWN_PERMISSIONS
        .iter()
        .any(|(_, _, canonical)| *canonical == scope)
}

/// Converts a `role.permissions` JSON value into sorted, deduplicated canonical
/// scope strings suitable for JWT issuance.
///
/// The `permissions` value must be a JSON **object** whose keys are module names
/// and whose values are JSON **arrays of action strings**.  Any other shape is
/// treated as an empty permission set.
///
/// Unknown module/action pairs are silently ignored.  No new permissions are
/// invented; the returned list is strictly a subset of the canonical vocabulary.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::normalize_permissions;
///
/// let perms = serde_json::json!({
///     "entity": ["read", "write"],
///     "user":   ["manage"],
///     "unknown": ["anything"]
/// });
///
/// let scopes = normalize_permissions(&perms);
/// assert!(scopes.contains(&"entity:read".to_string()));
/// assert!(scopes.contains(&"entity:write".to_string()));
/// assert!(scopes.contains(&"user:manage".to_string()));
/// // Unknown entries are dropped
/// assert!(!scopes.contains(&"unknown:anything".to_string()));
/// ```
///
/// Empty or malformed input:
/// ```rust
/// use sakaloka_secure::rbac::permission::normalize_permissions;
///
/// assert!(normalize_permissions(&serde_json::json!({})).is_empty());
/// assert!(normalize_permissions(&serde_json::json!(null)).is_empty());
/// ```
pub fn normalize_permissions(permissions: &Value) -> Vec<String> {
    let obj = match permissions.as_object() {
        Some(o) => o,
        None => return Vec::new(),
    };

    let mut scopes: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    for (module, actions_val) in obj {
        let actions = match actions_val.as_array() {
            Some(a) => a,
            None => continue,
        };
        for action_val in actions {
            let action = match action_val.as_str() {
                Some(s) => s,
                None => continue,
            };
            // Look up the canonical string for this (module, action) pair.
            if let Some((_, _, canonical)) = KNOWN_PERMISSIONS
                .iter()
                .find(|(m, a, _)| *m == module.as_str() && *a == action)
            {
                scopes.insert((*canonical).to_string());
            }
        }
    }

    scopes.into_iter().collect()
}

/// Validates that every scope string in `permissions` is a known canonical
/// scope.  Returns the first unknown scope string if any is found.
///
/// Call this before persisting a role's permission set so that only valid
/// entries reach the database.
///
/// The input must be the object shape described in [`normalize_permissions`].
///
/// # Errors
///
/// Returns `Err(unknown_scope)` containing the first unrecognised
/// `module:action` string encountered.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::validate_permissions;
///
/// let ok = serde_json::json!({ "entity": ["read"] });
/// assert!(validate_permissions(&ok).is_ok());
///
/// let bad = serde_json::json!({ "entity": ["read"], "mystery": ["power"] });
/// assert!(validate_permissions(&bad).is_err());
/// ```
pub fn validate_permissions(permissions: &Value) -> Result<(), String> {
    let obj = match permissions.as_object() {
        Some(o) => o,
        None => return Ok(()), // null / non-object → treated as empty, not invalid
    };

    for (module, actions_val) in obj {
        let actions = match actions_val.as_array() {
            Some(a) => a,
            None => {
                return Err(format!(
                    "permissions[\"{module}\"]: expected an array of action strings"
                ))
            }
        };
        for action_val in actions {
            let action = match action_val.as_str() {
                Some(s) => s,
                None => {
                    return Err(format!(
                        "permissions[\"{module}\"]: action must be a string"
                    ))
                }
            };
            let found = KNOWN_PERMISSIONS
                .iter()
                .any(|(m, a, _)| *m == module.as_str() && *a == action);
            if !found {
                return Err(format!("{module}:{action}"));
            }
        }
    }

    Ok(())
}

/// Returns a [`serde_json::Value`] object containing all known permissions
/// (every module + every action), suitable for seeding a superadmin role.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::full_permissions;
///
/// let perms = full_permissions();
/// let scopes = sakaloka_secure::rbac::permission::normalize_permissions(&perms);
/// assert_eq!(scopes.len(), 11); // all canonical scopes
/// ```
pub fn full_permissions() -> Value {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (module, action, _) in KNOWN_PERMISSIONS {
        map.entry((*module).to_string())
            .or_default()
            .push((*action).to_string());
    }

    serde_json::to_value(map).unwrap_or_else(|_| Value::Object(Default::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_returns_known_scopes() {
        let perms = serde_json::json!({
            "entity": ["read", "write", "delete"],
            "user":   ["read", "manage"],
            "search": ["read"],
        });
        let scopes = normalize_permissions(&perms);
        assert!(scopes.contains(&"entity:read".to_string()));
        assert!(scopes.contains(&"entity:write".to_string()));
        assert!(scopes.contains(&"entity:delete".to_string()));
        assert!(scopes.contains(&"user:read".to_string()));
        assert!(scopes.contains(&"user:manage".to_string()));
        assert!(scopes.contains(&"search:read".to_string()));
        assert_eq!(scopes.len(), 6);
    }

    #[test]
    fn normalize_ignores_unknown_entries() {
        let perms = serde_json::json!({
            "entity": ["read"],
            "mystery": ["power"],
        });
        let scopes = normalize_permissions(&perms);
        assert_eq!(scopes, vec!["entity:read".to_string()]);
    }

    #[test]
    fn normalize_deduplicates() {
        let perms = serde_json::json!({
            "entity": ["read", "read"],
        });
        let scopes = normalize_permissions(&perms);
        assert_eq!(scopes, vec!["entity:read".to_string()]);
    }

    #[test]
    fn normalize_empty_object_yields_no_scopes() {
        let scopes = normalize_permissions(&serde_json::json!({}));
        assert!(scopes.is_empty());
    }

    #[test]
    fn normalize_null_yields_no_scopes() {
        let scopes = normalize_permissions(&serde_json::json!(null));
        assert!(scopes.is_empty());
    }

    #[test]
    fn validate_accepts_known_permissions() {
        let ok = serde_json::json!({
            "entity": ["read", "write"],
            "user": ["manage"],
        });
        assert!(validate_permissions(&ok).is_ok());
    }

    #[test]
    fn validate_rejects_unknown_module_action() {
        let bad = serde_json::json!({
            "entity": ["read"],
            "mystery": ["power"],
        });
        let err = validate_permissions(&bad);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("mystery:power"));
    }

    #[test]
    fn validate_rejects_non_array_actions() {
        let bad = serde_json::json!({ "entity": "read" });
        assert!(validate_permissions(&bad).is_err());
    }

    #[test]
    fn full_permissions_covers_all_scopes() {
        let perms = full_permissions();
        let scopes = normalize_permissions(&perms);
        assert_eq!(scopes.len(), KNOWN_PERMISSIONS.len());
    }

    #[test]
    fn is_known_scope_identifies_canonical_scopes() {
        assert!(is_known_scope("entity:read"));
        assert!(is_known_scope("zenoh:subscribe"));
        assert!(!is_known_scope("mystery:power"));
        assert!(!is_known_scope(""));
    }
}
