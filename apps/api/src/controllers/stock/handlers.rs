//! Handler functions for stock/inventory endpoints.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::views::{
    stock::{AdjustStockRequest, StockHistoryResponse, StockResponse},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

#[utoipa::path(
    get,
    path = "/api/inventory/pos-stock",
    tag = "Stock",
    params(PaginationParams),
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "Stock items retrieved", body = PaginatedResponse<StockResponse>)
    )
)]
/// `GET /api/inventory/pos-stock` — list all stock items with pagination.
pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<StockResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;
    let org_id = claims.org_id.as_deref();

    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_desc = params.is_desc();
    let (count_result, list_result) = tokio::join!(
        state.db.count_inventory(org_id),
        state
            .db
            .list_inventory(org_id, limit, start, sort_by, sort_desc),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count inventory items"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to list inventory items"))?;

    let responses: Vec<StockResponse> = items.iter().map(StockResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Stock items retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

#[utoipa::path(
    get,
    path = "/api/inventory/pos-stock/low-stock",
    tag = "Stock",
    params(PaginationParams),
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "Low stock items retrieved", body = PaginatedResponse<StockResponse>)
    )
)]
/// `GET /api/inventory/pos-stock/low-stock` — list items below their minimum
/// stock level.
pub async fn low_stock(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<StockResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;
    let org_id = claims.org_id.as_deref();

    let (count_result, list_result) = tokio::join!(
        state.db.count_low_stock(org_id),
        state.db.list_low_stock(org_id, limit, start),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count low stock items"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to query low stock items"))?;

    let responses: Vec<StockResponse> = items.iter().map(StockResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Low stock items retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

#[utoipa::path(
    get,
    path = "/api/inventory/pos-stock/{id}",
    tag = "Stock",
    params(("id" = String, Path, description = "Stock ID")),
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "Stock item retrieved", body = ApiResponse<StockResponse>),
        (status = 404, description = "Not found")
    )
)]
/// `GET /api/inventory/pos-stock/:id` — fetch a single stock item.
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<StockResponse>>, ApiError> {
    let item = state
        .db
        .find_inventory_item(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find inventory item"))?;

    match item {
        Some(i) => Ok(Json(ApiResponse::ok(
            StockResponse::from_model(&i),
            "Stock item retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Stock item"))),
    }
}

#[utoipa::path(
    get,
    path = "/api/inventory/pos-stock/{id}/history",
    tag = "Stock",
    params(("id" = String, Path, description = "Stock ID"), PaginationParams),
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "Stock history retrieved", body = PaginatedResponse<StockHistoryResponse>)
    )
)]
/// `GET /api/inventory/pos-stock/:id/history` — fetch stock movement history.
pub async fn history(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<StockHistoryResponse>>, ApiError> {
    let item = state
        .db
        .find_inventory_item(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find inventory item"))?;

    if item.is_none() {
        return Ok(Json(PaginatedResponse {
            success: false,
            message: "Stock item not found".to_string(),
            data: PaginatedData {
                data: vec![],
                pagination: PageMeta::new(1, 10, 0),
                filters: ListFilters::from_params(&params),
            },
        }));
    }

    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let (count_result, list_result) = tokio::join!(
        state.db.count_stock_movements(&id),
        state.db.list_stock_movements(&id, limit, start),
    );
    let total = count_result
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to count stock movements")))?;
    let movements = list_result
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to list stock movements")))?;

    let responses: Vec<StockHistoryResponse> = movements
        .iter()
        .map(StockHistoryResponse::from_model)
        .collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Stock history retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

#[utoipa::path(
    post,
    path = "/api/inventory/pos-stock/products/{product_id}/adjust",
    tag = "Stock",
    params(("product_id" = String, Path, description = "Product ID")),
    request_body = AdjustStockRequest,
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "Stock adjusted", body = ApiResponse<StockResponse>),
        (status = 400, description = "Validation error")
    )
)]
/// `POST /api/inventory/pos-stock/products/:product_id/adjust` — adjust stock
/// levels.
pub async fn adjust(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(product_id): Path<String>,
    Json(payload): Json<AdjustStockRequest>,
) -> Result<Json<ApiResponse<StockResponse>>, ApiError> {
    let mut errors = Vec::new();
    if payload.quantity == 0 {
        errors.push("quantity must be non-zero".to_string());
    }
    if payload.reason.trim().is_empty() {
        errors.push("reason is required".to_string());
    }
    let valid_types = ["purchase", "sale", "adjustment", "damage", "return"];
    if !valid_types.contains(&payload.movement_type.as_str()) {
        errors.push(format!(
            "movement_type must be one of: {}",
            valid_types.join(", ")
        ));
    }
    if !errors.is_empty() {
        return Ok(Json(ApiResponse::validation(errors)));
    }

    let org_id = crate::helpers::org_resolver::resolve_caller_org(&state, &claims).await?;

    let inventory_item = state
        .db
        .find_inventory_by_product_org(&product_id, &org_id)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("DB error")))?;

    let inventory_item = match inventory_item {
        Some(item) => item,
        None => state
            .db
            .create_inventory_item(&product_id, &org_id)
            .await
            .map_err(|e| db_err(e, "Failed to create inventory item"))?,
    };

    let total_before = inventory_item.quantity_on_hand;
    let quantity_change = payload.quantity;
    let total_after = total_before + quantity_change;

    if total_after < 0 {
        return Ok(Json(ApiResponse::error(
            "insufficient_stock",
            &format!(
                "Cannot reduce stock below zero. Current: {total_before}, requested change: {quantity_change}"
            ),
        )));
    }

    let inv_id = inventory_item.id.clone();

    state
        .db
        .create_stock_movement(
            &inv_id,
            &payload.movement_type,
            quantity_change,
            Some(&payload.movement_type),
            payload.reference.as_deref(),
            payload.notes.as_deref().or(Some(&payload.reason)),
            Some(claims.sub.as_str()),
        )
        .await
        .map_err(|e| db_err(e, "Failed to create stock movement"))?;

    let updated = state
        .db
        .update_inventory_stock(&inv_id, total_after, total_after)
        .await
        .map_err(|e| db_err(e, "Failed to update inventory item"))?;

    Ok(Json(ApiResponse::ok(
        StockResponse::from_model(&updated),
        "Stock adjusted",
    )))
}
