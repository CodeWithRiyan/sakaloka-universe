//! Role-to-scope lookup matrix.
//!
//! This matrix exists for non-user defaults and service-oriented role presets.
//! User JWT scopes are issued from the `role.permissions` JSON stored in the
//! database and should not be derived from this module.

use crate::rbac::{Role, Scope};

/// Returns the list of [`Scope`]s allowed for a given [`Role`].
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::rbac::{Role, Scope, matrix::allowed_scopes};
///
/// let scopes = allowed_scopes(&Role::Viewer);
/// assert!(scopes.contains(&Scope::ProductRead));
/// assert!(!scopes.contains(&Scope::ProductDelete));
/// ```
pub fn allowed_scopes(role: &Role) -> Vec<Scope> {
    match role {
        Role::Admin => vec![
            Scope::OrganizationRead,
            Scope::OrganizationCreate,
            Scope::OrganizationUpdate,
            Scope::OrganizationDelete,
            Scope::OrganizationSelect,
            Scope::ProductRead,
            Scope::ProductCreate,
            Scope::ProductUpdate,
            Scope::ProductDelete,
            Scope::BrandRead,
            Scope::BrandCreate,
            Scope::BrandUpdate,
            Scope::BrandDelete,
            Scope::CategoryRead,
            Scope::CategoryCreate,
            Scope::CategoryUpdate,
            Scope::CategoryDelete,
            Scope::StockRead,
            Scope::StockUpdate,
            Scope::UserRead,
            Scope::UserCreate,
            Scope::UserUpdate,
            Scope::UserDelete,
            Scope::RoleRead,
            Scope::RoleCreate,
            Scope::RoleUpdate,
            Scope::RoleDelete,
            Scope::PosRead,
            Scope::PosCreate,
            Scope::PosUpdate,
            Scope::BomRead,
            Scope::BomCreate,
            Scope::BomUpdate,
            Scope::BomDelete,
            Scope::DbRead,
            Scope::DbWrite,
            Scope::DbAdmin,
            Scope::ZenohPublish,
            Scope::ZenohSubscribe,
        ],
        Role::Editor => vec![
            Scope::OrganizationRead,
            Scope::OrganizationSelect,
            Scope::ProductRead,
            Scope::ProductCreate,
            Scope::ProductUpdate,
            Scope::BrandRead,
            Scope::BrandCreate,
            Scope::BrandUpdate,
            Scope::CategoryRead,
            Scope::CategoryCreate,
            Scope::CategoryUpdate,
            Scope::StockRead,
            Scope::StockUpdate,
            Scope::PosRead,
            Scope::PosCreate,
            Scope::PosUpdate,
            Scope::BomRead,
            Scope::BomCreate,
            Scope::BomUpdate,
        ],
        Role::Viewer => vec![
            Scope::OrganizationRead,
            Scope::ProductRead,
            Scope::BrandRead,
            Scope::CategoryRead,
            Scope::StockRead,
            Scope::PosRead,
            Scope::BomRead,
        ],
        Role::Service => vec![
            Scope::DbRead,
            Scope::DbWrite,
            Scope::ZenohPublish,
            Scope::ZenohSubscribe,
        ],
    }
}
