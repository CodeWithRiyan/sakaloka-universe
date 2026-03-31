//! Handler functions for brand endpoints.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::helpers::patch_builder::PatchBuilder;
use crate::views::{
    brand::{BrandResponse, CreateBrandRequest, UpdateBrandRequest},
    slugify, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

/// `GET /api/products/brands` — list brands with pagination and search.
pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<BrandResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let search = params.search.as_deref();
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_desc = params.is_desc();
    let org_id = claims.org_id.as_deref();
    let (count_result, list_result) = tokio::join!(
        state.db.count_brands(org_id, search),
        state
            .db
            .list_brands(org_id, limit, start, search, sort_by, sort_desc),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count brands"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to list brands"))?;

    let responses: Vec<BrandResponse> = items.iter().map(BrandResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Brands retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/products/brands/:id` — fetch a single brand.
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<BrandResponse>>, ApiError> {
    let brand = state
        .db
        .find_brand(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find brand"))?;

    match brand {
        Some(b) => Ok(Json(ApiResponse::ok(
            BrandResponse::from_model(&b),
            "Brand retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Brand"))),
    }
}

/// `POST /api/products/brands` — create a new brand.
pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateBrandRequest>,
) -> Result<Json<ApiResponse<BrandResponse>>, ApiError> {
    if payload.name.trim().is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "name is required".to_string()
        ])));
    }

    let slug = slugify(&payload.name);
    let org_id = crate::helpers::org_resolver::resolve_caller_org(&state, &claims).await?;

    let result = state
        .db
        .create_brand(
            &payload.name,
            &slug,
            &org_id,
            payload.description.as_deref(),
            payload.logo.as_deref(),
            payload.website.as_deref(),
            &claims.sub,
        )
        .await
        .map_err(|e| db_err(e, "Failed to create brand"))?;

    Ok(Json(ApiResponse::created(
        BrandResponse::from_model(&result),
        "Brand created",
    )))
}

/// `PATCH /api/products/brands/:id` — update an existing brand.
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBrandRequest>,
) -> Result<Json<ApiResponse<BrandResponse>>, ApiError> {
    let existing = state
        .db
        .find_brand(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find brand"))?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Brand")));
    }

    let slug = payload.name.as_ref().map(|n| slugify(n));

    let updates = PatchBuilder::new()
        .set_string("name", payload.name)
        .set_string("slug", slug)
        .set_string("description", payload.description)
        .set_string("logo", payload.logo)
        .set_string("website", payload.website)
        .build();

    let updated = state
        .db
        .update_brand(&id, &updates)
        .await
        .map_err(|e| db_err(e, "Failed to update brand"))?;

    Ok(Json(ApiResponse::ok(
        BrandResponse::from_model(&updated),
        "Brand updated",
    )))
}

/// `DELETE /api/products/brands/:id` — delete a brand.
pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state
        .db
        .find_brand(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find brand"))?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Brand")));
    }

    state
        .db
        .delete_brand(&id)
        .await
        .map_err(|e| db_err(e, "Failed to delete brand"))?;

    Ok(Json(ApiResponse::ok((), "Brand deleted")))
}
