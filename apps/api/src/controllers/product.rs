//! Product CRUD controller.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::models::_entities::{brands, categories, products};
use crate::views::{
    product::{
        BrandSummary, CategorySummary, CreateProductRequest, ProductResponse, UpdateProductRequest,
    },
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/api/products` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products")
        .add("/", get(list))
        .add("/", post(create))
        .add("/:id", get(show))
        .add("/:id", patch(update))
        .add("/:id", delete(remove))
}

/// `GET /api/products` — list products with pagination, search, and sorting.
async fn list(
    State(ctx): State<AppContext>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page();
    let limit = params.limit();

    // Parse organization_id from claims or allow admin to see all
    let mut condition = Condition::all().add(products::Column::DeletedAt.is_null());

    // Apply search filter
    if let Some(ref search) = params.search {
        if !search.is_empty() {
            condition = condition.add(
                Condition::any()
                    .add(products::Column::Name.contains(search))
                    .add(products::Column::Sku.contains(search))
                    .add(products::Column::Barcode.contains(search)),
            );
        }
    }

    // Count total
    let total = products::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count products");
            Error::InternalServerError
        })?;

    // Apply sorting
    let mut query = products::Entity::find().filter(condition);
    match params.sort_by.as_deref() {
        Some("name") => {
            query = if params.is_desc() {
                query.order_by_desc(products::Column::Name)
            } else {
                query.order_by_asc(products::Column::Name)
            };
        }
        Some("base_price") => {
            query = if params.is_desc() {
                query.order_by_desc(products::Column::BasePrice)
            } else {
                query.order_by_asc(products::Column::BasePrice)
            };
        }
        Some("sku") => {
            query = if params.is_desc() {
                query.order_by_desc(products::Column::Sku)
            } else {
                query.order_by_asc(products::Column::Sku)
            };
        }
        Some("created_at") | None => {
            query = if params.is_desc() {
                query.order_by_desc(products::Column::CreatedAt)
            } else {
                query.order_by_asc(products::Column::CreatedAt)
            };
        }
        _ => {
            query = query.order_by_desc(products::Column::CreatedAt);
        }
    }

    // Fetch page
    let items = query
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list products");
            Error::InternalServerError
        })?;

    // Resolve categories and brands
    let mut responses = Vec::with_capacity(items.len());
    for item in items {
        let cat = if let Some(cat_id) = item.category_id {
            categories::Entity::find_by_id(cat_id)
                .one(&ctx.db)
                .await
                .ok()
                .flatten()
                .map(|c| CategorySummary {
                    id: c.id,
                    name: c.name,
                })
        } else {
            None
        };

        let brand = if let Some(brand_id) = item.brand_id {
            brands::Entity::find_by_id(brand_id)
                .one(&ctx.db)
                .await
                .ok()
                .flatten()
                .map(|b| BrandSummary {
                    id: b.id,
                    name: b.name,
                })
        } else {
            None
        };

        responses.push(ProductResponse::from_model(item, cat, brand));
    }

    format::json(PaginatedResponse {
        success: true,
        message: "Products retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/products/:id` — fetch a single product.
async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let product = products::Entity::find_by_id(id)
        .filter(products::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let product = match product {
        Some(p) => p,
        None => return format::json(ApiResponse::<()>::not_found("Product")),
    };

    let cat = if let Some(cat_id) = product.category_id {
        categories::Entity::find_by_id(cat_id)
            .one(&ctx.db)
            .await
            .ok()
            .flatten()
            .map(|c| CategorySummary {
                id: c.id,
                name: c.name,
            })
    } else {
        None
    };

    let brand = if let Some(brand_id) = product.brand_id {
        brands::Entity::find_by_id(brand_id)
            .one(&ctx.db)
            .await
            .ok()
            .flatten()
            .map(|b| BrandSummary {
                id: b.id,
                name: b.name,
            })
    } else {
        None
    };

    let response = ProductResponse::from_model(product, cat, brand);
    format::json(ApiResponse::ok(response, "Product retrieved"))
}

/// `POST /api/products` — create a new product.
async fn create(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Response> {
    // Validate required fields
    if payload.name.trim().is_empty() {
        return format::json(ApiResponse::<()>::validation(vec![
            "name is required".to_string()
        ]));
    }
    if payload.sku.trim().is_empty() {
        return format::json(ApiResponse::<()>::validation(vec![
            "sku is required".to_string()
        ]));
    }

    let user_id: Option<Uuid> = claims.sub.parse().ok();

    // Check for duplicate SKU
    let existing = products::Entity::find()
        .filter(products::Column::Sku.eq(&payload.sku))
        .filter(products::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if existing.is_some() {
        return format::json(ApiResponse::<()>::error(
            "duplicate_sku",
            "A product with this SKU already exists",
        ));
    }

    let now = chrono::Utc::now().fixed_offset();
    let id = Uuid::new_v4();

    // For the organization_id, we would normally extract from claims or
    // a session context.  Using a placeholder UUID until org context is wired.
    let org_id = user_id.unwrap_or(Uuid::new_v4());

    let model = products::ActiveModel {
        id: Set(id),
        name: Set(payload.name),
        description: Set(payload.description),
        sku: Set(payload.sku),
        barcode: Set(payload.barcode),
        base_price: Set(payload.base_price),
        cost_price: Set(payload.cost_price),
        category_id: Set(payload.category_id),
        brand_id: Set(payload.brand_id),
        image_url: Set(payload.image_url),
        weight: Set(payload.weight),
        dimensions: Set(payload.dimensions),
        track_inventory: Set(payload.track_inventory.unwrap_or(false)),
        min_stock_level: Set(payload.min_stock_level),
        is_featured: Set(payload.is_featured.unwrap_or(false)),
        tags: Set(payload.tags),
        organization_id: Set(org_id),
        created_by: Set(user_id),
        deleted_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = model.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to create product");
        Error::InternalServerError
    })?;

    let response = ProductResponse::from_model(result, None, None);
    format::json(ApiResponse::created(response, "Product created"))
}

/// `PATCH /api/products/:id` — update an existing product.
async fn update(
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Response> {
    let existing = products::Entity::find_by_id(id)
        .filter(products::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(p) => p,
        None => return format::json(ApiResponse::<()>::not_found("Product")),
    };

    let mut active: products::ActiveModel = existing.into();

    if let Some(name) = payload.name {
        active.name = Set(name);
    }
    if let Some(sku) = payload.sku {
        active.sku = Set(sku);
    }
    if let Some(base_price) = payload.base_price {
        active.base_price = Set(base_price);
    }
    if let Some(description) = payload.description {
        active.description = Set(Some(description));
    }
    if let Some(barcode) = payload.barcode {
        active.barcode = Set(Some(barcode));
    }
    if let Some(cost_price) = payload.cost_price {
        active.cost_price = Set(Some(cost_price));
    }
    if let Some(category_id) = payload.category_id {
        active.category_id = Set(Some(category_id));
    }
    if let Some(brand_id) = payload.brand_id {
        active.brand_id = Set(Some(brand_id));
    }
    if let Some(image_url) = payload.image_url {
        active.image_url = Set(Some(image_url));
    }
    if let Some(weight) = payload.weight {
        active.weight = Set(Some(weight));
    }
    if let Some(dimensions) = payload.dimensions {
        active.dimensions = Set(Some(dimensions));
    }
    if let Some(track_inventory) = payload.track_inventory {
        active.track_inventory = Set(track_inventory);
    }
    if let Some(min_stock_level) = payload.min_stock_level {
        active.min_stock_level = Set(Some(min_stock_level));
    }
    if let Some(is_featured) = payload.is_featured {
        active.is_featured = Set(is_featured);
    }
    if let Some(tags) = payload.tags {
        active.tags = Set(Some(tags));
    }

    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to update product");
        Error::InternalServerError
    })?;

    let response = ProductResponse::from_model(updated, None, None);
    format::json(ApiResponse::ok(response, "Product updated"))
}

/// `DELETE /api/products/:id` — soft-delete a product.
async fn remove(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let existing = products::Entity::find_by_id(id)
        .filter(products::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(p) => p,
        None => return format::json(ApiResponse::<()>::not_found("Product")),
    };

    let mut active: products::ActiveModel = existing.into();
    active.deleted_at = Set(Some(chrono::Utc::now().fixed_offset()));

    active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to soft-delete product");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::<()>::ok((), "Product deleted"))
}
