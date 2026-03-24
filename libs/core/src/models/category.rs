use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId as Thing};
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

/// Represents a Category record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct Category {
    /// The record ID (e.g. `category:01J...`).
    #[schema(value_type = String)]
    pub id: Thing,
    /// Category name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Optional description.
    pub description: Option<String>,
    /// Parent category for nesting.
    #[schema(value_type = Option<String>)]
    pub parent_id: Option<Thing>,
    /// Category image URL.
    pub image_url: Option<String>,
    /// Display sort order.
    pub sort_order: i64,
    /// Whether the category is active.
    pub is_active: bool,
    /// Owning organization.
    #[schema(value_type = String)]
    pub organization_id: Thing,
    /// User who created this category.
    #[schema(value_type = Option<String>)]
    pub created_by: Option<Thing>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}
