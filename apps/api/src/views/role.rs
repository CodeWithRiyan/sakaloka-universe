//! Role view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::record_id_to_string;

/// Full role response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleResponse {
    /// Role ID.
    pub id: String,
    /// Role name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Organization ID.
    pub organization_id: String,
    /// Whether this is a system-defined role.
    pub is_system_role: bool,
    /// JSON permissions for this role.
    pub permissions: Option<serde_json::Value>,
    /// User who created this role.
    pub created_by: Option<String>,
    /// Whether the role is active.
    pub is_active: bool,
    /// Record creation timestamp.
    pub created_at: String,
    /// Record last-update timestamp.
    pub updated_at: String,
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
    /// Convert a domain [`Role`](sakaloka_core::models::role::Role) model
    /// into a response DTO.
    pub fn from_model(model: &sakaloka_core::models::role::Role) -> Self {
        Self {
            id: record_id_to_string(&model.id),
            name: model.name.clone(),
            description: model.description.clone(),
            organization_id: record_id_to_string(&model.organization_id),
            is_system_role: model.is_system_role,
            permissions: model.permissions.clone(),
            created_by: model.created_by.as_ref().map(record_id_to_string),
            is_active: model.is_active,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
