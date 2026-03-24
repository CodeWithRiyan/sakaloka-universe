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
    /// Optional barcode (UPC, EAN, etc.).
    pub barcode: Option<String>,
    /// Base selling price in smallest currency unit.
    pub base_price: i64,
    /// Cost/purchase price in smallest currency unit.
    pub cost_price: Option<i64>,
    /// Link to category.
    #[schema(value_type = Option<String>)]
    pub category_id: Option<Thing>,
    /// Link to brand.
    #[schema(value_type = Option<String>)]
    pub brand_id: Option<Thing>,
    /// Product image URL.
    pub image_url: Option<String>,
    /// Weight in kilograms.
    pub weight: Option<f64>,
    /// Dimensions as JSON object.
    pub dimensions: Option<serde_json::Value>,
    /// Whether inventory tracking is enabled.
    pub track_inventory: bool,
    /// Minimum stock level before low-stock alerts.
    pub min_stock_level: i64,
    /// Whether the product is featured/promoted.
    pub is_featured: bool,
    /// Tags for search and filtering.
    pub tags: Option<Vec<String>>,
    /// Owning organization.
    #[schema(value_type = String)]
    pub organization_id: Thing,
    /// User who created this product.
    #[schema(value_type = Option<String>)]
    pub created_by: Option<Thing>,
    /// Soft-delete timestamp.
    #[schema(value_type = Option<String>)]
    pub deleted_at: Option<Datetime>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}
