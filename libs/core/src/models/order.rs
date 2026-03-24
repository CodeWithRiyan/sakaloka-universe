use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId as Thing};
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

/// Represents an Order record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct Order {
    /// The record ID (e.g. `order:01J...`).
    #[schema(value_type = String)]
    pub id: Thing,
    /// Unique order number (e.g. `ORD-20260101-0001`).
    pub order_number: String,
    /// Optional link to a registered customer.
    #[schema(value_type = Option<String>)]
    pub customer_id: Option<Thing>,
    /// Owning organization.
    #[schema(value_type = String)]
    pub organization_id: Thing,
    /// Order lifecycle status.
    pub status: String,
    /// Order type (dine_in, takeaway, delivery).
    #[serde(rename = "type")]
    pub order_type: String,
    /// Subtotal before tax/discounts.
    pub subtotal: i64,
    /// Tax amount.
    pub tax_amount: i64,
    /// Discount amount.
    pub discount_amount: i64,
    /// Total amount after tax and discounts.
    pub total_amount: i64,
    /// Payment method.
    pub payment_method: Option<String>,
    /// Payment status.
    pub payment_status: String,
    /// Amount already paid.
    pub paid_amount: i64,
    /// Order notes / special instructions.
    pub notes: Option<String>,
    /// Table number for dine-in orders.
    pub table_number: Option<String>,
    /// Walk-in customer name.
    pub customer_name: Option<String>,
    /// User who created this order.
    #[schema(value_type = Option<String>)]
    pub created_by: Option<Thing>,
    /// User who last updated this order.
    #[schema(value_type = Option<String>)]
    pub updated_by: Option<Thing>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}

/// Represents an OrderItem record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct OrderItem {
    /// The record ID.
    #[schema(value_type = String)]
    pub id: Thing,
    /// Link to the parent order.
    #[schema(value_type = String)]
    pub order_id: Thing,
    /// Link to the product.
    #[schema(value_type = String)]
    pub product_id: Thing,
    /// Snapshot of the product name at time of order.
    pub item_name: String,
    /// Quantity ordered.
    pub quantity: i32,
    /// Unit price at time of order.
    pub unit_price: i64,
    /// Discount applied to this line item.
    pub discount_amount: i64,
    /// Total price for this line.
    pub total_price: i64,
    /// Optional item-level notes.
    pub notes: Option<String>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
}
