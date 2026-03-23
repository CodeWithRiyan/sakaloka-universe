//! Organization view DTOs.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::_entities::organizations;

/// Full organization response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationResponse {
    /// Organization ID.
    pub id: Uuid,
    /// Organization name.
    pub name: String,
    /// Organization type.
    #[serde(rename = "type")]
    pub org_type: String,
    /// Optional organization code.
    pub code: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional parent organization ID.
    pub parent_id: Option<Uuid>,
    /// Optional contact email.
    pub email: Option<String>,
    /// Optional contact phone.
    pub phone: Option<String>,
    /// Optional website URL.
    pub website: Option<String>,
    /// Optional physical address.
    pub address: Option<String>,
    /// Optional city.
    pub city: Option<String>,
    /// Optional state/province.
    pub state: Option<String>,
    /// Optional country.
    pub country: Option<String>,
    /// Optional postal code.
    pub postal_code: Option<String>,
    /// Optional tax identification number.
    pub tax_number: Option<String>,
    /// Optional business registration number.
    pub registration_number: Option<String>,
    /// Optional logo URL.
    pub logo: Option<String>,
    /// Optional JSON settings.
    pub settings: Option<serde_json::Value>,
    /// Whether the organization is active.
    pub is_active: bool,
    /// Optional owner user ID.
    pub owner_id: Option<Uuid>,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
    /// Record last-update timestamp.
    pub updated_at: DateTime<FixedOffset>,
}

/// Request body for creating an organization.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrgRequest {
    /// Organization name.
    pub name: String,
    /// Organization type (e.g. "restaurant", "retail").
    #[serde(rename = "type")]
    pub org_type: String,
    /// Optional organization code.
    pub code: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional parent organization ID.
    pub parent_id: Option<Uuid>,
    /// Optional contact email.
    pub email: Option<String>,
    /// Optional contact phone.
    pub phone: Option<String>,
    /// Optional website URL.
    pub website: Option<String>,
    /// Optional physical address.
    pub address: Option<String>,
    /// Optional city.
    pub city: Option<String>,
    /// Optional state/province.
    pub state: Option<String>,
    /// Optional country.
    pub country: Option<String>,
    /// Optional postal code.
    pub postal_code: Option<String>,
    /// Optional tax identification number.
    pub tax_number: Option<String>,
    /// Optional business registration number.
    pub registration_number: Option<String>,
    /// Optional logo URL.
    pub logo: Option<String>,
    /// Optional JSON settings.
    pub settings: Option<serde_json::Value>,
}

/// Request body for updating an organization (all fields optional).
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrgRequest {
    /// Updated organization name.
    pub name: Option<String>,
    /// Updated organization type.
    #[serde(rename = "type")]
    pub org_type: Option<String>,
    /// Updated organization code.
    pub code: Option<String>,
    /// Updated description.
    pub description: Option<String>,
    /// Updated contact email.
    pub email: Option<String>,
    /// Updated contact phone.
    pub phone: Option<String>,
    /// Updated website URL.
    pub website: Option<String>,
    /// Updated physical address.
    pub address: Option<String>,
    /// Updated city.
    pub city: Option<String>,
    /// Updated state/province.
    pub state: Option<String>,
    /// Updated country.
    pub country: Option<String>,
    /// Updated postal code.
    pub postal_code: Option<String>,
    /// Updated tax identification number.
    pub tax_number: Option<String>,
    /// Updated business registration number.
    pub registration_number: Option<String>,
    /// Updated logo URL.
    pub logo: Option<String>,
    /// Updated JSON settings.
    pub settings: Option<serde_json::Value>,
    /// Updated active flag.
    pub is_active: Option<bool>,
}

/// Request body to select / switch to a different organization.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectOrgRequest {
    /// ID of the organization to switch to.
    pub organization_id: Uuid,
}

impl OrganizationResponse {
    /// Convert a SeaORM organization model into a response DTO.
    pub fn from_model(model: organizations::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            org_type: model.r#type,
            code: model.code,
            description: model.description,
            parent_id: model.parent_id,
            email: model.email,
            phone: model.phone,
            website: model.website,
            address: model.address,
            city: model.city,
            state: model.state,
            country: model.country,
            postal_code: model.postal_code,
            tax_number: model.tax_number,
            registration_number: model.registration_number,
            logo: model.logo,
            settings: model.settings,
            is_active: model.is_active,
            owner_id: model.owner_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
