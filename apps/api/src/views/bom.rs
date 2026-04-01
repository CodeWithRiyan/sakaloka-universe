//! Bill of Materials (BOM) view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// BOM response for GET endpoints.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BomResponse {
    /// Unique identifier.
    pub id: String,
    /// Name of the BOM.
    pub name: String,
    /// Product ID this BOM belongs to.
    pub product_id: String,
    /// Organization ID.
    pub organization_id: String,
    /// Version string.
    pub version: String,
    /// Status (draft, active, archived).
    pub status: String,
    /// Effective from date.
    pub effective_from: Option<String>,
    /// Effective to date.
    pub effective_to: Option<String>,
    /// Total cost in smallest currency unit.
    pub total_cost: Option<i64>,
    /// Optional notes.
    pub notes: Option<String>,
    /// User who created this BOM.
    pub created_by: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// BOM item response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BomItemResponse {
    /// Unique identifier.
    pub id: String,
    /// Parent BOM ID.
    pub bom_id: String,
    /// Component product ID.
    pub component_product_id: String,
    /// Quantity needed.
    pub quantity: f64,
    /// Unit of measure.
    pub unit: String,
    /// Waste percentage.
    pub waste_percent: f64,
    /// Yield percentage.
    pub yield_percent: f64,
    /// Unit cost snapshot.
    pub unit_cost: Option<i64>,
    /// Line cost (qty × unit_cost).
    pub line_cost: Option<i64>,
    /// Optional notes.
    pub notes: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// BOM with its items combined.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BomWithItemsResponse {
    /// The BOM fields.
    #[serde(flatten)]
    pub bom: BomResponse,
    /// List of BOM items.
    pub items: Vec<BomItemResponse>,
}

/// Production run response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProductionRunResponse {
    /// Unique identifier.
    pub id: String,
    /// BOM ID.
    pub bom_id: String,
    /// Organization ID.
    pub organization_id: String,
    /// Quantity produced.
    pub quantity_produced: f64,
    /// Estimated cost.
    pub estimated_cost: Option<i64>,
    /// Actual cost.
    pub actual_cost: Option<i64>,
    /// Status.
    pub status: String,
    /// Started timestamp.
    pub started_at: Option<String>,
    /// Completed timestamp.
    pub completed_at: Option<String>,
    /// User who created this run.
    pub created_by: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Consumption record response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionResponse {
    /// Unique identifier.
    pub id: String,
    /// Production run ID.
    pub production_run_id: String,
    /// Component product ID.
    pub component_product_id: String,
    /// Planned quantity.
    pub planned_quantity: f64,
    /// Actual quantity consumed.
    pub actual_quantity: f64,
    /// Unit of measure.
    pub unit: String,
    /// Waste quantity.
    pub waste_quantity: f64,
    /// Creation timestamp.
    pub created_at: String,
}

/// Production run with consumptions.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProductionRunWithConsumptionResponse {
    /// The production run fields.
    #[serde(flatten)]
    pub run: ProductionRunResponse,
    /// List of consumption records.
    pub consumptions: Vec<ConsumptionResponse>,
}

/// Request to create a BOM.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBomRequest {
    /// Name of the BOM.
    pub name: String,
    /// Product ID this BOM belongs to.
    pub product_id: String,
    /// Version string (optional, defaults to "1.0.0").
    pub version: Option<String>,
    /// Status (optional, defaults to "draft").
    pub status: Option<String>,
    /// Effective from date (ISO 8601).
    pub effective_from: Option<String>,
    /// Effective to date (ISO 8601).
    pub effective_to: Option<String>,
    /// Optional notes.
    pub notes: Option<String>,
}

/// Request to update a BOM.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBomRequest {
    /// Name of the BOM.
    pub name: Option<String>,
    /// Version string.
    pub version: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Effective from date.
    pub effective_from: Option<String>,
    /// Effective to date.
    pub effective_to: Option<String>,
    /// Total cost.
    pub total_cost: Option<i64>,
    /// Optional notes.
    pub notes: Option<String>,
}

/// Request to create a BOM item.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBomItemRequest {
    /// Component product ID.
    pub component_product_id: String,
    /// Quantity needed.
    pub quantity: f64,
    /// Unit of measure.
    pub unit: String,
    /// Waste percentage (optional, defaults to 0).
    pub waste_percent: Option<f64>,
    /// Yield percentage (optional, defaults to 100).
    pub yield_percent: Option<f64>,
    /// Optional notes.
    pub notes: Option<String>,
}

/// Request to update a BOM item.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBomItemRequest {
    /// Quantity needed.
    pub quantity: Option<f64>,
    /// Unit of measure.
    pub unit: Option<String>,
    /// Waste percentage.
    pub waste_percent: Option<f64>,
    /// Yield percentage.
    pub yield_percent: Option<f64>,
    /// Optional notes.
    pub notes: Option<String>,
}

/// Request to create a production run.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductionRunRequest {
    /// Quantity to produce.
    pub quantity_produced: f64,
    /// Estimated cost (optional).
    pub estimated_cost: Option<i64>,
    /// Status (optional, defaults to "planned").
    pub status: Option<String>,
    /// Started timestamp (ISO 8601).
    pub started_at: Option<String>,
    /// Completed timestamp (ISO 8601).
    pub completed_at: Option<String>,
}

/// Request to update a production run.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProductionRunRequest {
    /// Quantity produced.
    pub quantity_produced: Option<f64>,
    /// Estimated cost.
    pub estimated_cost: Option<i64>,
    /// Actual cost.
    pub actual_cost: Option<i64>,
    /// Status.
    pub status: Option<String>,
    /// Started timestamp.
    pub started_at: Option<String>,
    /// Completed timestamp.
    pub completed_at: Option<String>,
}

/// Request to create a consumption record.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateConsumptionRequest {
    /// Component product ID.
    pub component_product_id: String,
    /// Planned quantity.
    pub planned_quantity: f64,
    /// Actual quantity consumed.
    pub actual_quantity: f64,
    /// Unit of measure.
    pub unit: String,
    /// Waste quantity (optional, defaults to 0).
    pub waste_quantity: Option<f64>,
}

/// Cost breakdown response for a BOM.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BomCostBreakdownResponse {
    /// BOM ID.
    pub bom_id: String,
    /// Total cost.
    pub total_cost: i64,
    /// Item cost details.
    pub items: Vec<BomItemCostResponse>,
}

/// Item cost details in a BOM cost breakdown.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BomItemCostResponse {
    /// Component product ID.
    pub component_product_id: String,
    /// Quantity.
    pub quantity: f64,
    /// Unit of measure.
    pub unit: String,
    /// Unit cost.
    pub unit_cost: i64,
    /// Line cost.
    pub line_cost: i64,
    /// Waste percentage.
    pub waste_percent: f64,
}
