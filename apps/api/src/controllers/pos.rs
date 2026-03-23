//! POS (Point of Sale) controller — menu, orders, and order management.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::models::_entities::{categories, order_items, orders, products, users};
use crate::views::{
    order::{CreateOrderRequest, MenuResponse, OrderResponse, UpdateOrderRequest},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/api/pos` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/pos")
        .add("/menu", get(menu))
        .add("/orders", get(list_orders))
        .add("/orders", post(create_order))
        .add("/orders/active", get(active_orders))
        .add("/orders/history", get(order_history))
        .add("/orders/:id", get(show_order))
        .add("/orders/:id", patch(update_order))
}

/// `GET /api/pos/menu` — list products available for the POS order screen.
async fn menu(
    State(ctx): State<AppContext>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page();
    let limit = params.limit();

    // Only show non-deleted products
    let mut condition = Condition::all().add(products::Column::DeletedAt.is_null());

    if let Some(ref search) = params.search {
        if !search.is_empty() {
            condition = condition.add(
                Condition::any()
                    .add(products::Column::Name.contains(search))
                    .add(products::Column::Sku.contains(search)),
            );
        }
    }

    let total = products::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count menu items");
            Error::InternalServerError
        })?;

    let items = products::Entity::find()
        .filter(condition)
        .order_by_asc(products::Column::Name)
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list menu items");
            Error::InternalServerError
        })?;

    let mut responses = Vec::with_capacity(items.len());
    for item in items {
        let cat_name = if let Some(cat_id) = item.category_id {
            categories::Entity::find_by_id(cat_id)
                .one(&ctx.db)
                .await
                .ok()
                .flatten()
                .map(|c| c.name)
        } else {
            None
        };

        responses.push(MenuResponse {
            id: item.id,
            name: item.name,
            sku: item.sku,
            base_price: item.base_price,
            image_url: item.image_url,
            category_id: item.category_id,
            category_name: cat_name,
            is_featured: item.is_featured,
        });
    }

    format::json(PaginatedResponse {
        success: true,
        message: "Menu retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/pos/orders` — list all orders with pagination.
async fn list_orders(
    State(ctx): State<AppContext>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page();
    let limit = params.limit();

    let mut condition = Condition::all();
    if let Some(ref search) = params.search {
        if !search.is_empty() {
            condition = condition.add(
                Condition::any()
                    .add(orders::Column::OrderNumber.contains(search))
                    .add(orders::Column::CustomerName.contains(search)),
            );
        }
    }

    let total = orders::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count orders");
            Error::InternalServerError
        })?;

    let mut query = orders::Entity::find().filter(condition);
    query = match params.sort_by.as_deref() {
        Some("order_number") if params.is_desc() => {
            query.order_by_desc(orders::Column::OrderNumber)
        }
        Some("order_number") => query.order_by_asc(orders::Column::OrderNumber),
        Some("total_amount") if params.is_desc() => {
            query.order_by_desc(orders::Column::TotalAmount)
        }
        Some("total_amount") => query.order_by_asc(orders::Column::TotalAmount),
        _ => query.order_by_desc(orders::Column::CreatedAt),
    };

    let items = query
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list orders");
            Error::InternalServerError
        })?;

    let responses: Vec<OrderResponse> = items.into_iter().map(OrderResponse::from_model).collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Orders retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/pos/orders/active` — list active (non-completed, non-cancelled) orders.
async fn active_orders(
    State(ctx): State<AppContext>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page();
    let limit = params.limit();

    let condition = Condition::all()
        .add(orders::Column::Status.ne("completed"))
        .add(orders::Column::Status.ne("cancelled"));

    let total = orders::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let items = orders::Entity::find()
        .filter(condition)
        .order_by_desc(orders::Column::CreatedAt)
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|_| Error::InternalServerError)?;

    let responses: Vec<OrderResponse> = items.into_iter().map(OrderResponse::from_model).collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Active orders retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/pos/orders/history` — list completed / cancelled orders.
