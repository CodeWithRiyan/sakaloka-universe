//! SeaORM entity for the `stock_movements` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Stock movement entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "stock_movements")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Inventory item this movement applies to.
    pub inventory_item_id: Uuid,
    /// Type of movement (e.g. purchase, sale, adjustment).
    pub movement_type: String,
    /// Quantity moved.
    pub quantity: i32,
    /// Reason for the movement.
    pub reason: String,
    /// Optional reference (e.g. order number, PO number).
    pub reference: Option<String>,
    /// Optional notes.
    pub notes: Option<String>,
    /// Total inventory before this movement.
    pub total_before: i32,
    /// Signed quantity change.
    pub quantity_change: i32,
    /// Total inventory after this movement.
    pub total_after: i32,
    /// User who created this movement.
    pub created_by: Option<Uuid>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
}

/// Relations for the stock movements entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// A stock movement belongs to an inventory item.
    #[sea_orm(
        belongs_to = "super::inventory_items::Entity",
        from = "Column::InventoryItemId",
        to = "super::inventory_items::Column::Id"
    )]
    InventoryItem,
}

impl Related<super::inventory_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InventoryItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
