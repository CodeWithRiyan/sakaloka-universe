use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId as Thing};
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

/// Represents a Product record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct Product {
    /// The record ID (e.g. `product:01J...`).
    #[schema(value_type = String)]
    pub id: Thing,
    /// The name of the product.
    pub name: String,
    /// A detailed description.
    pub description: Option<String>,
    /// Unique stock-keeping unit.
    pub sku: String,
    /// Price in cents (integer).
    pub price: u64,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}
