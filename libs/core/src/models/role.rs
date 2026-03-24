use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId as Thing};
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

/// Represents a Role record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct Role {
    /// The record ID (e.g. `role:01J...`).
    #[schema(value_type = String)]
    pub id: Thing,
    /// Role name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// JSON permissions object.
    pub permissions: Option<serde_json::Value>,
    /// Owning organization.
    #[schema(value_type = String)]
    pub organization_id: Thing,
    /// Whether this is a system-defined role.
    pub is_system_role: bool,
    /// Whether the role is active.
    pub is_active: bool,
    /// User who created this role.
    #[schema(value_type = Option<String>)]
    pub created_by: Option<Thing>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}
