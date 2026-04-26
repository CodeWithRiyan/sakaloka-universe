//! Handler functions for category endpoints.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::helpers::patch_builder::PatchBuilder;
use crate::views::{
    category::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest},
    slugify, ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse,
    PaginationParams,
};

#[utoipa::path(
    get,
    path = "/api/products/categories",
    tag = "Categories",
    params(PaginationParams),
    responses(
        (status = 200, description = "Success", body = PaginatedResponse<CategoryResponse>)
    ),
    security(("bearer" = []))
)]
/// `GET /api/products/categories` — list categories with pagination and
/// search.
pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<CategoryResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let search = params.search.as_deref();
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_desc = params.is_desc();
    let org_id = claims.org_id.as_deref();
    let (count_result, list_result) = tokio::join!(
        state.db.count_categories(org_id, search),
        state
            .db
            .list_categories(org_id, limit, start, search, sort_by, sort_desc),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count categories"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to list categories"))?;

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

#[utoipa::path(
    get,
    path = "/api/products/categories/{id}",
    tag = "Categories",
    params(("id" = String, Path, description = "Category ID")),
    responses(
        (status = 200, description = "Success", body = ApiResponse<CategoryResponse>),
        (status = 404, description = "Not found", body = crate::views::ErrorResponse)
    ),
    security(("bearer" = []))
)]
/// `GET /api/products/categories/:id` — fetch a single category.
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<CategoryResponse>>, ApiError> {
    let category = state
        .db
        .find_category(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find category"))?;

    match category {
        Some(c) => Ok(Json(ApiResponse::ok(
            CategoryResponse::from_model(&c),
            "Category retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Category"))),
    }
}

#[utoipa::path(
    post,
    path = "/api/products/categories",
    tag = "Categories",
    request_body = CreateCategoryRequest,
    responses(
        (status = 200, description = "Success", body = ApiResponse<CategoryResponse>),
        (status = 400, description = "Validation error", body = crate::views::ErrorResponse)
    ),
    security(("bearer" = []))
)]
/// `POST /api/products/categories` — create a new category.
pub async fn create(
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
        let parent = state
            .db
            .find_category(parent_id)
            .await
            .map_err(|e| db_err(e, "Failed to validate parent category"))?;
        if parent.is_none() {
            return Ok(Json(ApiResponse::error(
                "invalid_parent",
                "Parent category not found",
            )));
        }
    }

    let slug = slugify(&payload.name);
    let org_id = crate::helpers::org_resolver::resolve_caller_org(&state, &claims).await?;

    let result = state
        .db
        .create_category(
            &payload.name,
            &slug,
            &org_id,
            payload.description.as_deref(),
            payload.parent_id.as_deref(),
            payload.image_url.as_deref(),
            0,
            &claims.sub,
        )
        .await
        .map_err(|e| db_err(e, "Failed to create category"))?;

    Ok(Json(ApiResponse::created(
        CategoryResponse::from_model(&result),
        "Category created",
    )))
}

#[utoipa::path(
    patch,
    path = "/api/products/categories/{id}",
    tag = "Categories",
    params(("id" = String, Path, description = "Category ID")),
    request_body = UpdateCategoryRequest,
    responses(
        (status = 200, description = "Success", body = ApiResponse<CategoryResponse>),
        (status = 404, description = "Not found", body = crate::views::ErrorResponse)
    ),
    security(("bearer" = []))
)]
/// `PATCH /api/products/categories/:id` — update an existing category.
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<ApiResponse<CategoryResponse>>, ApiError> {
    let existing = state
        .db
        .find_category(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find category"))?;

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

    let updates = PatchBuilder::new()
        .set_string("name", payload.name)
        .set_string("slug", slug)
        .set_string("description", payload.description)
        .set_string("parent_id", payload.parent_id)
        .set_string("image_url", payload.image_url)
        .build();

    let updated = state
        .db
        .update_category(&id, &updates)
        .await
        .map_err(|e| db_err(e, "Failed to update category"))?;

    Ok(Json(ApiResponse::ok(
        CategoryResponse::from_model(&updated),
        "Category updated",
    )))
}

#[utoipa::path(
    delete,
    path = "/api/products/categories/{id}",
    tag = "Categories",
    params(("id" = String, Path, description = "Category ID")),
    responses(
        (status = 200, description = "Success"),
        (status = 404, description = "Not found", body = crate::views::ErrorResponse)
    ),
    security(("bearer" = []))
)]
/// `DELETE /api/products/categories/:id` — delete a category.
pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state
        .db
        .find_category(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find category"))?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("Category")));
    }

    // Check for child categories
    let children = state
        .db
        .count_child_categories(&id)
        .await
        .map_err(|e| db_err(e, "Failed to count child categories"))?;

    if children > 0 {
        return Ok(Json(ApiResponse::error(
            "has_children",
            "Cannot delete a category that has child categories",
        )));
    }

    state
        .db
        .delete_category(&id)
        .await
        .map_err(|e| db_err(e, "Failed to delete category"))?;

    Ok(Json(ApiResponse::ok((), "Category deleted")))
}
