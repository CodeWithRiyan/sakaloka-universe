//! Order view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Full order response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    /// Order ID.
    pub id: String,
    /// Unique order number.
    pub order_number: String,
    /// Optional customer user ID.
    pub customer_id: Option<String>,
    /// Organization ID.
    pub organization_id: String,
    /// Order status.
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
    /// Order notes.
    pub notes: Option<String>,
    /// Table number for dine-in orders.
    pub table_number: Option<String>,
    /// Walk-in customer name.
    pub customer_name: Option<String>,
    /// User who created this order.
    pub created_by: Option<String>,
    /// User who last updated this order.
    pub updated_by: Option<String>,
    /// Record creation timestamp.
    pub created_at: String,
    /// Record last-update timestamp.
    pub updated_at: String,
    /// Order items (populated when fetching a single order).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderItemResponse>>,
}

/// Single order item response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderItemResponse {
    /// Order item ID.
    pub id: String,
    /// Parent order ID.
    pub order_id: String,
    /// Product ID.
    pub product_id: String,
    /// Item name snapshot.
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
    /// Record creation timestamp.
    pub created_at: String,
}

/// POS menu item DTO (product + pricing for the order screen).
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MenuResponse {
    /// Product ID.
    pub id: String,
    /// Product name.
    pub name: String,
    /// Product SKU.
    pub sku: String,
    /// Base selling price.
    pub base_price: i64,
    /// Image URL.
    pub image_url: Option<String>,
    /// Category ID.
    pub category_id: Option<String>,
    /// Category name (if resolved).
    pub category_name: Option<String>,
    /// Whether the product is featured.
    pub is_featured: bool,
}

/// Order item within a create order request.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderItemRequest {
    /// Product ID.
    pub product_id: String,
    /// Quantity to order.
    pub quantity: i32,
    /// Optional custom item name override.
    pub item_name: Option<String>,
}

/// Request body for creating an order.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    /// Order type (dine_in, takeaway, delivery).
    #[serde(rename = "type")]
    pub order_type: String,
    /// Order items.
    pub items: Vec<CreateOrderItemRequest>,
    /// Optional table number.
    pub table_number: Option<String>,
    /// Optional walk-in customer name.
    pub customer_name: Option<String>,
    /// Optional notes.
    pub notes: Option<String>,
    /// Optional payment method.
    pub payment_method: Option<String>,
}

/// Request body for updating an order.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderRequest {
    /// Updated status.
    pub status: Option<String>,
    /// Updated payment method.
    pub payment_method: Option<String>,
    /// Updated payment status.
    pub payment_status: Option<String>,
    /// Amount paid (for partial payments).
    pub paid_amount: Option<i64>,
    /// Updated notes.
    pub notes: Option<String>,
    /// Updated table number.
    pub table_number: Option<String>,
    /// Updated customer name.
    pub customer_name: Option<String>,
}

impl OrderResponse {
    /// Convert a domain [`Order`](sakaloka_core::models::order::Order) model
    /// into a response DTO (without items).
    pub fn from_model(model: &sakaloka_core::models::order::Order) -> Self {
        Self {
            id: model.id.clone(),
            order_number: model.order_number.clone(),
            customer_id: model.customer_id.clone(),
            organization_id: model.organization_id.clone(),
            status: model.status.clone(),
            order_type: model.order_type.clone(),
            subtotal: model.subtotal,
            tax_amount: model.tax_amount,
            discount_amount: model.discount_amount,
            total_amount: model.total_amount,
            payment_method: model.payment_method.clone(),
            payment_status: model.payment_status.clone(),
            paid_amount: model.paid_amount,
            notes: model.notes.clone(),
            table_number: model.table_number.clone(),
            customer_name: model.customer_name.clone(),
            created_by: model.created_by.clone(),
            updated_by: model.updated_by.clone(),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            items: None,
        }
    }

    /// Convert a domain [`Order`](sakaloka_core::models::order::Order) model
    /// into a response DTO with its associated
    /// [`OrderItem`](sakaloka_core::models::order::OrderItem) list.
    pub fn from_model_with_items(
        model: &sakaloka_core::models::order::Order,
        items: &[sakaloka_core::models::order::OrderItem],
    ) -> Self {
        let item_responses = items.iter().map(OrderItemResponse::from_model).collect();
        let mut resp = Self::from_model(model);
        resp.items = Some(item_responses);
        resp
    }
}

impl OrderItemResponse {
    /// Convert a domain
    /// [`OrderItem`](sakaloka_core::models::order::OrderItem) model into a
    /// response DTO.
    pub fn from_model(model: &sakaloka_core::models::order::OrderItem) -> Self {
        Self {
            id: model.id.clone(),
            order_id: model.order_id.clone(),
            product_id: model.product_id.clone(),
            item_name: model.item_name.clone(),
            quantity: model.quantity,
            unit_price: model.unit_price,
            discount_amount: model.discount_amount,
            total_price: model.total_price,
            notes: model.notes.clone(),
            created_at: model.created_at.to_string(),
        }
    }
}
