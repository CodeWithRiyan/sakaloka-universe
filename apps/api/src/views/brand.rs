//! Brand view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::record_id_to_string;

/// Full brand response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BrandResponse {
    /// Brand ID.
    pub id: String,
    /// Brand name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional logo URL.
    pub logo: Option<String>,
    /// Optional website URL.
    pub website: Option<String>,
    /// Whether the brand is active.
    pub is_active: bool,
    /// Owning organization ID.
    pub organization_id: String,
    /// User who created this brand.
    pub created_by: Option<String>,
    /// Record creation timestamp.
    pub created_at: String,
    /// Record last-update timestamp.
    pub updated_at: String,
}

/// Request body for creating a brand.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrandRequest {
    /// Brand name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional logo URL.
    pub logo: Option<String>,
    /// Optional website URL.
    pub website: Option<String>,
}

/// Request body for updating a brand (all fields optional).
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBrandRequest {
    /// Updated brand name.
    pub name: Option<String>,
    /// Updated description.
    pub description: Option<String>,
    /// Updated logo URL.
    pub logo: Option<String>,
    /// Updated website URL.
    pub website: Option<String>,
}

impl BrandResponse {
    /// Convert a domain [`Brand`](sakaloka_core::models::brand::Brand)
    /// model into a response DTO.
    pub fn from_model(model: &sakaloka_core::models::brand::Brand) -> Self {
        Self {
            id: record_id_to_string(&model.id),
            name: model.name.clone(),
            slug: model.slug.clone(),
            description: model.description.clone(),
            logo: model.logo.clone(),
            website: model.website.clone(),
            is_active: model.is_active,
            organization_id: record_id_to_string(&model.organization_id),
            created_by: model.created_by.as_ref().map(record_id_to_string),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
