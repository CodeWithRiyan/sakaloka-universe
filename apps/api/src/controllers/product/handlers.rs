//! Handler functions for product endpoints.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::helpers::patch_builder::PatchBuilder;
use crate::views::{
    product::{
        BrandSummary, CategorySummary, CreateProductRequest, ProductResponse, UpdateProductRequest,
    },
    record_id_to_string, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

/// `GET /api/products` — list products with pagination, search, and sorting.
pub async fn list(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<ProductResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_desc = params.is_desc();

    // Parallelize count + list queries
    let search = params.search.as_deref();
    let (count_result, list_result) = tokio::join!(
        state.db.count_products(search),
        state
            .db
            .list_products(limit, start, search, sort_by, sort_desc),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count products"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to list products"))?;

    // Batch-fetch related categories and brands (avoids N+1 queries)
    let cat_ids: Vec<String> = items
        .iter()
        .filter_map(|p| p.category_id.as_ref().map(record_id_to_string))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    let brand_ids: Vec<String> = items
        .iter()
        .filter_map(|p| p.brand_id.as_ref().map(record_id_to_string))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    // Parallelize category + brand batch lookups
    let (cat_result, brand_result) = tokio::join!(
        state.db.find_categories_by_ids(&cat_ids),
        state.db.find_brands_by_ids(&brand_ids),
    );

    let cat_map: std::collections::HashMap<String, String> = cat_result
        .unwrap_or_default()
        .into_iter()
        .map(|c| (record_id_to_string(&c.id), c.name))
        .collect();

    let brand_map: std::collections::HashMap<String, String> = brand_result
        .unwrap_or_default()
        .into_iter()
        .map(|b| (record_id_to_string(&b.id), b.name))
        .collect();

    let responses: Vec<ProductResponse> = items
        .iter()
        .map(|item| {
            let cat = item.category_id.as_ref().and_then(|cid| {
                let key = record_id_to_string(cid);
                cat_map.get(&key).map(|name| CategorySummary {
                    id: key,
                    name: name.clone(),
                })
            });
            let brand = item.brand_id.as_ref().and_then(|bid| {
                let key = record_id_to_string(bid);
                brand_map.get(&key).map(|name| BrandSummary {
                    id: key,
                    name: name.clone(),
                })
            });
            ProductResponse::from_model(item, cat, brand)
        })
        .collect();

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
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    let product = state
        .db
        .find_product(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find product"))?;

    let product = match product {
        Some(p) => p,
        None => return Ok(Json(ApiResponse::not_found("Product"))),
    };

    let cat_key = product.category_id.as_ref().map(record_id_to_string);
    let brand_key = product.brand_id.as_ref().map(record_id_to_string);

    let (cat_result, brand_result) = tokio::join!(
        async {
            match cat_key.as_deref() {
                Some(k) => state.db.find_category(k).await,
                None => Ok(None),
            }
        },
        async {
            match brand_key.as_deref() {
                Some(k) => state.db.find_brand(k).await,
                None => Ok(None),
            }
        },
    );

    let cat = match cat_result {
        Ok(Some(c)) => Some(CategorySummary {
            id: record_id_to_string(&c.id),
            name: c.name,
        }),
        Ok(None) => None,
        Err(e) => {
            tracing::warn!(error = %e, "Failed to load category for product");
            None
        }
    };
    let brand = match brand_result {
        Ok(Some(b)) => Some(BrandSummary {
            id: record_id_to_string(&b.id),
            name: b.name,
        }),
        Ok(None) => None,
        Err(e) => {
            tracing::warn!(error = %e, "Failed to load brand for product");
            None
        }
    };

    let response = ProductResponse::from_model(&product, cat, brand);
    Ok(Json(ApiResponse::ok(response, "Product retrieved")))
}

/// `POST /api/products` — create a new product.
pub async fn create(
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

    let org_id = crate::helpers::org_resolver::resolve_caller_org(&state, &claims).await?;

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
            &org_id,
        )
        .await
        .map_err(|e| db_err(e, "Failed to create product"))?;

    let response = ProductResponse::from_model(&result, None, None);
    Ok(Json(ApiResponse::created(response, "Product created")))
}

/// `PATCH /api/products/:id` — update an existing product.
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    let existing = state
        .db
        .find_product(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find product"))?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Product")));
    }

    let updates = PatchBuilder::new()
        .set_string("name", payload.name)
        .set_string("sku", payload.sku)
        .set_i64("base_price", payload.base_price)
        .set_string("description", payload.description)
        .set_string("barcode", payload.barcode)
        .set_i64("cost_price", payload.cost_price)
        .set_string("category_id", payload.category_id)
        .set_string("brand_id", payload.brand_id)
        .set_string("image_url", payload.image_url)
        .set_f64("weight", payload.weight)
        .set_value("dimensions", payload.dimensions)
        .set_bool("track_inventory", payload.track_inventory)
        .set_i64("min_stock_level", payload.min_stock_level)
        .set_bool("is_featured", payload.is_featured)
        .set_value("tags", payload.tags.map(|t| serde_json::json!(t)))
        .build();

    let updated = state
        .db
        .update_product(&id, &updates)
        .await
        .map_err(|e| db_err(e, "Failed to update product"))?;

    let response = ProductResponse::from_model(&updated, None, None);
    Ok(Json(ApiResponse::ok(response, "Product updated")))
}

/// `DELETE /api/products/:id` — soft-delete a product.
pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state
        .db
        .find_product(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find product"))?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Product")));
    }

    state
        .db
        .delete_product(&id)
        .await
        .map_err(|e| db_err(e, "Failed to delete product"))?;

    Ok(Json(ApiResponse::ok((), "Product deleted")))
}
