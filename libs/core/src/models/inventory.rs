use serde::{Deserialize, Serialize};

/// Represents an InventoryItem record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct InventoryItem {
    /// The record ID.
    pub id: String,
    /// Link to the product.
    pub product_id: String,
    /// Owning organization.
    pub organization_id: String,
    /// Optional SKU override at inventory level.
    pub sku: Option<String>,
    /// Storage location identifier.
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
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Represents a StockMovement record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct StockMovement {
    /// The record ID.
    pub id: String,
    /// Link to the inventory item.
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
    /// User who performed this movement.
    pub created_by: Option<String>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}
