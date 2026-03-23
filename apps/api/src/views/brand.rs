//! Brand view DTOs.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::_entities::brands;

/// Full brand response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BrandResponse {
    /// Brand ID.
    pub id: Uuid,
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
    /// Owning organization ID.
    pub organization_id: Uuid,
    /// User who created this brand.
    pub created_by: Option<Uuid>,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
    /// Record last-update timestamp.
    pub updated_at: DateTime<FixedOffset>,
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
    /// Convert a SeaORM brand model into a response DTO.
    pub fn from_model(model: brands::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            description: model.description,
            logo: model.logo,
            website: model.website,
            organization_id: model.organization_id,
            created_by: model.created_by,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
