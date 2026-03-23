//! Stock / Inventory view DTOs.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::_entities::{inventory_items, stock_movements};

/// Full stock (inventory item) response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StockResponse {
    /// Inventory item ID.
    pub id: Uuid,
    /// Product ID.
    pub product_id: Uuid,
    /// Optional storage location ID.
    pub location_id: Option<Uuid>,
    /// Quantity currently on hand.
    pub quantity_on_hand: i32,
    /// Quantity reserved for pending orders.
    pub quantity_reserved: i32,
    /// Quantity available for sale.
    pub quantity_available: i32,
    /// Minimum stock level threshold.
    pub min_stock_level: Option<i32>,
    /// Maximum stock level.
    pub max_stock_level: Option<i32>,
    /// Reorder point.
    pub reorder_point: Option<i32>,
    /// Reorder quantity.
    pub reorder_quantity: Option<i32>,
    /// Average cost per unit.
    pub average_cost: Option<i64>,
    /// Last purchase cost per unit.
    pub last_cost: Option<i64>,
    /// Organization ID.
    pub organization_id: Uuid,
    /// Timestamp of last stock movement.
    pub last_movement_at: Option<DateTime<FixedOffset>>,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
    /// Record last-update timestamp.
    pub updated_at: DateTime<FixedOffset>,
}

/// Stock movement history entry response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StockHistoryResponse {
    /// Movement ID.
    pub id: Uuid,
    /// Inventory item ID.
    pub inventory_item_id: Uuid,
    /// Type of movement (purchase, sale, adjustment, etc.).
    pub movement_type: String,
    /// Quantity moved.
    pub quantity: i32,
    /// Reason for the movement.
    pub reason: String,
    /// Optional reference (order number, PO, etc.).
    pub reference: Option<String>,
    /// Optional notes.
    pub notes: Option<String>,
    /// Inventory before this movement.
    pub total_before: i32,
    /// Signed quantity change.
    pub quantity_change: i32,
    /// Inventory after this movement.
    pub total_after: i32,
    /// User who created the movement.
    pub created_by: Option<Uuid>,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
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
    /// Convert a SeaORM inventory item model into a response DTO.
    pub fn from_model(model: inventory_items::Model) -> Self {
        Self {
            id: model.id,
            product_id: model.product_id,
            location_id: model.location_id,
            quantity_on_hand: model.quantity_on_hand,
            quantity_reserved: model.quantity_reserved,
            quantity_available: model.quantity_available,
            min_stock_level: model.min_stock_level,
            max_stock_level: model.max_stock_level,
            reorder_point: model.reorder_point,
            reorder_quantity: model.reorder_quantity,
            average_cost: model.average_cost,
            last_cost: model.last_cost,
            organization_id: model.organization_id,
            last_movement_at: model.last_movement_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl StockHistoryResponse {
    /// Convert a SeaORM stock movement model into a response DTO.
    pub fn from_model(model: stock_movements::Model) -> Self {
        Self {
            id: model.id,
            inventory_item_id: model.inventory_item_id,
            movement_type: model.movement_type,
            quantity: model.quantity,
            reason: model.reason,
            reference: model.reference,
            notes: model.notes,
            total_before: model.total_before,
            quantity_change: model.quantity_change,
            total_after: model.total_after,
            created_by: model.created_by,
            created_at: model.created_at,
        }
    }
}
