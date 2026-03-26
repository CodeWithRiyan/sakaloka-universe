//! Shared helpers local to the POS controller.

/// A resolved order line item with named fields.
pub struct ResolvedOrderItem {
    /// The product ID string.
    pub product_id: String,
    /// Display name for the line item.
    pub item_name: String,
    /// Quantity ordered.
    pub quantity: i32,
    /// Price per unit.
    pub unit_price: i64,
    /// Line total (`unit_price * quantity`).
    pub total_price: i64,
}

/// Generate a unique order number with the format `ORD-YYYYMMDD-XXXXXXXX`.
pub fn generate_order_number() -> String {
    let date = chrono::Utc::now().format("%Y%m%d");
    let suffix = &uuid::Uuid::new_v4().to_string()[..8];
    format!("ORD-{date}-{suffix}")
}
