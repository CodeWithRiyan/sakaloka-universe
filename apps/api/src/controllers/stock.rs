//! Stock / Inventory management controller.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::models::_entities::{inventory_items, stock_movements, users};
use crate::views::{
    stock::{AdjustStockRequest, StockHistoryResponse, StockResponse},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/api/inventory/pos-stock` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/inventory/pos-stock")
        .add("/", get(list))
        .add("/low-stock", get(low_stock))
        .add("/:id", get(show))
        .add("/:id/history", get(history))
        .add("/products/:product_id/adjust", post(adjust))
}

/// `GET /api/inventory/pos-stock` — list all stock items with pagination.
async fn list(
    State(ctx): State<AppContext>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page();
    let limit = params.limit();

    let condition = Condition::all();

    let total = inventory_items::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count inventory items");
            Error::InternalServerError
        })?;

    let mut query = inventory_items::Entity::find().filter(condition);
    query = match params.sort_by.as_deref() {
        Some("quantity_on_hand") if params.is_desc() => {
            query.order_by_desc(inventory_items::Column::QuantityOnHand)
        }
        Some("quantity_on_hand") => query.order_by_asc(inventory_items::Column::QuantityOnHand),
        Some("quantity_available") if params.is_desc() => {
            query.order_by_desc(inventory_items::Column::QuantityAvailable)
        }
        Some("quantity_available") => {
            query.order_by_asc(inventory_items::Column::QuantityAvailable)
        }
        _ => query.order_by_desc(inventory_items::Column::UpdatedAt),
    };

    let items = query
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list inventory items");
            Error::InternalServerError
        })?;

    let responses: Vec<StockResponse> = items.into_iter().map(StockResponse::from_model).collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Stock items retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/inventory/pos-stock/low-stock` — list items below their minimum stock level.
async fn low_stock(
    State(ctx): State<AppContext>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page();
    let limit = params.limit();

    // Items where quantity_available <= min_stock_level (and min_stock_level is set)
    let condition = Condition::all().add(inventory_items::Column::MinStockLevel.is_not_null());

    // We need to fetch all matching and then filter in-app because SeaORM
    // doesn't easily support column-to-column comparisons. We paginate after.
    let all_items = inventory_items::Entity::find()
        .filter(condition)
        .order_by_asc(inventory_items::Column::QuantityAvailable)
        .all(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to query low stock items");
            Error::InternalServerError
        })?;

    let low_items: Vec<_> = all_items
        .into_iter()
        .filter(|item| {
            item.min_stock_level
                .map(|min| item.quantity_available <= min)
                .unwrap_or(false)
        })
        .collect();

    let total = low_items.len() as u64;
    let start = ((page - 1) * limit) as usize;
    let page_items: Vec<_> = low_items
        .into_iter()
        .skip(start)
        .take(limit as usize)
        .collect();

    let responses: Vec<StockResponse> = page_items
        .into_iter()
        .map(StockResponse::from_model)
        .collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Low stock items retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/inventory/pos-stock/:id` — fetch a single stock item.
async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let item = inventory_items::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    match item {
        Some(i) => format::json(ApiResponse::ok(
            StockResponse::from_model(i),
            "Stock item retrieved",
        )),
        None => format::json(ApiResponse::<()>::not_found("Stock item")),
    }
}

/// `GET /api/inventory/pos-stock/:id/history` — fetch stock movement history.
async fn history(
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    // Verify the inventory item exists
    let item = inventory_items::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if item.is_none() {
        return format::json(ApiResponse::<()>::not_found("Stock item"));
    }

    let page = params.page();
    let limit = params.limit();

    let condition = Condition::all().add(stock_movements::Column::InventoryItemId.eq(id));

    let total = stock_movements::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let movements = stock_movements::Entity::find()
        .filter(condition)
        .order_by_desc(stock_movements::Column::CreatedAt)
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|_| Error::InternalServerError)?;

    let responses: Vec<StockHistoryResponse> = movements
        .into_iter()
        .map(StockHistoryResponse::from_model)
        .collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Stock history retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `POST /api/inventory/pos-stock/products/:product_id/adjust` — adjust stock levels.
async fn adjust(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(product_id): Path<Uuid>,
    Json(payload): Json<AdjustStockRequest>,
) -> Result<Response> {
    // Validate input
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
        return format::json(ApiResponse::<()>::validation(errors));
    }

    let user_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;

    let caller = users::Entity::find_by_id(user_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or(Error::InternalServerError)?;

    // Find or create inventory item for this product + organization
    let inventory_item = inventory_items::Entity::find()
        .filter(inventory_items::Column::ProductId.eq(product_id))
        .filter(inventory_items::Column::OrganizationId.eq(caller.organization_id))
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let now = chrono::Utc::now().fixed_offset();

    let inventory_item = match inventory_item {
        Some(item) => item,
        None => {
            // Create a new inventory record for this product
            let new_item = inventory_items::ActiveModel {
                id: Set(Uuid::new_v4()),
                product_id: Set(product_id),
                location_id: Set(None),
                quantity_on_hand: Set(0),
                quantity_reserved: Set(0),
                quantity_available: Set(0),
                min_stock_level: Set(None),
                max_stock_level: Set(None),
                reorder_point: Set(None),
                reorder_quantity: Set(None),
                average_cost: Set(None),
                last_cost: Set(None),
                organization_id: Set(caller.organization_id),
                last_movement_at: Set(None),
                created_at: Set(now),
                updated_at: Set(now),
            };
            new_item.insert(&ctx.db).await.map_err(|e| {
                tracing::error!(error = %e, "Failed to create inventory item");
                Error::InternalServerError
            })?
        }
    };

    let total_before = inventory_item.quantity_on_hand;
    let quantity_change = payload.quantity;
    let total_after = total_before + quantity_change;

    if total_after < 0 {
        return format::json(ApiResponse::<()>::error(
            "insufficient_stock",
            &format!(
                "Cannot reduce stock below zero. Current: {total_before}, requested change: {quantity_change}"
            ),
        ));
    }

    // Record the stock movement
    let movement = stock_movements::ActiveModel {
        id: Set(Uuid::new_v4()),
        inventory_item_id: Set(inventory_item.id),
        movement_type: Set(payload.movement_type),
        quantity: Set(quantity_change.abs()),
        reason: Set(payload.reason),
        reference: Set(payload.reference),
        notes: Set(payload.notes),
        total_before: Set(total_before),
        quantity_change: Set(quantity_change),
        total_after: Set(total_after),
        created_by: Set(Some(user_id)),
        created_at: Set(now),
    };

    movement.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to create stock movement");
        Error::InternalServerError
    })?;

    // Update the inventory item
    let mut active: inventory_items::ActiveModel = inventory_item.into();
    active.quantity_on_hand = Set(total_after);
    active.quantity_available = Set(total_after); // simplified: reserved not modified here
    active.last_movement_at = Set(Some(now));
    active.updated_at = Set(now);

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to update inventory item");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::ok(
        StockResponse::from_model(updated),
        "Stock adjusted",
    ))
}
