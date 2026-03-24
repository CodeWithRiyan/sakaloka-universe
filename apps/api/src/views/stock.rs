//! Stock / Inventory view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::record_id_to_string;

/// Full stock (inventory item) response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StockResponse {
    /// Inventory item ID.
    pub id: String,
    /// Product ID.
    pub product_id: String,
    /// Organization ID.
    pub organization_id: String,
    /// Optional SKU override at inventory level.
    pub sku: Option<String>,
    /// Optional storage location identifier.
    pub location: Option<String>,
    /// Quantity currently on hand.
    pub quantity_on_hand: i32,
    /// Quantity reserved for pending orders.
    pub quantity_reserved: i32,
    /// Quantity available for sale.
    pub quantity_available: i32,
    /// Minimum stock level threshold.
    pub min_stock_level: i32,
    /// Maximum stock level.
    pub max_stock_level: Option<i32>,
    /// Reorder point.
    pub reorder_point: Option<i32>,
    /// Record creation timestamp.
    pub created_at: String,
    /// Record last-update timestamp.
    pub updated_at: String,
}

/// Stock movement history entry response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StockHistoryResponse {
    /// Movement ID.
    pub id: String,
    /// Inventory item ID.
    pub inventory_item_id: String,
    /// Type of movement (purchase, sale, adjustment, etc.).
    pub movement_type: String,
    /// Quantity moved.
    pub quantity: i32,
    /// Optional reference type (order, purchase_order).
    pub reference_type: Option<String>,
    /// Optional reference document ID.
    pub reference_id: Option<String>,
    /// Optional notes.
    pub notes: Option<String>,
    /// User who created the movement.
    pub created_by: Option<String>,
    /// Record creation timestamp.
    pub created_at: String,
}

/// Request body for adjusting stock levels.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdjustStockRequest {
    /// Type of adjustment (purchase, sale, adjustment, damage, return).
    pub movement_type: String,
    /// Signed quantity to adjust (positive = add, negative = remove).
    pub quantity: i32,
    /// Reason for the adjustment.
    pub reason: String,
    /// Optional reference string.
    pub reference: Option<String>,
    /// Optional notes.
    pub notes: Option<String>,
}

impl StockResponse {
    /// Convert a domain
    /// [`InventoryItem`](sakaloka_core::models::inventory::InventoryItem)
    /// model into a response DTO.
    pub fn from_model(model: &sakaloka_core::models::inventory::InventoryItem) -> Self {
        Self {
            id: record_id_to_string(&model.id),
            product_id: record_id_to_string(&model.product_id),
            organization_id: record_id_to_string(&model.organization_id),
            sku: model.sku.clone(),
            location: model.location.clone(),
            quantity_on_hand: model.quantity_on_hand,
            quantity_reserved: model.quantity_reserved,
            quantity_available: model.quantity_available,
            min_stock_level: model.min_stock_level,
            max_stock_level: model.max_stock_level,
            reorder_point: model.reorder_point,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

impl StockHistoryResponse {
    /// Convert a domain
    /// [`StockMovement`](sakaloka_core::models::inventory::StockMovement)
    /// model into a response DTO.
    pub fn from_model(model: &sakaloka_core::models::inventory::StockMovement) -> Self {
        Self {
            id: record_id_to_string(&model.id),
            inventory_item_id: record_id_to_string(&model.inventory_item_id),
            movement_type: model.movement_type.clone(),
            quantity: model.quantity,
            reference_type: model.reference_type.clone(),
            reference_id: model.reference_id.clone(),
            notes: model.notes.clone(),
            created_by: model.created_by.as_ref().map(record_id_to_string),
            created_at: model.created_at.to_string(),
        }
    }
}
