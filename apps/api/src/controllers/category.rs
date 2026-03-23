//! Category CRUD controller.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::models::_entities::categories;
use crate::views::{
    category::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/api/products/categories` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products/categories")
        .add("/", get(list))
        .add("/", post(create))
        .add("/:id", get(show))
        .add("/:id", patch(update))
        .add("/:id", delete(remove))
}

/// Produce a URL-friendly slug from a name.
fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// `GET /api/products/categories` — list categories with pagination and search.
async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page();
    let limit = params.limit();

    let mut condition = Condition::all();
    if let Some(ref search) = params.search {
        if !search.is_empty() {
            condition = condition.add(
                Condition::any()
                    .add(categories::Column::Name.contains(search))
                    .add(categories::Column::Slug.contains(search)),
            );
        }
    }

    let total = categories::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count categories");
            Error::InternalServerError
        })?;

    let mut query = categories::Entity::find().filter(condition);
    query = match params.sort_by.as_deref() {
        Some("name") if params.is_desc() => query.order_by_desc(categories::Column::Name),
        Some("name") => query.order_by_asc(categories::Column::Name),
        _ => query.order_by_desc(categories::Column::CreatedAt),
    };

    let items = query
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list categories");
            Error::InternalServerError
        })?;

    let responses: Vec<CategoryResponse> = items
        .into_iter()
        .map(CategoryResponse::from_model)
        .collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Categories retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/products/categories/:id` — fetch a single category.
async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let category = categories::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    match category {
        Some(c) => format::json(ApiResponse::ok(
            CategoryResponse::from_model(c),
            "Category retrieved",
        )),
        None => format::json(ApiResponse::<()>::not_found("Category")),
    }
}

/// `POST /api/products/categories` — create a new category.
async fn create(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Response> {
    if payload.name.trim().is_empty() {
        return format::json(ApiResponse::<()>::validation(vec![
            "name is required".to_string()
        ]));
    }

    // Validate parent_id if provided
    if let Some(parent_id) = payload.parent_id {
        let parent = categories::Entity::find_by_id(parent_id)
            .one(&ctx.db)
            .await
            .map_err(|_| Error::InternalServerError)?;
        if parent.is_none() {
            return format::json(ApiResponse::<()>::error(
                "invalid_parent",
                "Parent category not found",
            ));
        }
    }

    let user_id: Option<Uuid> = claims.sub.parse().ok();
    let slug = slugify(&payload.name);
    let now = chrono::Utc::now().fixed_offset();
    let id = Uuid::new_v4();
    let org_id = user_id.unwrap_or(Uuid::new_v4());

    let model = categories::ActiveModel {
        id: Set(id),
        name: Set(payload.name),
        slug: Set(slug),
        description: Set(payload.description),
        parent_id: Set(payload.parent_id),
        image_url: Set(payload.image_url),
        organization_id: Set(org_id),
        created_by: Set(user_id),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = model.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to create category");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::created(
        CategoryResponse::from_model(result),
        "Category created",
    ))
}

/// `PATCH /api/products/categories/:id` — update an existing category.
async fn update(
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Response> {
    let existing = categories::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(c) => c,
        None => return format::json(ApiResponse::<()>::not_found("Category")),
    };

    // Prevent circular parent references
    if let Some(parent_id) = payload.parent_id {
        if parent_id == id {
            return format::json(ApiResponse::<()>::error(
                "invalid_parent",
                "A category cannot be its own parent",
            ));
        }
    }

    let mut active: categories::ActiveModel = existing.into();

    if let Some(name) = payload.name {
        active.slug = Set(slugify(&name));
        active.name = Set(name);
    }
    if let Some(description) = payload.description {
        active.description = Set(Some(description));
    }
    if let Some(parent_id) = payload.parent_id {
        active.parent_id = Set(Some(parent_id));
    }
    if let Some(image_url) = payload.image_url {
        active.image_url = Set(Some(image_url));
    }

    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to update category");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::ok(
        CategoryResponse::from_model(updated),
        "Category updated",
    ))
}

/// `DELETE /api/products/categories/:id` — delete a category.
async fn remove(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let existing = categories::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if existing.is_none() {
        return format::json(ApiResponse::<()>::not_found("Category"));
    }

    // Check for child categories
    let children = categories::Entity::find()
        .filter(categories::Column::ParentId.eq(id))
        .count(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if children > 0 {
        return format::json(ApiResponse::<()>::error(
            "has_children",
            "Cannot delete a category that has child categories",
        ));
    }

    categories::Entity::delete_by_id(id)
        .exec(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete category");
            Error::InternalServerError
        })?;

    format::json(ApiResponse::<()>::ok((), "Category deleted"))
}
