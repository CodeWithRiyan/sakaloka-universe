//! Category CRUD controller.

use axum::{
    extract::{Extension, Path, Query, State},
    routing::get,
    Json, Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::{
    category::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest},
    slugify, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

/// Registers all `/products/categories` routes (nested under `/api` by the
/// top-level router).
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/products/categories", get(list))
        .route("/products/categories/{id}", get(show))
        .route(
            "/products/categories",
            axum::routing::post(create).layer(RequireScope::new(Scope::EntityWrite)),
        )
        .route(
            "/products/categories/{id}",
            axum::routing::patch(update).layer(RequireScope::new(Scope::EntityWrite)),
        )
        .route(
            "/products/categories/{id}",
            axum::routing::delete(remove).layer(RequireScope::new(Scope::EntityDelete)),
        )
}

/// `GET /api/products/categories` — list categories with pagination and
/// search.
async fn list(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<CategoryResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let total = state
        .db
        .count_categories(params.search.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count categories");
            ApiError::Internal(anyhow::anyhow!("Failed to count categories"))
        })?;

    let items = state
        .db
        .list_categories(
            limit,
            start,
            params.search.as_deref(),
            params.sort_by.as_deref().unwrap_or("created_at"),
            params.is_desc(),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list categories");
            ApiError::Internal(anyhow::anyhow!("Failed to list categories"))
        })?;

    let responses: Vec<CategoryResponse> = items.iter().map(CategoryResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Categories retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/products/categories/:id` — fetch a single category.
async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<CategoryResponse>>, ApiError> {
    let category = state.db.find_category(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find category");
        ApiError::Internal(anyhow::anyhow!("Failed to find category"))
    })?;

    match category {
        Some(c) => Ok(Json(ApiResponse::ok(
            CategoryResponse::from_model(&c),
            "Category retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Category"))),
    }
}

/// `POST /api/products/categories` — create a new category.
async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<ApiResponse<CategoryResponse>>, ApiError> {
    if payload.name.trim().is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "name is required".to_string()
        ])));
    }

    // Validate parent_id if provided
    if let Some(ref parent_id) = payload.parent_id {
        let parent = state.db.find_category(parent_id).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to validate parent category");
            ApiError::Internal(anyhow::anyhow!("Failed to validate parent category"))
        })?;
        if parent.is_none() {
            return Ok(Json(ApiResponse::error(
                "invalid_parent",
                "Parent category not found",
            )));
        }
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
        .unwrap_or_default();

    let result = state
        .db
        .create_category(
            &payload.name,
            &slug,
            &org_id,
            payload.description.as_deref(),
            payload.parent_id.as_deref(),
            payload.image_url.as_deref(),
            Some(claims.sub.as_str()),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create category");
            ApiError::Internal(anyhow::anyhow!("Failed to create category"))
        })?;

    Ok(Json(ApiResponse::created(
        CategoryResponse::from_model(&result),
        "Category created",
    )))
}

/// `PATCH /api/products/categories/:id` — update an existing category.
async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<ApiResponse<CategoryResponse>>, ApiError> {
    let existing = state.db.find_category(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find category for update");
        ApiError::Internal(anyhow::anyhow!("Failed to find category"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Category")));
    }

    // Prevent circular parent references
    if let Some(ref parent_id) = payload.parent_id {
        if *parent_id == id {
            return Ok(Json(ApiResponse::error(
                "invalid_parent",
                "A category cannot be its own parent",
            )));
        }
    }

    let slug = payload.name.as_ref().map(|n| slugify(n));

    let updated = state
        .db
        .update_category(
            &id,
            payload.name.as_deref(),
            slug.as_deref(),
            payload.description.as_deref(),
            payload.parent_id.as_deref(),
            payload.image_url.as_deref(),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update category");
            ApiError::Internal(anyhow::anyhow!("Failed to update category"))
        })?;

    Ok(Json(ApiResponse::ok(
        CategoryResponse::from_model(&updated),
        "Category updated",
    )))
}

/// `DELETE /api/products/categories/:id` — delete a category.
async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state.db.find_category(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find category for delete");
        ApiError::Internal(anyhow::anyhow!("Failed to find category"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Category")));
    }

    // Check for child categories
    let children = state.db.count_child_categories(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to count child categories");
        ApiError::Internal(anyhow::anyhow!("Failed to count child categories"))
    })?;

    if children > 0 {
        return Ok(Json(ApiResponse::error(
            "has_children",
            "Cannot delete a category that has child categories",
        )));
    }

    state.db.delete_category(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to delete category");
        ApiError::Internal(anyhow::anyhow!("Failed to delete category"))
    })?;

    Ok(Json(ApiResponse::ok((), "Category deleted")))
}
