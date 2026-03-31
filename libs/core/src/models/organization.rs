use serde::{Deserialize, Serialize};

/// Represents an Organization record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Organization {
    /// The record ID.
    pub id: String,
    /// Organization name.
    pub name: String,
    /// Organization type (company, branch, warehouse).
    #[serde(alias = "type")]
    pub org_type: String,
    /// Optional short code identifier.
    pub code: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Parent organization for hierarchy.
    pub parent_id: Option<String>,
    /// Contact email.
    pub email: Option<String>,
    /// Contact phone.
    pub phone: Option<String>,
    /// Website URL.
    pub website: Option<String>,
    /// Physical address.
    pub address: Option<String>,
    /// City.
    pub city: Option<String>,
    /// State/province.
    pub state: Option<String>,
    /// Country.
    pub country: Option<String>,
    /// Postal code.
    pub postal_code: Option<String>,
    /// Tax identification number.
    pub tax_number: Option<String>,
    /// Business registration number.
    pub registration_number: Option<String>,
    /// Logo URL.
    pub logo: Option<String>,
    /// Flexible settings object.
    pub settings: Option<serde_json::Value>,
    /// Whether the organization is active.
    pub is_active: bool,
    /// Link to the owning user.
    pub owner_id: Option<String>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
