use serde::{Deserialize, Serialize};

/// Represents an Order record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Order {
    /// The record ID.
    pub id: String,
    /// Unique order number (e.g. `ORD-20260101-0001`).
    pub order_number: String,
    /// Optional link to a registered customer.
    pub customer_id: Option<String>,
    /// Owning organization.
    pub organization_id: String,
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
    pub created_by: Option<String>,
    /// User who last updated this order.
    pub updated_by: Option<String>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Represents an OrderItem record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct OrderItem {
    /// The record ID.
    pub id: String,
    /// Link to the parent order.
    pub order_id: String,
    /// Link to the product.
    pub product_id: String,
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
    pub created_at: chrono::DateTime<chrono::Utc>,
}
