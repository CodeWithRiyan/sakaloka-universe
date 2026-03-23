//! Order view DTOs.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::_entities::{order_items, orders};

/// Full order response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    /// Order ID.
    pub id: Uuid,
    /// Unique order number.
    pub order_number: String,
    /// Optional customer user ID.
    pub customer_id: Option<Uuid>,
    /// Organization ID.
    pub organization_id: Uuid,
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
    pub created_by: Option<Uuid>,
    /// User who last updated this order.
    pub updated_by: Option<Uuid>,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
    /// Record last-update timestamp.
    pub updated_at: DateTime<FixedOffset>,
    /// Order items (populated when fetching a single order).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderItemResponse>>,
}

/// Single order item response DTO.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderItemResponse {
    /// Order item ID.
    pub id: Uuid,
    /// Parent order ID.
    pub order_id: Uuid,
    /// Product ID.
    pub product_id: Uuid,
    /// Item name snapshot.
    pub item_name: String,
    /// Quantity ordered.
    pub quantity: i32,
    /// Unit price at time of order.
    pub unit_price: i64,
    /// Total price for this line.
    pub total_price: i64,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
}

/// POS menu item DTO (product + pricing for the order screen).
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MenuResponse {
    /// Product ID.
    pub id: Uuid,
    /// Product name.
    pub name: String,
    /// Product SKU.
    pub sku: String,
    /// Base selling price.
    pub base_price: i64,
    /// Image URL.
    pub image_url: Option<String>,
    /// Category ID.
    pub category_id: Option<Uuid>,
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
    pub product_id: Uuid,
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
    /// Convert a SeaORM order model into a response DTO (without items).
    pub fn from_model(model: orders::Model) -> Self {
        Self {
            id: model.id,
            order_number: model.order_number,
            customer_id: model.customer_id,
            organization_id: model.organization_id,
            status: model.status,
            order_type: model.r#type,
            subtotal: model.subtotal,
            tax_amount: model.tax_amount,
            discount_amount: model.discount_amount,
            total_amount: model.total_amount,
            payment_method: model.payment_method,
            payment_status: model.payment_status,
            paid_amount: model.paid_amount,
            notes: model.notes,
            table_number: model.table_number,
            customer_name: model.customer_name,
            created_by: model.created_by,
            updated_by: model.updated_by,
            created_at: model.created_at,
            updated_at: model.updated_at,
            items: None,
        }
    }

    /// Convert a SeaORM order model into a response DTO with items.
    pub fn from_model_with_items(model: orders::Model, items: Vec<order_items::Model>) -> Self {
        let item_responses = items
            .into_iter()
            .map(OrderItemResponse::from_model)
            .collect();
        let mut resp = Self::from_model(model);
        resp.items = Some(item_responses);
        resp
    }
}

impl OrderItemResponse {
    /// Convert a SeaORM order item model into a response DTO.
    pub fn from_model(model: order_items::Model) -> Self {
        Self {
            id: model.id,
            order_id: model.order_id,
            product_id: model.product_id,
            item_name: model.item_name,
            quantity: model.quantity,
            unit_price: model.unit_price,
            total_price: model.total_price,
            created_at: model.created_at,
        }
    }
}
