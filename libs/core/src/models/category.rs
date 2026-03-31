use serde::{Deserialize, Serialize};

/// Represents a Category record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Category {
    /// The record ID.
    pub id: String,
    /// Category name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Optional description.
    pub description: Option<String>,
    /// Parent category for nesting.
    pub parent_id: Option<String>,
    /// Category image URL.
    pub image_url: Option<String>,
    /// Display sort order.
    pub sort_order: i64,
    /// Whether the category is active.
    pub is_active: bool,
    /// Owning organization.
    pub organization_id: String,
    /// User who created this category.
    pub created_by: Option<String>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
