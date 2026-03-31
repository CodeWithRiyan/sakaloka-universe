use serde::{Deserialize, Serialize};

/// Represents a Brand record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Brand {
    /// The record ID.
    pub id: String,
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
    pub organization_id: String,
    /// User who created this brand.
    pub created_by: Option<String>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
