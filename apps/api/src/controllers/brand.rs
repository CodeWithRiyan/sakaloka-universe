//! Brand CRUD controller.

use axum::{
    extract::{Extension, Path, Query, State},
    routing::get,
    Json, Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::{
    brand::{BrandResponse, CreateBrandRequest, UpdateBrandRequest},
    slugify, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

/// Registers all `/products/brands` routes (nested under `/api` by the
/// top-level router).
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/products/brands", get(list))
        .route("/products/brands/{id}", get(show))
        .route(
            "/products/brands",
            axum::routing::post(create).layer(RequireScope::new(Scope::EntityWrite)),
        )
        .route(
            "/products/brands/{id}",
            axum::routing::patch(update).layer(RequireScope::new(Scope::EntityWrite)),
        )
        .route(
            "/products/brands/{id}",
            axum::routing::delete(remove).layer(RequireScope::new(Scope::EntityDelete)),
        )
}

/// `GET /api/products/brands` — list brands with pagination and search.
async fn list(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<BrandResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let total = state
        .db
        .count_brands(params.search.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count brands");
            ApiError::Internal(anyhow::anyhow!("Failed to count brands"))
        })?;

    let items = state
        .db
        .list_brands(
            limit,
            start,
            params.search.as_deref(),
            params.sort_by.as_deref().unwrap_or("created_at"),
            params.is_desc(),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list brands");
            ApiError::Internal(anyhow::anyhow!("Failed to list brands"))
        })?;

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
async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<BrandResponse>>, ApiError> {
    let brand = state.db.find_brand(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find brand");
        ApiError::Internal(anyhow::anyhow!("Failed to find brand"))
    })?;

    match brand {
        Some(b) => Ok(Json(ApiResponse::ok(
            BrandResponse::from_model(&b),
            "Brand retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Brand"))),
    }
}

/// `POST /api/products/brands` — create a new brand.
async fn create(
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

    // Resolve organization from the caller's user record
    let caller = state
        .db
        .find_user_by_id(&claims.sub)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to find caller")))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Caller not found")))?;
    let org_id = caller
        .organization_id
        .as_ref()
        .map(crate::views::record_id_to_string)
        .ok_or_else(|| ApiError::BadRequest("User has no organization assigned".to_string()))?;

    let result = state
        .db
        .create_brand(
            &payload.name,
            &slug,
            &org_id,
            payload.description.as_deref(),
            payload.logo.as_deref(),
            payload.website.as_deref(),
            Some(claims.sub.as_str()),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create brand");
            ApiError::Internal(anyhow::anyhow!("Failed to create brand"))
        })?;

    Ok(Json(ApiResponse::created(
        BrandResponse::from_model(&result),
        "Brand created",
    )))
}

/// `PATCH /api/products/brands/:id` — update an existing brand.
async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBrandRequest>,
) -> Result<Json<ApiResponse<BrandResponse>>, ApiError> {
    let existing = state.db.find_brand(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find brand for update");
        ApiError::Internal(anyhow::anyhow!("Failed to find brand"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Brand")));
    }

    let slug = payload.name.as_ref().map(|n| slugify(n));

    let updated = state
        .db
        .update_brand(
            &id,
            payload.name.as_deref(),
            slug.as_deref(),
            payload.description.as_deref(),
            payload.logo.as_deref(),
            payload.website.as_deref(),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update brand");
            ApiError::Internal(anyhow::anyhow!("Failed to update brand"))
        })?;

    Ok(Json(ApiResponse::ok(
        BrandResponse::from_model(&updated),
        "Brand updated",
    )))
}

/// `DELETE /api/products/brands/:id` — delete a brand.
async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state.db.find_brand(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find brand for delete");
        ApiError::Internal(anyhow::anyhow!("Failed to find brand"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Brand")));
    }

    state.db.delete_brand(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to delete brand");
        ApiError::Internal(anyhow::anyhow!("Failed to delete brand"))
    })?;

    Ok(Json(ApiResponse::ok((), "Brand deleted")))
}
