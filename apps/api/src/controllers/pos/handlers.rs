//! Handler functions for POS endpoints.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::views::{
    order::{CreateOrderRequest, MenuResponse, OrderResponse, UpdateOrderRequest},
    record_id_to_string, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

use super::helpers::{generate_order_number, ResolvedOrderItem};

/// `GET /api/pos/menu` — list products available for the POS order screen.
pub async fn menu(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<MenuResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let search = params.search.as_deref();
    let (count_result, list_result) = tokio::join!(
        state.db.count_products(search),
        state.db.list_products(limit, start, search, "name", false),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count menu items"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to list menu items"))?;

    // Batch-fetch categories (avoids N+1 queries)
    let cat_ids: Vec<String> = items
        .iter()
        .filter_map(|p| p.category_id.as_ref().map(record_id_to_string))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let cat_map: std::collections::HashMap<String, String> = state
        .db
        .find_categories_by_ids(&cat_ids)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|c| (record_id_to_string(&c.id), c.name))
        .collect();

    let responses: Vec<MenuResponse> = items
        .iter()
        .map(|item| {
            let cat_key = item.category_id.as_ref().map(record_id_to_string);
            let cat_name = cat_key.as_ref().and_then(|k| cat_map.get(k)).cloned();
            MenuResponse {
                id: record_id_to_string(&item.id),
                name: item.name.clone(),
                sku: item.sku.clone(),
                base_price: item.base_price,
                image_url: item.image_url.clone(),
                category_id: cat_key,
                category_name: cat_name,
                is_featured: item.is_featured,
            }
        })
        .collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Menu retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/pos/orders` — list all orders with pagination.
pub async fn list_orders(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<OrderResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let search = params.search.as_deref();
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_desc = params.is_desc();
    let (count_result, list_result) = tokio::join!(
        state.db.count_orders(search, None, None),
        state
            .db
            .list_orders(limit, start, search, sort_by, sort_desc, None, None),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count orders"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to list orders"))?;

    let responses: Vec<OrderResponse> = items.iter().map(OrderResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Orders retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/pos/orders/active` — list active (non-completed, non-cancelled)
