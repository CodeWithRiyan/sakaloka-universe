//! Role-Based Access Control (RBAC) — roles, scopes, and enforcement.

pub mod guard;
pub mod matrix;

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
    /// Read entity records.
    EntityRead,
    /// Create or update entity records.
    EntityWrite,
    /// Delete entity records (admin only).
    EntityDelete,
    /// Execute semantic search queries.
    SearchRead,
    /// Read user accounts (admin only).
    UserRead,
    /// Manage user accounts (admin only).
    UserManage,
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
            Scope::EntityRead => "entity:read",
            Scope::EntityWrite => "entity:write",
            Scope::EntityDelete => "entity:delete",
            Scope::SearchRead => "search:read",
            Scope::UserRead => "user:read",
            Scope::UserManage => "user:manage",
            Scope::DbRead => "db:read",
            Scope::DbWrite => "db:write",
            Scope::DbAdmin => "db:admin",
            Scope::ZenohPublish => "zenoh:publish",
            Scope::ZenohSubscribe => "zenoh:subscribe",
        };
        write!(f, "{s}")
    }
}
