//! POS (Point of Sale) controller — menu, orders, and order management.

use axum::{
    extract::{Extension, Path, Query, State},
    routing::get,
    Json, Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::{
    order::{CreateOrderRequest, MenuResponse, OrderResponse, UpdateOrderRequest},
    record_id_to_string, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

/// Registers all `/pos` routes (nested under `/api` by the top-level router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/pos/menu", get(menu))
        .route("/pos/orders", get(list_orders))
        .route("/pos/orders/active", get(active_orders))
        .route("/pos/orders/history", get(order_history))
        .route("/pos/orders/{id}", get(show_order));

    let write_routes = Router::new()
        .route("/pos/orders", axum::routing::post(create_order))
        .route("/pos/orders/{id}", axum::routing::patch(update_order))
        .route_layer(RequireScope::new(Scope::EntityWrite));

    Router::new().merge(read_routes).merge(write_routes)
}

/// `GET /api/pos/menu` — list products available for the POS order screen.
async fn menu(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<MenuResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let total = state
        .db
        .count_products(params.search.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count menu items");
            ApiError::Internal(anyhow::anyhow!("Failed to count menu items"))
        })?;

    let items = state
        .db
        .list_products(limit, start, params.search.as_deref(), "name", false)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list menu items");
            ApiError::Internal(anyhow::anyhow!("Failed to list menu items"))
        })?;

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
async fn list_orders(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<OrderResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let total = state
        .db
        .count_orders(params.search.as_deref(), None, None)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count orders");
            ApiError::Internal(anyhow::anyhow!("Failed to count orders"))
        })?;

    let items = state
        .db
        .list_orders(
            limit,
            start,
            params.search.as_deref(),
            params.sort_by.as_deref().unwrap_or("created_at"),
            params.is_desc(),
            None,
            None,
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list orders");
            ApiError::Internal(anyhow::anyhow!("Failed to list orders"))
        })?;

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
async fn active_orders(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<OrderResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    // Exclude completed and cancelled
    let exclude_statuses: &[&str] = &["completed", "cancelled"];

    let total = state
        .db
        .count_orders(None, None, Some(exclude_statuses))
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to count active orders")))?;

    let items = state
        .db
        .list_orders(
            limit,
            start,
            None,
            "created_at",
            true,
            None,
            Some(exclude_statuses),
        )
        .await
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
async fn order_history(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<OrderResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    // Only completed or cancelled
    let filter_statuses: &[&str] = &["completed", "cancelled"];

    let total = state
        .db
        .count_orders(None, Some(filter_statuses), None)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to count history orders")))?;

    let items = state
        .db
        .list_orders(
            limit,
            start,
            None,
            "created_at",
            true,
            Some(filter_statuses),
            None,
        )
        .await
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
async fn show_order(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<OrderResponse>>, ApiError> {
    let order = state.db.find_order(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find order");
        ApiError::Internal(anyhow::anyhow!("Failed to find order"))
    })?;

    let order = match order {
        Some(o) => o,
        None => return Ok(Json(ApiResponse::not_found("Order"))),
    };

    // Fetch order items
    let items = state.db.list_order_items(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to list order items");
        ApiError::Internal(anyhow::anyhow!("Failed to list order items"))
    })?;

    let response = OrderResponse::from_model_with_items(&order, &items);
    Ok(Json(ApiResponse::ok(response, "Order retrieved")))
}

/// Generate a unique order number with the format `ORD-YYYYMMDD-XXXXXXXX`.
fn generate_order_number() -> String {
    let date = chrono::Utc::now().format("%Y%m%d");
    let suffix = &uuid::Uuid::new_v4().to_string()[..8];
    format!("ORD-{date}-{suffix}")
}

/// `POST /api/pos/orders` — create a new order.
async fn create_order(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<ApiResponse<OrderResponse>>, ApiError> {
    if payload.items.is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "items cannot be empty".to_string()
        ])));
    }

    let caller = state
        .db
        .find_user_by_id(&claims.sub)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to find user")))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("User not found")))?;

    let org_id = caller
        .organization_id
        .as_ref()
        .map(record_id_to_string)
        .unwrap_or_default();

    let order_number = generate_order_number();

    // Resolve products and compute totals
    let mut subtotal: i64 = 0;
    let mut resolved_items: Vec<(String, String, i32, i64, i64)> = Vec::new();

    for item_req in &payload.items {
        let product = state
            .db
            .find_product(&item_req.product_id)
            .await
            .map_err(|_| ApiError::Internal(anyhow::anyhow!("DB error")))?;

        let product = match product {
            Some(p) => p,
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

        let product_id = record_id_to_string(&product.id);
        resolved_items.push((
            product_id,
            item_name,
            item_req.quantity,
            unit_price,
            total_price,
        ));
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
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create order");
            ApiError::Internal(anyhow::anyhow!("Failed to create order"))
        })?;

    let order_id = record_id_to_string(&order.id);

    // Insert all order items
    for (product_id, item_name, quantity, unit_price, total_price) in &resolved_items {
        state
            .db
            .create_order_item(
                &order_id,
                product_id,
                item_name,
                *quantity,
                *unit_price,
                *total_price,
            )
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to create order item");
                ApiError::Internal(anyhow::anyhow!("Failed to create order item"))
            })?;
    }

    // Fetch the items back for the response
    let items = state.db.list_order_items(&order_id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to list order items for response");
        ApiError::Internal(anyhow::anyhow!("Failed to list order items"))
    })?;

    let response = OrderResponse::from_model_with_items(&order, &items);
    Ok(Json(ApiResponse::created(response, "Order created")))
}

/// `PATCH /api/pos/orders/:id` — update an existing order (status, payment,
/// etc.).
async fn update_order(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateOrderRequest>,
) -> Result<Json<ApiResponse<OrderResponse>>, ApiError> {
    let existing = state.db.find_order(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find order for update");
        ApiError::Internal(anyhow::anyhow!("Failed to find order"))
    })?;

    let existing = match existing {
        Some(o) => o,
        None => return Ok(Json(ApiResponse::not_found("Order"))),
    };

    // Prevent modification of cancelled orders
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
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update order");
            ApiError::Internal(anyhow::anyhow!("Failed to update order"))
        })?;

    // Fetch items for complete response
    let items = state.db.list_order_items(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to list order items");
        ApiError::Internal(anyhow::anyhow!("Failed to list order items"))
    })?;

    let response = OrderResponse::from_model_with_items(&updated, &items);
    Ok(Json(ApiResponse::ok(response, "Order updated")))
}
