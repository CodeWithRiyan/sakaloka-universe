//! Permission catalogue and normalisation helpers.
//!
//! Converts stored `role.permissions` JSON objects into canonical scope
//! strings embedded in User JWTs. The same catalogue also drives validation
//! and frontend-friendly permission discovery for business roles.

use serde_json::{Map, Value};
use std::collections::BTreeSet;

/// A single permission action exposed for a business module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionActionCatalog {
    /// The action key persisted under the module in `role.permissions`.
    pub key: &'static str,
    /// A human-readable label for UIs.
    pub label: &'static str,
}

/// A business permission module exposed by `/roles/permissions`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionModuleCatalog {
    /// The module key persisted in `role.permissions`.
    pub key: &'static str,
    /// A human-readable label for UIs.
    pub label: &'static str,
    /// A short description of the module.
    pub description: &'static str,
    /// The allowed actions for this business module.
    pub permissions: &'static [PermissionActionCatalog],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PermissionKind {
    Business,
    Technical,
}

const ACTION_READ: PermissionActionCatalog = PermissionActionCatalog {
    key: "read",
    label: "Read",
};
const ACTION_CREATE: PermissionActionCatalog = PermissionActionCatalog {
    key: "create",
    label: "Create",
};
const ACTION_UPDATE: PermissionActionCatalog = PermissionActionCatalog {
    key: "update",
    label: "Update",
};
const ACTION_DELETE: PermissionActionCatalog = PermissionActionCatalog {
    key: "delete",
    label: "Delete",
};
const ACTION_SELECT: PermissionActionCatalog = PermissionActionCatalog {
    key: "select",
    label: "Select",
};

const ORGANIZATION_ACTIONS: &[PermissionActionCatalog] = &[
    ACTION_READ,
    ACTION_CREATE,
    ACTION_UPDATE,
    ACTION_DELETE,
    ACTION_SELECT,
];
const CRUD_ACTIONS: &[PermissionActionCatalog] =
    &[ACTION_READ, ACTION_CREATE, ACTION_UPDATE, ACTION_DELETE];
const STOCK_ACTIONS: &[PermissionActionCatalog] = &[ACTION_READ, ACTION_UPDATE];
const POS_ACTIONS: &[PermissionActionCatalog] = &[ACTION_READ, ACTION_CREATE, ACTION_UPDATE];
const BOM_ACTIONS: &[PermissionActionCatalog] = CRUD_ACTIONS;

const BUSINESS_MODULES: &[PermissionModuleCatalog] = &[
    PermissionModuleCatalog {
        key: "organization",
        label: "Organizations",
        description: "Manage organizations and active organization context.",
        permissions: ORGANIZATION_ACTIONS,
    },
    PermissionModuleCatalog {
        key: "product",
        label: "Products",
        description: "Manage product catalog items.",
        permissions: CRUD_ACTIONS,
    },
    PermissionModuleCatalog {
        key: "brand",
        label: "Brands",
        description: "Manage product brand records.",
        permissions: CRUD_ACTIONS,
    },
    PermissionModuleCatalog {
        key: "category",
        label: "Categories",
        description: "Manage product category records.",
        permissions: CRUD_ACTIONS,
    },
    PermissionModuleCatalog {
        key: "stock",
        label: "Stock",
        description: "View and adjust inventory and POS stock.",
        permissions: STOCK_ACTIONS,
    },
    PermissionModuleCatalog {
        key: "user",
        label: "Users",
        description: "Manage users and their memberships.",
        permissions: CRUD_ACTIONS,
    },
    PermissionModuleCatalog {
        key: "role",
        label: "Roles",
        description: "Manage role definitions and assigned permissions.",
        permissions: CRUD_ACTIONS,
    },
    PermissionModuleCatalog {
        key: "pos",
        label: "POS",
        description: "Operate POS menus and order workflows.",
        permissions: POS_ACTIONS,
    },
    PermissionModuleCatalog {
        key: "bom",
        label: "Bill of Materials",
        description: "Manage BOM recipes and production runs.",
        permissions: BOM_ACTIONS,
    },
];

