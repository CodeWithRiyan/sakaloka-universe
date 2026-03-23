//! SeaORM entity for the `orders` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Order entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Unique order number.
    #[sea_orm(unique)]
    pub order_number: String,
    /// Optional customer user ID.
    pub customer_id: Option<Uuid>,
    /// Organization this order belongs to.
    pub organization_id: Uuid,
    /// Order status (e.g. pending, completed, cancelled).
    pub status: String,
    /// Order type (e.g. dine_in, takeaway, delivery).
    pub r#type: String,
    /// Subtotal before tax and discounts.
    pub subtotal: i64,
    /// Tax amount.
    pub tax_amount: i64,
    /// Discount amount.
    pub discount_amount: i64,
    /// Total amount after tax and discounts.
    pub total_amount: i64,
    /// Optional payment method.
    pub payment_method: Option<String>,
    /// Payment status (e.g. unpaid, paid, refunded).
    pub payment_status: String,
    /// Amount already paid.
    pub paid_amount: i64,
    /// Optional order notes.
    pub notes: Option<String>,
    /// Optional table number for dine-in orders.
    pub table_number: Option<String>,
    /// Optional customer name for walk-in customers.
    pub customer_name: Option<String>,
    /// User who created this order.
    pub created_by: Option<Uuid>,
    /// User who last updated this order.
    pub updated_by: Option<Uuid>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
    /// Timestamp when the record was last updated.
    pub updated_at: DateTimeWithTimeZone,
}

/// Relations for the orders entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// An order belongs to an organization.
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id"
    )]
    Organization,
    /// An order has many order items.
    #[sea_orm(has_many = "super::order_items::Entity")]
    OrderItems,
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::order_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
