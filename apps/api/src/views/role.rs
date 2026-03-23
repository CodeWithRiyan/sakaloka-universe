//! Role view DTOs.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::_entities::roles;

/// Full role response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleResponse {
    /// Role ID.
    pub id: Uuid,
    /// Role name.
    pub name: String,
    /// Organization ID.
    pub organization_id: Uuid,
    /// Whether this is a system-defined role.
    pub is_system_role: bool,
    /// JSON permissions for this role.
    pub permissions: serde_json::Value,
    /// User who created this role.
    pub created_by: Option<Uuid>,
    /// Whether the role is active.
    pub is_active: bool,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
    /// Record last-update timestamp.
    pub updated_at: DateTime<FixedOffset>,
}

/// Request body for creating a role.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleRequest {
    /// Role name.
    pub name: String,
    /// JSON permissions to assign.
    pub permissions: serde_json::Value,
    /// Whether the role should be active immediately.
    pub is_active: Option<bool>,
}

/// Request body for updating a role (all fields optional).
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleRequest {
    /// Updated role name.
    pub name: Option<String>,
    /// Updated permissions.
    pub permissions: Option<serde_json::Value>,
    /// Updated active flag.
    pub is_active: Option<bool>,
}

/// Response listing all available permissions.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsResponse {
    /// List of all known permission strings.
    pub permissions: Vec<String>,
}

impl RoleResponse {
    /// Convert a SeaORM role model into a response DTO.
    pub fn from_model(model: roles::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            organization_id: model.organization_id,
            is_system_role: model.is_system_role,
            permissions: model.permissions,
            created_by: model.created_by,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