const KNOWN_PERMISSIONS: &[(&str, &str, &str, PermissionKind)] = &[
    (
        "organization",
        "read",
        "organization:read",
        PermissionKind::Business,
    ),
    (
        "organization",
        "create",
        "organization:create",
        PermissionKind::Business,
    ),
    (
        "organization",
        "update",
        "organization:update",
        PermissionKind::Business,
    ),
    (
        "organization",
        "delete",
        "organization:delete",
        PermissionKind::Business,
    ),
    (
        "organization",
        "select",
        "organization:select",
        PermissionKind::Business,
    ),
    ("product", "read", "product:read", PermissionKind::Business),
    (
        "product",
        "create",
        "product:create",
        PermissionKind::Business,
    ),
    (
        "product",
        "update",
        "product:update",
        PermissionKind::Business,
    ),
    (
        "product",
        "delete",
        "product:delete",
        PermissionKind::Business,
    ),
    ("brand", "read", "brand:read", PermissionKind::Business),
    ("brand", "create", "brand:create", PermissionKind::Business),
    ("brand", "update", "brand:update", PermissionKind::Business),
    ("brand", "delete", "brand:delete", PermissionKind::Business),
    (
        "category",
        "read",
        "category:read",
        PermissionKind::Business,
    ),
    (
        "category",
        "create",
        "category:create",
        PermissionKind::Business,
    ),
    (
        "category",
        "update",
        "category:update",
        PermissionKind::Business,
    ),
    (
        "category",
        "delete",
        "category:delete",
        PermissionKind::Business,
    ),
    ("stock", "read", "stock:read", PermissionKind::Business),
    ("stock", "update", "stock:update", PermissionKind::Business),
    ("user", "read", "user:read", PermissionKind::Business),
    ("user", "create", "user:create", PermissionKind::Business),
    ("user", "update", "user:update", PermissionKind::Business),
    ("user", "delete", "user:delete", PermissionKind::Business),
    ("role", "read", "role:read", PermissionKind::Business),
    ("role", "create", "role:create", PermissionKind::Business),
    ("role", "update", "role:update", PermissionKind::Business),
    ("role", "delete", "role:delete", PermissionKind::Business),
    ("pos", "read", "pos:read", PermissionKind::Business),
    ("pos", "create", "pos:create", PermissionKind::Business),
    ("pos", "update", "pos:update", PermissionKind::Business),
    ("bom", "read", "bom:read", PermissionKind::Business),
    ("bom", "create", "bom:create", PermissionKind::Business),
    ("bom", "update", "bom:update", PermissionKind::Business),
    ("bom", "delete", "bom:delete", PermissionKind::Business),
    ("db", "read", "db:read", PermissionKind::Technical),
    ("db", "write", "db:write", PermissionKind::Technical),
    ("db", "admin", "db:admin", PermissionKind::Technical),
    (
        "zenoh",
        "publish",
        "zenoh:publish",
        PermissionKind::Technical,
    ),
    (
        "zenoh",
        "subscribe",
        "zenoh:subscribe",
        PermissionKind::Technical,
    ),
];

/// Returns `true` if the given scope string is part of the canonical
/// permission vocabulary.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::is_known_scope;
///
/// assert!(is_known_scope("product:read"));
/// assert!(!is_known_scope("unknown:permission"));
/// ```
pub fn is_known_scope(scope: &str) -> bool {
    KNOWN_PERMISSIONS
        .iter()
        .any(|(_, _, canonical, _)| *canonical == scope)
}

/// Returns the UI-facing catalogue for business permissions only.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::business_permission_catalog;
///
/// let modules = business_permission_catalog();
/// assert!(modules.iter().any(|module| module.key == "product"));
/// ```
pub fn business_permission_catalog() -> Vec<PermissionModuleCatalog> {
    BUSINESS_MODULES.to_vec()
}

