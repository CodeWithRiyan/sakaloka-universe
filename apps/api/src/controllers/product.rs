//! Product CRUD controller.

use axum::{
    extract::{Extension, Path, Query, State},
    routing::get,
    Json, Router,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::{
    product::{
        BrandSummary, CategorySummary, CreateProductRequest, ProductResponse, UpdateProductRequest,
    },
    record_id_to_string, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

/// Registers all `/products` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/products", get(list).post(create))
        .route("/products/{id}", get(show).patch(update).delete(remove))
}

/// `GET /api/products` — list products with pagination, search, and sorting.
async fn list(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<ProductResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_desc = params.is_desc();

    let total = state
        .db
        .count_products(params.search.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count products");
            ApiError::Internal(anyhow::anyhow!("Failed to count products"))
        })?;

    let items = state
        .db
        .list_products(limit, start, params.search.as_deref(), sort_by, sort_desc)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list products");
            ApiError::Internal(anyhow::anyhow!("Failed to list products"))
        })?;

    // Resolve categories and brands
    let mut responses = Vec::with_capacity(items.len());
    for item in &items {
        let cat = if let Some(ref cat_id) = item.category_id {
            let cat_key = record_id_to_string(cat_id);
            state
                .db
                .find_category(&cat_key)
                .await
                .ok()
                .flatten()
                .map(|c| CategorySummary {
                    id: record_id_to_string(&c.id),
                    name: c.name,
                })
        } else {
            None
        };

        let brand = if let Some(ref brand_id) = item.brand_id {
            let brand_key = record_id_to_string(brand_id);
            state
                .db
                .find_brand(&brand_key)
                .await
                .ok()
                .flatten()
                .map(|b| BrandSummary {
                    id: record_id_to_string(&b.id),
                    name: b.name,
                })
        } else {
            None
        };

        responses.push(ProductResponse::from_model(item, cat, brand));
    }

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Products retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/products/:id` — fetch a single product.
async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    let product = state.db.find_product(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find product");
        ApiError::Internal(anyhow::anyhow!("Failed to find product"))
    })?;

    let product = match product {
        Some(p) => p,
        None => return Ok(Json(ApiResponse::not_found("Product"))),
    };

    let cat = if let Some(ref cat_id) = product.category_id {
        let cat_key = record_id_to_string(cat_id);
        state
            .db
            .find_category(&cat_key)
            .await
            .ok()
            .flatten()
            .map(|c| CategorySummary {
                id: record_id_to_string(&c.id),
                name: c.name,
            })
    } else {
        None
    };

    let brand = if let Some(ref brand_id) = product.brand_id {
        let brand_key = record_id_to_string(brand_id);
        state
            .db
            .find_brand(&brand_key)
            .await
            .ok()
            .flatten()
            .map(|b| BrandSummary {
                id: record_id_to_string(&b.id),
                name: b.name,
            })
    } else {
        None
    };

    let response = ProductResponse::from_model(&product, cat, brand);
    Ok(Json(ApiResponse::ok(response, "Product retrieved")))
}

/// `POST /api/products` — create a new product.
async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    // Validate required fields
    if payload.name.trim().is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "name is required".to_string()
        ])));
    }
    if payload.sku.trim().is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "sku is required".to_string()
        ])));
    }

    let result = state
        .db
        .create_product(
            &payload.name,
            &payload.sku,
            payload.base_price,
            payload.description.as_deref(),
            payload.barcode.as_deref(),
            payload.cost_price,
            payload.category_id.as_deref(),
            payload.brand_id.as_deref(),
            payload.image_url.as_deref(),
            payload.weight,
            payload.dimensions.as_ref(),
            payload.track_inventory.unwrap_or(false),
            payload.min_stock_level,
            payload.is_featured.unwrap_or(false),
            payload.tags.as_deref(),
            &claims.sub,
            &claims.sub, // org_id fallback — caller's ID until org context is wired
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create product");
            ApiError::Internal(anyhow::anyhow!("Failed to create product"))
        })?;

    let response = ProductResponse::from_model(&result, None, None);
    Ok(Json(ApiResponse::created(response, "Product created")))
}

/// `PATCH /api/products/:id` — update an existing product.
async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    // Verify the product exists
    let existing = state.db.find_product(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find product for update");
        ApiError::Internal(anyhow::anyhow!("Failed to find product"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Product")));
    }

    // Build a JSON value with only the provided fields
    let mut updates = serde_json::Map::new();
    if let Some(name) = payload.name {
        updates.insert("name".to_string(), serde_json::Value::String(name));
    }
    if let Some(sku) = payload.sku {
        updates.insert("sku".to_string(), serde_json::Value::String(sku));
    }
    if let Some(base_price) = payload.base_price {
        updates.insert(
            "base_price".to_string(),
            serde_json::Value::Number(base_price.into()),
        );
    }
    if let Some(description) = payload.description {
        updates.insert(
            "description".to_string(),
            serde_json::Value::String(description),
        );
    }
    if let Some(barcode) = payload.barcode {
        updates.insert("barcode".to_string(), serde_json::Value::String(barcode));
    }
    if let Some(cost_price) = payload.cost_price {
        updates.insert(
            "cost_price".to_string(),
            serde_json::Value::Number(cost_price.into()),
        );
    }
    if let Some(category_id) = payload.category_id {
        updates.insert(
            "category_id".to_string(),
            serde_json::Value::String(category_id),
        );
    }
    if let Some(brand_id) = payload.brand_id {
        updates.insert("brand_id".to_string(), serde_json::Value::String(brand_id));
    }
    if let Some(image_url) = payload.image_url {
        updates.insert(
            "image_url".to_string(),
            serde_json::Value::String(image_url),
        );
    }
    if let Some(weight) = payload.weight {
        updates.insert("weight".to_string(), serde_json::json!(weight));
    }
    if let Some(dimensions) = payload.dimensions {
        updates.insert("dimensions".to_string(), dimensions);
    }
    if let Some(track_inventory) = payload.track_inventory {
        updates.insert(
            "track_inventory".to_string(),
            serde_json::Value::Bool(track_inventory),
        );
    }
    if let Some(min_stock_level) = payload.min_stock_level {
        updates.insert(
            "min_stock_level".to_string(),
            serde_json::Value::Number(min_stock_level.into()),
        );
    }
    if let Some(is_featured) = payload.is_featured {
        updates.insert(
            "is_featured".to_string(),
            serde_json::Value::Bool(is_featured),
        );
    }
    if let Some(tags) = payload.tags {
        updates.insert("tags".to_string(), serde_json::json!(tags));
    }

    let updated = state
        .db
        .update_product(&id, &serde_json::Value::Object(updates))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update product");
            ApiError::Internal(anyhow::anyhow!("Failed to update product"))
        })?;

    let response = ProductResponse::from_model(&updated, None, None);
    Ok(Json(ApiResponse::ok(response, "Product updated")))
}

/// `DELETE /api/products/:id` — soft-delete a product.
async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state.db.find_product(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find product for delete");
        ApiError::Internal(anyhow::anyhow!("Failed to find product"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Product")));
    }

    state.db.delete_product(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to soft-delete product");
        ApiError::Internal(anyhow::anyhow!("Failed to delete product"))
    })?;

    Ok(Json(ApiResponse::ok((), "Product deleted")))
}
