//! SeaORM entity for the `order_items` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Order item entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "order_items")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Order this item belongs to.
    pub order_id: Uuid,
    /// Product referenced by this item.
    pub product_id: Uuid,
    /// Snapshot of the item name at time of order.
    pub item_name: String,
    /// Quantity ordered.
    pub quantity: i32,
    /// Unit price at time of order.
    pub unit_price: i64,
    /// Total price (quantity * unit_price).
    pub total_price: i64,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
}

/// Relations for the order items entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// An order item belongs to an order.
    #[sea_orm(
        belongs_to = "super::orders::Entity",
        from = "Column::OrderId",
        to = "super::orders::Column::Id"
    )]
    Order,
    /// An order item belongs to a product.
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id"
    )]
    Product,
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
