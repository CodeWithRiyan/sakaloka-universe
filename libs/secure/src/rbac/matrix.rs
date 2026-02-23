//! Role-to-scope lookup matrix.
//!
//! This is the single source of truth for which scopes each role is granted.
//! The matrix mirrors the RBAC table in the PRD Section 5.2.

use crate::rbac::{Role, Scope};

/// Returns the list of [`Scope`]s allowed for a given [`Role`].
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::{Role, Scope, matrix::allowed_scopes};
///
/// let scopes = allowed_scopes(&Role::Viewer);
/// assert!(scopes.contains(&Scope::EntityRead));
/// assert!(!scopes.contains(&Scope::EntityWrite));
/// ```
pub fn allowed_scopes(role: &Role) -> Vec<Scope> {
    match role {
        Role::Admin => vec![
            Scope::EntityRead,
            Scope::EntityWrite,
            Scope::EntityDelete,
            Scope::SearchRead,
            Scope::UserRead,
            Scope::UserManage,
            Scope::DbRead,
            Scope::DbWrite,
            Scope::DbAdmin,
            Scope::ZenohPublish,
            Scope::ZenohSubscribe,
        ],
        Role::Editor => vec![Scope::EntityRead, Scope::EntityWrite, Scope::SearchRead],
        Role::Viewer => vec![Scope::EntityRead, Scope::SearchRead],
        Role::Service => vec![
            Scope::EntityRead,
            Scope::EntityWrite,
            Scope::SearchRead,
            Scope::DbRead,
            Scope::DbWrite,
            Scope::ZenohPublish,
            Scope::ZenohSubscribe,
        ],
    }
}
