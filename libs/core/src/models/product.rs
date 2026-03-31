use serde::{Deserialize, Serialize};

/// Represents a Product record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Product {
    /// The record ID.
    pub id: String,
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
    pub category_id: Option<String>,
    /// Link to brand.
    pub brand_id: Option<String>,
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
    pub organization_id: String,
    /// User who created this product.
    pub created_by: Option<String>,
    /// Soft-delete timestamp.
    #[schema(value_type = Option<String>)]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
