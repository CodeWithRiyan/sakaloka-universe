//! Brand CRUD controller.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::models::_entities::brands;
use crate::views::{
    brand::{BrandResponse, CreateBrandRequest, UpdateBrandRequest},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/api/products/brands` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/products/brands")
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

/// `GET /api/products/brands` — list brands with pagination and search.
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
                    .add(brands::Column::Name.contains(search))
                    .add(brands::Column::Slug.contains(search)),
            );
        }
    }

    let total = brands::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count brands");
            Error::InternalServerError
        })?;

    let mut query = brands::Entity::find().filter(condition);
    query = match params.sort_by.as_deref() {
        Some("name") if params.is_desc() => query.order_by_desc(brands::Column::Name),
        Some("name") => query.order_by_asc(brands::Column::Name),
        _ => query.order_by_desc(brands::Column::CreatedAt),
    };

    let items = query
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list brands");
            Error::InternalServerError
        })?;

    let responses: Vec<BrandResponse> = items.into_iter().map(BrandResponse::from_model).collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Brands retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/products/brands/:id` — fetch a single brand.
async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let brand = brands::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    match brand {
        Some(b) => format::json(ApiResponse::ok(
            BrandResponse::from_model(b),
            "Brand retrieved",
        )),
        None => format::json(ApiResponse::<()>::not_found("Brand")),
    }
}

/// `POST /api/products/brands` — create a new brand.
async fn create(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateBrandRequest>,
) -> Result<Response> {
    if payload.name.trim().is_empty() {
        return format::json(ApiResponse::<()>::validation(vec![
            "name is required".to_string()
        ]));
    }

    let user_id: Option<Uuid> = claims.sub.parse().ok();
    let slug = slugify(&payload.name);
    let now = chrono::Utc::now().fixed_offset();
    let id = Uuid::new_v4();
    let org_id = user_id.unwrap_or(Uuid::new_v4());

    let model = brands::ActiveModel {
        id: Set(id),
        name: Set(payload.name),
        slug: Set(slug),
        description: Set(payload.description),
        logo: Set(payload.logo),
        website: Set(payload.website),
        organization_id: Set(org_id),
        created_by: Set(user_id),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = model.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to create brand");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::created(
        BrandResponse::from_model(result),
        "Brand created",
    ))
}

/// `PATCH /api/products/brands/:id` — update an existing brand.
async fn update(
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBrandRequest>,
) -> Result<Response> {
    let existing = brands::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(b) => b,
        None => return format::json(ApiResponse::<()>::not_found("Brand")),
    };

    let mut active: brands::ActiveModel = existing.into();

    if let Some(name) = payload.name {
        active.slug = Set(slugify(&name));
        active.name = Set(name);
    }
    if let Some(description) = payload.description {
        active.description = Set(Some(description));
    }
    if let Some(logo) = payload.logo {
        active.logo = Set(Some(logo));
    }
    if let Some(website) = payload.website {
        active.website = Set(Some(website));
    }

    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to update brand");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::ok(
        BrandResponse::from_model(updated),
        "Brand updated",
    ))
}

/// `DELETE /api/products/brands/:id` — delete a brand.
async fn remove(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let existing = brands::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if existing.is_none() {
        return format::json(ApiResponse::<()>::not_found("Brand"));
    }

    brands::Entity::delete_by_id(id)
        .exec(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete brand");
            Error::InternalServerError
        })?;

    format::json(ApiResponse::<()>::ok((), "Brand deleted"))
}
