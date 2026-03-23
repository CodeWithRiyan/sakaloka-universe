//! Organization management controller.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::models::_entities::{organizations, users};
use crate::views::{
    organization::{CreateOrgRequest, OrganizationResponse, SelectOrgRequest, UpdateOrgRequest},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/api/organizations` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/organizations")
        .add("/", get(list))
        .add("/", post(create))
        .add("/current", get(current))
        .add("/select", post(select))
        .add("/:id", get(show))
        .add("/:id", patch(update))
        .add("/:id", delete(remove))
}

/// `GET /api/organizations` — list organizations the caller has access to.
async fn list(
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
                    .add(organizations::Column::Name.contains(search))
                    .add(organizations::Column::Code.contains(search)),
            );
        }
    }

    let total = organizations::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count organizations");
            Error::InternalServerError
        })?;

    let mut query = organizations::Entity::find().filter(condition);
    query = match params.sort_by.as_deref() {
        Some("name") if params.is_desc() => query.order_by_desc(organizations::Column::Name),
        Some("name") => query.order_by_asc(organizations::Column::Name),
        _ => query.order_by_desc(organizations::Column::CreatedAt),
    };

    let items = query
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list organizations");
            Error::InternalServerError
        })?;

    let responses: Vec<OrganizationResponse> = items
        .into_iter()
        .map(OrganizationResponse::from_model)
        .collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Organizations retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/organizations/current` — fetch the caller's current organization.
async fn current(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
) -> Result<Response> {
    let user_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;

    let user = users::Entity::find_by_id(user_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or(Error::InternalServerError)?;

    let org = organizations::Entity::find_by_id(user.organization_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    match org {
        Some(o) => format::json(ApiResponse::ok(
            OrganizationResponse::from_model(o),
            "Current organization retrieved",
        )),
        None => format::json(ApiResponse::<()>::not_found("Organization")),
    }
}

/// `POST /api/organizations/select` — switch the caller's active organization.
async fn select(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<SelectOrgRequest>,
) -> Result<Response> {
    // Verify the target organization exists and is active
    let org = organizations::Entity::find_by_id(payload.organization_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let org = match org {
        Some(o) if o.is_active => o,
        Some(_) => {
            return format::json(ApiResponse::<()>::error(
                "org_inactive",
                "The selected organization is not active",
            ))
        }
        None => return format::json(ApiResponse::<()>::not_found("Organization")),
    };

    // Update the user's organization_id
    let user_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;

    let user = users::Entity::find_by_id(user_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or(Error::InternalServerError)?;

    let mut active: users::ActiveModel = user.into();
    active.organization_id = Set(payload.organization_id);
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to switch organization");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::ok(
        OrganizationResponse::from_model(org),
        "Organization switched",
    ))
}

/// `GET /api/organizations/:id` — fetch a single organization.
async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let org = organizations::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    match org {
        Some(o) => format::json(ApiResponse::ok(
            OrganizationResponse::from_model(o),
            "Organization retrieved",
        )),
        None => format::json(ApiResponse::<()>::not_found("Organization")),
    }
}

/// `POST /api/organizations` — create a new organization.
async fn create(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateOrgRequest>,
) -> Result<Response> {
    if payload.name.trim().is_empty() {
        return format::json(ApiResponse::<()>::validation(vec![
            "name is required".to_string()
        ]));
    }

    let user_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;

    let now = chrono::Utc::now().fixed_offset();
    let id = Uuid::new_v4();

    let model = organizations::ActiveModel {
        id: Set(id),
        name: Set(payload.name),
        r#type: Set(payload.org_type),
        code: Set(payload.code),
        description: Set(payload.description),
        parent_id: Set(payload.parent_id),
        email: Set(payload.email),
        phone: Set(payload.phone),
        website: Set(payload.website),
        address: Set(payload.address),
        city: Set(payload.city),
        state: Set(payload.state),
        country: Set(payload.country),
        postal_code: Set(payload.postal_code),
        tax_number: Set(payload.tax_number),
        registration_number: Set(payload.registration_number),
        logo: Set(payload.logo),
        settings: Set(payload.settings),
        is_active: Set(true),
        owner_id: Set(Some(user_id)),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = model.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to create organization");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::created(
        OrganizationResponse::from_model(result),
        "Organization created",
    ))
}

/// `PATCH /api/organizations/:id` — update an existing organization.
async fn update(
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOrgRequest>,
) -> Result<Response> {
    let existing = organizations::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(o) => o,
        None => return format::json(ApiResponse::<()>::not_found("Organization")),
    };

    let mut active: organizations::ActiveModel = existing.into();

    if let Some(name) = payload.name {
        active.name = Set(name);
    }
    if let Some(org_type) = payload.org_type {
        active.r#type = Set(org_type);
    }
    if let Some(code) = payload.code {
        active.code = Set(Some(code));
    }
    if let Some(description) = payload.description {
        active.description = Set(Some(description));
    }
    if let Some(email) = payload.email {
        active.email = Set(Some(email));
    }
    if let Some(phone) = payload.phone {
        active.phone = Set(Some(phone));
    }
    if let Some(website) = payload.website {
        active.website = Set(Some(website));
    }
    if let Some(address) = payload.address {
        active.address = Set(Some(address));
    }
    if let Some(city) = payload.city {
        active.city = Set(Some(city));
    }
    if let Some(state) = payload.state {
        active.state = Set(Some(state));
    }
    if let Some(country) = payload.country {
        active.country = Set(Some(country));
    }
    if let Some(postal_code) = payload.postal_code {
        active.postal_code = Set(Some(postal_code));
    }
    if let Some(tax_number) = payload.tax_number {
        active.tax_number = Set(Some(tax_number));
    }
    if let Some(registration_number) = payload.registration_number {
        active.registration_number = Set(Some(registration_number));
    }
    if let Some(logo) = payload.logo {
        active.logo = Set(Some(logo));
    }
    if let Some(settings) = payload.settings {
        active.settings = Set(Some(settings));
    }
    if let Some(is_active) = payload.is_active {
        active.is_active = Set(is_active);
    }

    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to update organization");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::ok(
        OrganizationResponse::from_model(updated),
        "Organization updated",
    ))
}

/// `DELETE /api/organizations/:id` — deactivate an organization.
async fn remove(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let existing = organizations::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(o) => o,
        None => return format::json(ApiResponse::<()>::not_found("Organization")),
    };

    // Soft-deactivate
    let mut active: organizations::ActiveModel = existing.into();
    active.is_active = Set(false);
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to deactivate organization");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::<()>::ok((), "Organization deactivated"))
}
