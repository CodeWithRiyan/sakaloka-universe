//! Category view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::record_id_to_string;

/// Full category response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryResponse {
    /// Category ID.
    pub id: String,
    /// Category name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional parent category ID for hierarchy.
    pub parent_id: Option<String>,
    /// Optional image URL.
    pub image_url: Option<String>,
    /// Display sort order.
    pub sort_order: i64,
    /// Whether the category is active.
    pub is_active: bool,
    /// Owning organization ID.
    pub organization_id: String,
    /// User who created this category.
    pub created_by: Option<String>,
    /// Record creation timestamp.
    pub created_at: String,
    /// Record last-update timestamp.
    pub updated_at: String,
}

/// Request body for creating a category.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryRequest {
    /// Category name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional parent category ID.
    pub parent_id: Option<String>,
    /// Optional image URL.
    pub image_url: Option<String>,
}

/// Request body for updating a category (all fields optional).
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryRequest {
    /// Updated category name.
    pub name: Option<String>,
    /// Updated description.
    pub description: Option<String>,
    /// Updated parent category ID.
    pub parent_id: Option<String>,
    /// Updated image URL.
    pub image_url: Option<String>,
}

impl CategoryResponse {
    /// Convert a domain [`Category`](sakaloka_core::models::category::Category)
    /// model into a response DTO.
    pub fn from_model(model: &sakaloka_core::models::category::Category) -> Self {
        Self {
            id: record_id_to_string(&model.id),
            name: model.name.clone(),
            slug: model.slug.clone(),
            description: model.description.clone(),
            parent_id: model.parent_id.as_ref().map(record_id_to_string),
            image_url: model.image_url.clone(),
            sort_order: model.sort_order,
            is_active: model.is_active,
            organization_id: record_id_to_string(&model.organization_id),
            created_by: model.created_by.as_ref().map(record_id_to_string),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
