use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId as Thing};
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

/// Represents a Brand record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct Brand {
    /// The record ID (e.g. `brand:01J...`).
    #[schema(value_type = String)]
    pub id: Thing,
    /// Brand name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Optional description.
    pub description: Option<String>,
    /// Brand logo URL.
    pub logo: Option<String>,
    /// Brand website URL.
    pub website: Option<String>,
    /// Whether the brand is active.
    pub is_active: bool,
    /// Owning organization.
    #[schema(value_type = String)]
    pub organization_id: Thing,
    /// User who created this brand.
    #[schema(value_type = Option<String>)]
    pub created_by: Option<Thing>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}