/// Converts a `role.permissions` JSON value into sorted, deduplicated canonical
/// scope strings suitable for JWT issuance.
///
/// Unknown module/action pairs are ignored. Validation should happen before
/// calling this function when malformed data must be rejected.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::normalize_permissions;
///
/// let perms = serde_json::json!({
///     "product": ["read", "update"],
///     "user": ["delete"],
///     "unknown": ["anything"]
/// });
///
/// let scopes = normalize_permissions(&perms);
/// assert!(scopes.contains(&"product:read".to_string()));
/// assert!(scopes.contains(&"product:update".to_string()));
/// assert!(scopes.contains(&"user:delete".to_string()));
/// assert!(!scopes.contains(&"unknown:anything".to_string()));
/// ```
pub fn normalize_permissions(permissions: &Value) -> Vec<String> {
    let obj = match permissions.as_object() {
        Some(o) => o,
        None => return Vec::new(),
    };

    let mut scopes: BTreeSet<String> = BTreeSet::new();

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
            if let Some((_, _, canonical, _)) = KNOWN_PERMISSIONS
                .iter()
                .find(|(m, a, _, _)| *m == module.as_str() && *a == action)
            {
                scopes.insert((*canonical).to_string());
            }
        }
    }

    scopes.into_iter().collect()
}

/// Validates a permissions object and returns its canonical scope strings.
///
/// # Errors
///
/// Returns `Err(...)` when the input is not the expected object-of-arrays
/// shape or contains an unknown `module:action` pair.
pub fn validated_scopes(permissions: &Value) -> Result<Vec<String>, String> {
    validate_permissions(permissions)?;
    Ok(normalize_permissions(permissions))
}

/// Validates that a permissions object only contains known business or
/// technical permission pairs.
///
/// # Errors
///
/// Returns `Err(...)` when the payload shape is invalid or the first unknown
/// permission is encountered.
pub fn validate_permissions(permissions: &Value) -> Result<(), String> {
    validate_permissions_by_kind(permissions, None)
}

/// Validates that a permissions object only contains known business
/// permissions suitable for user-managed roles.
///
/// # Errors
///
/// Returns `Err(...)` when the payload shape is invalid or contains technical
/// or unknown permissions.
pub fn validate_business_permissions(permissions: &Value) -> Result<(), String> {
    validate_permissions_by_kind(permissions, Some(PermissionKind::Business))
}

/// Returns a permissions JSON object containing every business permission.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::full_business_permissions;
///
/// let perms = full_business_permissions();
/// let scopes = sakaloka_secure::rbac::permission::normalize_permissions(&perms);
/// assert!(scopes.contains(&"organization:read".to_string()));
/// assert!(scopes.contains(&"pos:update".to_string()));
/// ```
pub fn full_business_permissions() -> Value {
    permissions_value_for_kind(PermissionKindSet::BUSINESS)
}

/// Returns a permissions JSON object containing every known permission,
/// including technical ones reserved for system/bootstrap roles.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::permission::full_system_permissions;
///
/// let perms = full_system_permissions();
/// let scopes = sakaloka_secure::rbac::permission::normalize_permissions(&perms);
/// assert!(scopes.contains(&"db:admin".to_string()));
/// assert!(scopes.contains(&"product:delete".to_string()));
/// ```
pub fn full_system_permissions() -> Value {
    permissions_value_for_kind(PermissionKind::Technical | PermissionKind::Business)
}

fn validate_permissions_by_kind(
    permissions: &Value,
    expected_kind: Option<PermissionKind>,
) -> Result<(), String> {
    let obj = match permissions.as_object() {
        Some(o) => o,
        None => {
            return Err("permissions: expected an object of modules to action arrays".to_string())
        }
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
            let found = KNOWN_PERMISSIONS.iter().find(|(m, a, _, kind)| {
                *m == module.as_str()
                    && *a == action
                    && expected_kind.is_none_or(|expected| *kind == expected)
            });

            if found.is_none() {
                return Err(format!("{module}:{action}"));
            }
        }
    }

    Ok(())
}

fn permissions_value_for_kind(kind: PermissionKindSet) -> Value {
    let mut map: Map<String, Value> = Map::new();

    for (module, action, _, entry_kind) in KNOWN_PERMISSIONS {
        if kind.contains(*entry_kind) {
            let entry = map
                .entry((*module).to_string())
                .or_insert_with(|| Value::Array(Vec::new()));
            if let Value::Array(actions) = entry {
                actions.push(Value::String((*action).to_string()));
            }
        }
    }

    Value::Object(map)
}

#[derive(Clone, Copy)]
struct PermissionKindSet(u8);

impl PermissionKindSet {
    const BUSINESS: Self = Self(0b01);
    const TECHNICAL: Self = Self(0b10);

