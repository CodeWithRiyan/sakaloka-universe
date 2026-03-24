use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId as Thing};
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

/// Represents an InventoryItem record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct InventoryItem {
    /// The record ID.
    #[schema(value_type = String)]
    pub id: Thing,
    /// Link to the product.
    #[schema(value_type = String)]
    pub product_id: Thing,
    /// Owning organization.
    #[schema(value_type = String)]
    pub organization_id: Thing,
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
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}

/// Represents a StockMovement record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct StockMovement {
    /// The record ID.
    #[schema(value_type = String)]
    pub id: Thing,
    /// Link to the inventory item.
    #[schema(value_type = String)]
    pub inventory_item_id: Thing,
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
    #[schema(value_type = Option<String>)]
    pub created_by: Option<Thing>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
}
