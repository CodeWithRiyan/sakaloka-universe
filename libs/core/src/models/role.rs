use serde::{Deserialize, Serialize};

/// Represents a Role record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Role {
    /// The record ID.
    pub id: String,
    /// Role name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// JSON permissions object.
    pub permissions: Option<serde_json::Value>,
    /// Owning organization.
    pub organization_id: String,
    /// Whether this is a system-defined role.
    pub is_system_role: bool,
    /// Whether the role is active.
    pub is_active: bool,
    /// User who created this role.
    pub created_by: Option<String>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