/// orders.
pub async fn active_orders(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<OrderResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let exclude_statuses: &[&str] = &["completed", "cancelled"];

    let (count_result, list_result) = tokio::join!(
        state.db.count_orders(None, None, Some(exclude_statuses)),
        state.db.list_orders(
            limit,
            start,
            None,
            "created_at",
            true,
            None,
            Some(exclude_statuses),
        ),
    );
    let total = count_result
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to count active orders")))?;
    let items = list_result
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to list active orders")))?;

    let responses: Vec<OrderResponse> = items.iter().map(OrderResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Active orders retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/pos/orders/history` — list completed / cancelled orders.
pub async fn order_history(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<OrderResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let filter_statuses: &[&str] = &["completed", "cancelled"];

    let (count_result, list_result) = tokio::join!(
        state.db.count_orders(None, Some(filter_statuses), None),
        state.db.list_orders(
            limit,
            start,
            None,
            "created_at",
            true,
            Some(filter_statuses),
            None,
        ),
    );
    let total = count_result
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to count history orders")))?;
    let items = list_result
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to list history orders")))?;

    let responses: Vec<OrderResponse> = items.iter().map(OrderResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Order history retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/pos/orders/:id` — fetch a single order with its items.
pub async fn show_order(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<OrderResponse>>, ApiError> {
    let order = state
        .db
        .find_order(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find order"))?;

    let order = match order {
        Some(o) => o,
        None => return Ok(Json(ApiResponse::not_found("Order"))),
    };

    let items = state
        .db
        .list_order_items(&id)
        .await
        .map_err(|e| db_err(e, "Failed to list order items"))?;

    let response = OrderResponse::from_model_with_items(&order, &items);
    Ok(Json(ApiResponse::ok(response, "Order retrieved")))
}

/// `POST /api/pos/orders` — create a new order.
pub async fn create_order(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<ApiResponse<OrderResponse>>, ApiError> {
    if payload.items.is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "items cannot be empty".to_string()
        ])));
    }

    let org_id = crate::helpers::org_resolver::resolve_caller_org(&state, &claims).await?;
    let order_number = generate_order_number();

    // Batch-fetch all products in one query (avoids N+1)
    let product_ids: Vec<String> = payload.items.iter().map(|i| i.product_id.clone()).collect();

    let products = state
        .db
        .find_products_by_ids(&product_ids)
        .await
        .map_err(|e| db_err(e, "Failed to batch-fetch products for order"))?;

    let product_map: std::collections::HashMap<String, &sakaloka_core::models::product::Product> =
        products
            .iter()
            .map(|p| (record_id_to_string(&p.id), p))
            .collect();

    let mut subtotal: i64 = 0;
    let mut resolved_items: Vec<ResolvedOrderItem> = Vec::with_capacity(payload.items.len());

    for item_req in &payload.items {
        let product = match product_map.get(&item_req.product_id) {
            Some(p) => *p,
            None => {
                return Ok(Json(ApiResponse::error(
                    "product_not_found",
                    &format!("Product {} not found", item_req.product_id),
                )))
            }
        };

        let unit_price = product.base_price;
        let total_price = unit_price * i64::from(item_req.quantity);
        subtotal += total_price;

        let item_name = item_req
            .item_name
            .clone()
            .unwrap_or_else(|| product.name.clone());

        resolved_items.push(ResolvedOrderItem {
            product_id: record_id_to_string(&product.id),
            item_name,
            quantity: item_req.quantity,
            unit_price,
            total_price,
        });
    }

    // Compute tax (10% default)
    let tax_amount = subtotal / 10;
    let total_amount = subtotal + tax_amount;

    let order = state
        .db
        .create_order(
            &order_number,
            &org_id,
            &payload.order_type,
            subtotal,
            tax_amount,
            total_amount,
            payload.payment_method.as_deref(),
            payload.notes.as_deref(),
            payload.table_number.as_deref(),
            payload.customer_name.as_deref(),
            &claims.sub,
        )
        .await
        .map_err(|e| db_err(e, "Failed to create order"))?;

    let order_id = record_id_to_string(&order.id);

    // Insert all order items
    for item in &resolved_items {
        state
            .db
            .create_order_item(
                &order_id,
                &item.product_id,
                &item.item_name,
                item.quantity,
                item.unit_price,
                item.total_price,
            )
            .await
            .map_err(|e| db_err(e, "Failed to create order item"))?;
    }

    // Fetch the items back for the response
    let items = state
        .db
        .list_order_items(&order_id)
        .await
        .map_err(|e| db_err(e, "Failed to list order items"))?;

    let response = OrderResponse::from_model_with_items(&order, &items);
    Ok(Json(ApiResponse::created(response, "Order created")))
}

/// `PATCH /api/pos/orders/:id` — update an existing order (status, payment,
/// etc.).
pub async fn update_order(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateOrderRequest>,
) -> Result<Json<ApiResponse<OrderResponse>>, ApiError> {
    let existing = state
        .db
        .find_order(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find order"))?;

    let existing = match existing {
        Some(o) => o,
        None => return Ok(Json(ApiResponse::not_found("Order"))),
    };

    if existing.status == "cancelled" {
        return Ok(Json(ApiResponse::error(
            "order_cancelled",
            "Cannot modify a cancelled order",
        )));
    }

    let updated = state
        .db
        .update_order(
            &id,
            payload.status.as_deref(),
            payload.payment_method.as_deref(),
            payload.payment_status.as_deref(),
            payload.paid_amount,
            payload.notes.as_deref(),
            payload.table_number.as_deref(),
            payload.customer_name.as_deref(),
            &claims.sub,
        )
        .await
        .map_err(|e| db_err(e, "Failed to update order"))?;

    let items = state
        .db
        .list_order_items(&id)
        .await
        .map_err(|e| db_err(e, "Failed to list order items"))?;

    let response = OrderResponse::from_model_with_items(&updated, &items);
    Ok(Json(ApiResponse::ok(response, "Order updated")))
}
