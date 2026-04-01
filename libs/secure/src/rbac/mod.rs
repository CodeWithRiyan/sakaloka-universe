//! Role-Based Access Control (RBAC) — roles, scopes, and enforcement.

pub mod guard;
/// Role-to-scope lookup matrix.
///
/// **Non-user auth only.** This matrix must not be used for issuing User JWTs.
/// User JWT scopes are derived from the `role.permissions` column via
/// [`permission::normalize_permissions`].  The matrix is retained solely for
/// service-to-service tokens and internal defaults.
pub mod matrix;
/// Permission normalisation helpers for converting DB `role.permissions` JSON
/// objects into canonical scope strings embedded in User JWTs.
pub mod permission;

use serde::{Deserialize, Serialize};
use std::fmt;

/// All roles in the Sakaloka-Universe permission system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Full access to all resources and admin operations.
    Admin,
    /// Can read and write entities but cannot delete or manage users.
    Editor,
    /// Read-only access to entities and search.
    Viewer,
    /// Internal service-to-service role (planets only).
    Service,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Editor => write!(f, "editor"),
            Role::Viewer => write!(f, "viewer"),
            Role::Service => write!(f, "service"),
        }
    }
}

/// All permission scopes in the Sakaloka-Universe.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Read organization records.
    OrganizationRead,
    /// Create organization records.
    OrganizationCreate,
    /// Update organization records.
    OrganizationUpdate,
    /// Delete organization records.
    OrganizationDelete,
    /// Select an active organization context.
    OrganizationSelect,
    /// Read product records.
    ProductRead,
    /// Create product records.
    ProductCreate,
    /// Update product records.
    ProductUpdate,
    /// Delete product records.
    ProductDelete,
    /// Read brand records.
    BrandRead,
    /// Create brand records.
    BrandCreate,
    /// Update brand records.
    BrandUpdate,
    /// Delete brand records.
    BrandDelete,
    /// Read category records.
    CategoryRead,
    /// Create category records.
    CategoryCreate,
    /// Update category records.
    CategoryUpdate,
    /// Delete category records.
    CategoryDelete,
    /// Read stock records.
    StockRead,
    /// Update stock records.
    StockUpdate,
    /// Read user accounts.
    UserRead,
    /// Create user accounts.
    UserCreate,
    /// Update user accounts.
    UserUpdate,
    /// Delete user accounts.
    UserDelete,
    /// Read role definitions.
    RoleRead,
    /// Create role definitions.
    RoleCreate,
    /// Update role definitions.
    RoleUpdate,
    /// Delete role definitions.
    RoleDelete,
    /// Read POS menus and orders.
    PosRead,
    /// Create POS orders.
    PosCreate,
    /// Update POS orders.
    PosUpdate,
    /// Read BOM records.
    BomRead,
    /// Create BOM records.
    BomCreate,
    /// Update BOM records.
    BomUpdate,
    /// Delete BOM records.
    BomDelete,
    /// Read from the database (service only).
    DbRead,
    /// Write to the database (service only).
    DbWrite,
    /// Database admin operations (admin only).
    DbAdmin,
    /// Publish messages to Zenoh topics.
    ZenohPublish,
    /// Subscribe to Zenoh topics.
    ZenohSubscribe,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Scope::OrganizationRead => "organization:read",
            Scope::OrganizationCreate => "organization:create",
            Scope::OrganizationUpdate => "organization:update",
            Scope::OrganizationDelete => "organization:delete",
            Scope::OrganizationSelect => "organization:select",
            Scope::ProductRead => "product:read",
            Scope::ProductCreate => "product:create",
            Scope::ProductUpdate => "product:update",
            Scope::ProductDelete => "product:delete",
            Scope::BrandRead => "brand:read",
            Scope::BrandCreate => "brand:create",
            Scope::BrandUpdate => "brand:update",
            Scope::BrandDelete => "brand:delete",
            Scope::CategoryRead => "category:read",
            Scope::CategoryCreate => "category:create",
            Scope::CategoryUpdate => "category:update",
            Scope::CategoryDelete => "category:delete",
            Scope::StockRead => "stock:read",
            Scope::StockUpdate => "stock:update",
            Scope::UserRead => "user:read",
            Scope::UserCreate => "user:create",
            Scope::UserUpdate => "user:update",
            Scope::UserDelete => "user:delete",
            Scope::RoleRead => "role:read",
            Scope::RoleCreate => "role:create",
            Scope::RoleUpdate => "role:update",
            Scope::RoleDelete => "role:delete",
            Scope::PosRead => "pos:read",
            Scope::PosCreate => "pos:create",
            Scope::PosUpdate => "pos:update",
            Scope::BomRead => "bom:read",
            Scope::BomCreate => "bom:create",
            Scope::BomUpdate => "bom:update",
            Scope::BomDelete => "bom:delete",
            Scope::DbRead => "db:read",
            Scope::DbWrite => "db:write",
            Scope::DbAdmin => "db:admin",
            Scope::ZenohPublish => "zenoh:publish",
            Scope::ZenohSubscribe => "zenoh:subscribe",
        };
        write!(f, "{s}")
    }
}
