//! Category view DTOs.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::_entities::categories;

/// Full category response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryResponse {
    /// Category ID.
    pub id: Uuid,
    /// Category name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional parent category ID for hierarchy.
    pub parent_id: Option<Uuid>,
    /// Optional image URL.
    pub image_url: Option<String>,
    /// Owning organization ID.
    pub organization_id: Uuid,
    /// User who created this category.
    pub created_by: Option<Uuid>,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
    /// Record last-update timestamp.
    pub updated_at: DateTime<FixedOffset>,
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
    pub parent_id: Option<Uuid>,
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
    pub parent_id: Option<Uuid>,
    /// Updated image URL.
    pub image_url: Option<String>,
}

impl CategoryResponse {
    /// Convert a SeaORM category model into a response DTO.
    pub fn from_model(model: categories::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            description: model.description,
            parent_id: model.parent_id,
            image_url: model.image_url,
            organization_id: model.organization_id,
            created_by: model.created_by,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
