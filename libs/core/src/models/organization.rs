use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId as Thing};
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

/// Represents an Organization record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct Organization {
    /// The record ID (e.g. `organization:01J...`).
    #[schema(value_type = String)]
    pub id: Thing,
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
    #[schema(value_type = Option<String>)]
    pub parent_id: Option<Thing>,
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
    #[schema(value_type = Option<String>)]
    pub owner_id: Option<Thing>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}