    fn contains(self, kind: PermissionKind) -> bool {
        match kind {
            PermissionKind::Business => self.0 & Self::BUSINESS.0 != 0,
            PermissionKind::Technical => self.0 & Self::TECHNICAL.0 != 0,
        }
    }
}

impl std::ops::BitOr<PermissionKind> for PermissionKind {
    type Output = PermissionKindSet;

    fn bitor(self, rhs: PermissionKind) -> Self::Output {
        let left = match self {
            PermissionKind::Business => PermissionKindSet::BUSINESS,
            PermissionKind::Technical => PermissionKindSet::TECHNICAL,
        };
        let right = match rhs {
            PermissionKind::Business => PermissionKindSet::BUSINESS,
            PermissionKind::Technical => PermissionKindSet::TECHNICAL,
        };
        PermissionKindSet(left.0 | right.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_returns_known_scopes() {
        let perms = serde_json::json!({
            "product": ["read", "update"],
            "user": ["delete"],
            "organization": ["select"],
        });
        let scopes = normalize_permissions(&perms);
        assert!(scopes.contains(&"product:read".to_string()));
        assert!(scopes.contains(&"product:update".to_string()));
        assert!(scopes.contains(&"user:delete".to_string()));
        assert!(scopes.contains(&"organization:select".to_string()));
        assert_eq!(scopes.len(), 4);
    }

    #[test]
    fn normalize_ignores_unknown_entries() {
        let perms = serde_json::json!({
            "product": ["read"],
            "mystery": ["power"],
        });
        let scopes = normalize_permissions(&perms);
        assert_eq!(scopes, vec!["product:read".to_string()]);
    }

    #[test]
    fn normalize_deduplicates() {
        let perms = serde_json::json!({
            "product": ["read", "read"],
        });
        let scopes = normalize_permissions(&perms);
        assert_eq!(scopes, vec!["product:read".to_string()]);
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
            "product": ["read", "update"],
            "user": ["delete"],
        });
        assert!(validate_permissions(&ok).is_ok());
    }

    #[test]
    fn validate_business_rejects_technical_permissions() {
        let bad = serde_json::json!({
            "db": ["read"],
        });
        assert!(validate_business_permissions(&bad).is_err());
    }

    #[test]
    fn validate_rejects_unknown_module_action() {
        let bad = serde_json::json!({
            "product": ["read"],
            "mystery": ["power"],
        });
        match validate_permissions(&bad) {
            Err(e) => assert!(e.contains("mystery:power")),
            Ok(_) => panic!("Expected error for mystery:power"),
        }
    }

    #[test]
    fn validate_rejects_non_array_actions() {
        let bad = serde_json::json!({ "product": "read" });
        assert!(validate_permissions(&bad).is_err());
    }

    #[test]
    fn validate_rejects_null_permissions() {
        let bad = serde_json::json!(null);
        assert!(validate_permissions(&bad).is_err());
    }

    #[test]
    fn validated_scopes_rejects_malformed_permissions() {
        let bad = serde_json::json!({ "product": "read" });
        assert!(validated_scopes(&bad).is_err());
    }

    #[test]
    fn business_catalog_exposes_expected_modules() {
        let modules = business_permission_catalog();
        assert!(modules.iter().any(|module| module.key == "organization"));
        assert!(modules.iter().any(|module| module.key == "pos"));
    }

    #[test]
    fn full_business_permissions_excludes_technical_scopes() {
        let perms = full_business_permissions();
        let scopes = normalize_permissions(&perms);
        assert!(scopes.contains(&"role:update".to_string()));
        assert!(!scopes.contains(&"db:admin".to_string()));
    }

    #[test]
    fn full_system_permissions_covers_technical_scopes() {
        let perms = full_system_permissions();
        let scopes = normalize_permissions(&perms);
        assert!(scopes.contains(&"db:admin".to_string()));
        assert!(scopes.contains(&"zenoh:publish".to_string()));
    }

    #[test]
    fn is_known_scope_identifies_canonical_scopes() {
        assert!(is_known_scope("product:read"));
        assert!(is_known_scope("organization:select"));
        assert!(is_known_scope("zenoh:subscribe"));
        assert!(!is_known_scope("mystery:power"));
        assert!(!is_known_scope(""));
    }
}
