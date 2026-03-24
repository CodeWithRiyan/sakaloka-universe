//! Product view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::record_id_to_string;

/// Abbreviated category info embedded in product responses.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategorySummary {
    /// Category ID.
    pub id: String,
    /// Category name.
    pub name: String,
}

/// Abbreviated brand info embedded in product responses.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BrandSummary {
    /// Brand ID.
    pub id: String,
    /// Brand name.
    pub name: String,
}

/// Variant count for list endpoints.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VariantCount {
    /// Number of variants.
    pub variants: i64,
}

/// Full product response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProductResponse {
    /// Product ID.
    pub id: String,
    /// Product name.
    pub name: String,
    /// Optional product description.
    pub description: Option<String>,
    /// Stock keeping unit.
    pub sku: String,
    /// Optional barcode.
    pub barcode: Option<String>,
    /// Base selling price in smallest currency unit.
    pub base_price: i64,
    /// Optional cost price in smallest currency unit.
    pub cost_price: Option<i64>,
    /// Category ID.
    pub category_id: Option<String>,
    /// Brand ID.
    pub brand_id: Option<String>,
    /// Image URL.
    pub image_url: Option<String>,
    /// Weight in kilograms.
    pub weight: Option<f64>,
    /// Dimensions as JSON.
    pub dimensions: Option<serde_json::Value>,
    /// Whether inventory tracking is enabled.
    pub track_inventory: bool,
    /// Minimum stock level threshold.
    pub min_stock_level: i64,
    /// Whether the product is featured.
    pub is_featured: bool,
    /// Tags for search and filtering.
    pub tags: Option<Vec<String>>,
    /// Owning organization ID.
    pub organization_id: String,
    /// Soft-delete timestamp.
    pub deleted_at: Option<String>,
    /// Record creation timestamp.
    pub created_at: String,
    /// Record last-update timestamp.
    pub updated_at: String,
    /// Resolved category summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CategorySummary>,
    /// Resolved brand summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<BrandSummary>,
}

/// Product list response DTO (includes variant count).
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProductListResponse {
    /// Product ID.
    pub id: String,
    /// Product name.
    pub name: String,
    /// Optional product description.
    pub description: Option<String>,
    /// Stock keeping unit.
    pub sku: String,
    /// Optional barcode.
    pub barcode: Option<String>,
    /// Base selling price.
    pub base_price: i64,
    /// Cost price.
    pub cost_price: Option<i64>,
    /// Category ID.
    pub category_id: Option<String>,
    /// Brand ID.
    pub brand_id: Option<String>,
    /// Image URL.
    pub image_url: Option<String>,
    /// Whether the product is featured.
    pub is_featured: bool,
    /// Tags for search and filtering.
    pub tags: Option<Vec<String>>,
    /// Owning organization ID.
    pub organization_id: String,
    /// Record creation timestamp.
    pub created_at: String,
    /// Record last-update timestamp.
    pub updated_at: String,
    /// Resolved category summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CategorySummary>,
    /// Resolved brand summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<BrandSummary>,
    /// Variant count.
    #[serde(rename = "_count")]
    pub count: VariantCount,
}

/// Request body for creating a product.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductRequest {
    /// Product name.
    pub name: String,
    /// Stock keeping unit (unique within organization).
    pub sku: String,
    /// Base selling price in smallest currency unit.
    pub base_price: i64,
    /// Optional product description.
    pub description: Option<String>,
    /// Optional barcode.
    pub barcode: Option<String>,
    /// Optional cost price.
    pub cost_price: Option<i64>,
    /// Optional category ID.
    pub category_id: Option<String>,
    /// Optional brand ID.
    pub brand_id: Option<String>,
    /// Optional image URL.
    pub image_url: Option<String>,
    /// Optional weight in kilograms.
    pub weight: Option<f64>,
    /// Optional dimensions as JSON.
    pub dimensions: Option<serde_json::Value>,
    /// Whether to track inventory (default false).
    pub track_inventory: Option<bool>,
    /// Optional minimum stock level.
    pub min_stock_level: Option<i64>,
    /// Whether the product is featured (default false).
    pub is_featured: Option<bool>,
    /// Optional tags for search and filtering.
    pub tags: Option<Vec<String>>,
}

/// Request body for updating a product (all fields optional).
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProductRequest {
    /// Updated product name.
    pub name: Option<String>,
    /// Updated SKU.
    pub sku: Option<String>,
    /// Updated base price.
    pub base_price: Option<i64>,
    /// Updated description.
    pub description: Option<String>,
    /// Updated barcode.
    pub barcode: Option<String>,
    /// Updated cost price.
    pub cost_price: Option<i64>,
    /// Updated category ID.
    pub category_id: Option<String>,
    /// Updated brand ID.
    pub brand_id: Option<String>,
    /// Updated image URL.
    pub image_url: Option<String>,
    /// Updated weight.
    pub weight: Option<f64>,
    /// Updated dimensions.
    pub dimensions: Option<serde_json::Value>,
    /// Updated track_inventory flag.
    pub track_inventory: Option<bool>,
    /// Updated minimum stock level.
    pub min_stock_level: Option<i64>,
    /// Updated is_featured flag.
    pub is_featured: Option<bool>,
    /// Updated tags.
    pub tags: Option<Vec<String>>,
}

impl ProductResponse {
    /// Convert a domain [`Product`](sakaloka_core::models::product::Product)
    /// model into a response DTO.
    pub fn from_model(
        model: &sakaloka_core::models::product::Product,
        category: Option<CategorySummary>,
        brand: Option<BrandSummary>,
    ) -> Self {
        Self {
            id: record_id_to_string(&model.id),
            name: model.name.clone(),
            description: model.description.clone(),
            sku: model.sku.clone(),
            barcode: model.barcode.clone(),
            base_price: model.base_price,
            cost_price: model.cost_price,
            category_id: model.category_id.as_ref().map(record_id_to_string),
            brand_id: model.brand_id.as_ref().map(record_id_to_string),
            image_url: model.image_url.clone(),
            weight: model.weight,
            dimensions: model.dimensions.clone(),
            track_inventory: model.track_inventory,
            min_stock_level: model.min_stock_level,
            is_featured: model.is_featured,
            tags: model.tags.clone(),
            organization_id: record_id_to_string(&model.organization_id),
            deleted_at: model.deleted_at.as_ref().map(|dt| dt.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            category,
            brand,
        }
    }
}
