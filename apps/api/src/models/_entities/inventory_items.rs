//! SeaORM entity for the `inventory_items` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Inventory item entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "inventory_items")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Product this inventory record tracks.
    pub product_id: Uuid,
    /// Optional storage location ID.
    pub location_id: Option<Uuid>,
    /// Current quantity on hand.
    pub quantity_on_hand: i32,
    /// Quantity reserved for pending orders.
    pub quantity_reserved: i32,
    /// Quantity available for sale.
    pub quantity_available: i32,
    /// Optional minimum stock level threshold.
    pub min_stock_level: Option<i32>,
    /// Optional maximum stock level.
    pub max_stock_level: Option<i32>,
    /// Optional reorder point.
    pub reorder_point: Option<i32>,
    /// Optional reorder quantity.
    pub reorder_quantity: Option<i32>,
    /// Optional average cost in smallest currency unit.
    pub average_cost: Option<i64>,
    /// Optional last cost in smallest currency unit.
    pub last_cost: Option<i64>,
    /// Organization this inventory item belongs to.
    pub organization_id: Uuid,
    /// Timestamp of last stock movement.
    pub last_movement_at: Option<DateTimeWithTimeZone>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
    /// Timestamp when the record was last updated.
    pub updated_at: DateTimeWithTimeZone,
}

/// Relations for the inventory items entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// An inventory item belongs to a product.
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id"
    )]
    Product,
    /// An inventory item belongs to an organization.
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id"
    )]
    Organization,
    /// An inventory item has many stock movements.
    #[sea_orm(has_many = "super::stock_movements::Entity")]
    StockMovements,
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::stock_movements::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StockMovements.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