async fn order_history(
    State(ctx): State<AppContext>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page();
    let limit = params.limit();

    let condition = Condition::all().add(
        Condition::any()
            .add(orders::Column::Status.eq("completed"))
            .add(orders::Column::Status.eq("cancelled")),
    );

    let total = orders::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let items = orders::Entity::find()
        .filter(condition)
        .order_by_desc(orders::Column::CreatedAt)
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|_| Error::InternalServerError)?;

    let responses: Vec<OrderResponse> = items.into_iter().map(OrderResponse::from_model).collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Order history retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/pos/orders/:id` — fetch a single order with its items.
async fn show_order(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let order = orders::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let order = match order {
        Some(o) => o,
        None => return format::json(ApiResponse::<()>::not_found("Order")),
    };

    // Fetch order items
    let items = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(id))
        .order_by_asc(order_items::Column::CreatedAt)
        .all(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let response = OrderResponse::from_model_with_items(order, items);
    format::json(ApiResponse::ok(response, "Order retrieved"))
}

/// Generate a unique order number with the format `ORD-YYYYMMDD-XXXX`.
fn generate_order_number() -> String {
    let date = chrono::Utc::now().format("%Y%m%d");
    let random: u16 = rand_u16();
    format!("ORD-{date}-{random:04}")
}

/// Generate a pseudo-random u16 without pulling in the `rand` crate.
fn rand_u16() -> u16 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    (hasher.finish() % 10000) as u16
}

/// `POST /api/pos/orders` — create a new order.
async fn create_order(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Response> {
    if payload.items.is_empty() {
        return format::json(ApiResponse::<()>::validation(vec![
            "items cannot be empty".to_string()
        ]));
    }

    let user_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;

    let caller = users::Entity::find_by_id(user_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or(Error::InternalServerError)?;

    let now = chrono::Utc::now().fixed_offset();
    let order_id = Uuid::new_v4();
    let order_number = generate_order_number();

    // Resolve products and compute totals
    let mut subtotal: i64 = 0;
    let mut item_models = Vec::with_capacity(payload.items.len());

    for item_req in &payload.items {
        let product = products::Entity::find_by_id(item_req.product_id)
            .filter(products::Column::DeletedAt.is_null())
            .one(&ctx.db)
            .await
            .map_err(|_| Error::InternalServerError)?;

        let product = match product {
            Some(p) => p,
            None => {
                return format::json(ApiResponse::<()>::error(
                    "product_not_found",
                    &format!("Product {} not found", item_req.product_id),
                ))
            }
        };

        let unit_price = product.base_price;
        let total_price = unit_price * i64::from(item_req.quantity);
        subtotal += total_price;

        let item_name = item_req
            .item_name
            .clone()
            .unwrap_or_else(|| product.name.clone());

        item_models.push(order_items::ActiveModel {
            id: Set(Uuid::new_v4()),
            order_id: Set(order_id),
            product_id: Set(product.id),
            item_name: Set(item_name),
            quantity: Set(item_req.quantity),
            unit_price: Set(unit_price),
            total_price: Set(total_price),
            created_at: Set(now),
        });
    }

    // Compute tax (10% default)
    let tax_amount = subtotal / 10;
    let total_amount = subtotal + tax_amount;

    let order = orders::ActiveModel {
        id: Set(order_id),
        order_number: Set(order_number),
        customer_id: Set(None),
        organization_id: Set(caller.organization_id),
        status: Set("pending".to_string()),
        r#type: Set(payload.order_type),
        subtotal: Set(subtotal),
        tax_amount: Set(tax_amount),
        discount_amount: Set(0),
        total_amount: Set(total_amount),
        payment_method: Set(payload.payment_method),
        payment_status: Set("unpaid".to_string()),
        paid_amount: Set(0),
        notes: Set(payload.notes),
        table_number: Set(payload.table_number),
        customer_name: Set(payload.customer_name),
        created_by: Set(Some(user_id)),
        updated_by: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };

    // Insert the order
    let order_result = order.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to create order");
        Error::InternalServerError
    })?;

    // Insert all order items
    for item in item_models {
        item.insert(&ctx.db).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to create order item");
            Error::InternalServerError
        })?;
    }

    // Fetch the items back for the response
    let items = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order_id))
        .all(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let response = OrderResponse::from_model_with_items(order_result, items);
    format::json(ApiResponse::created(response, "Order created"))
}

/// `PATCH /api/pos/orders/:id` — update an existing order (status, payment, etc.).
async fn update_order(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOrderRequest>,
) -> Result<Response> {
    let existing = orders::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(o) => o,
        None => return format::json(ApiResponse::<()>::not_found("Order")),
    };

    // Prevent modification of completed/cancelled orders (except payment updates)
    if existing.status == "cancelled" {
        return format::json(ApiResponse::<()>::error(
            "order_cancelled",
            "Cannot modify a cancelled order",
        ));
    }

    let user_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;

    let mut active: orders::ActiveModel = existing.into();

    if let Some(status) = payload.status {
        active.status = Set(status);
    }
    if let Some(payment_method) = payload.payment_method {
        active.payment_method = Set(Some(payment_method));
    }
    if let Some(payment_status) = payload.payment_status {
        active.payment_status = Set(payment_status);
    }
    if let Some(paid_amount) = payload.paid_amount {
        active.paid_amount = Set(paid_amount);
    }
    if let Some(notes) = payload.notes {
        active.notes = Set(Some(notes));
    }
    if let Some(table_number) = payload.table_number {
        active.table_number = Set(Some(table_number));
    }
    if let Some(customer_name) = payload.customer_name {
        active.customer_name = Set(Some(customer_name));
    }

    active.updated_by = Set(Some(user_id));
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to update order");
        Error::InternalServerError
    })?;

    // Fetch items for complete response
    let items = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(id))
        .all(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let response = OrderResponse::from_model_with_items(updated, items);
    format::json(ApiResponse::ok(response, "Order updated"))
}
