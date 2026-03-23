//! Role management controller.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::models::_entities::{roles, users};
use crate::views::{
    role::{CreateRoleRequest, PermissionsResponse, RoleResponse, UpdateRoleRequest},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/api/roles` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/roles")
        .add("/", get(list))
        .add("/", post(create))
        .add("/permissions", get(permissions))
        .add("/:id", get(show))
        .add("/:id", patch(update))
        .add("/:id", delete(remove))
}

/// All known permission strings for the Sakaloka platform.
const ALL_PERMISSIONS: &[&str] = &[
    "entity:read",
    "entity:write",
    "entity:delete",
    "search:read",
    "user:read",
    "user:manage",
    "db:read",
    "db:write",
    "db:admin",
    "zenoh:publish",
    "zenoh:subscribe",
];

/// `GET /api/roles` — list roles with pagination and search.
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
            condition = condition.add(roles::Column::Name.contains(search));
        }
    }

    let total = roles::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count roles");
            Error::InternalServerError
        })?;

    let mut query = roles::Entity::find().filter(condition);
    query = match params.sort_by.as_deref() {
        Some("name") if params.is_desc() => query.order_by_desc(roles::Column::Name),
        Some("name") => query.order_by_asc(roles::Column::Name),
        _ => query.order_by_desc(roles::Column::CreatedAt),
    };

    let items = query
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list roles");
            Error::InternalServerError
        })?;

    let responses: Vec<RoleResponse> = items.into_iter().map(RoleResponse::from_model).collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Roles retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/roles/permissions` — list all available permissions.
async fn permissions() -> Result<Response> {
    let perms = ALL_PERMISSIONS.iter().map(|s| s.to_string()).collect();

    format::json(ApiResponse::ok(
        PermissionsResponse { permissions: perms },
        "Permissions retrieved",
    ))
}

/// `GET /api/roles/:id` — fetch a single role.
async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let role = roles::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    match role {
        Some(r) => format::json(ApiResponse::ok(
            RoleResponse::from_model(r),
            "Role retrieved",
        )),
        None => format::json(ApiResponse::<()>::not_found("Role")),
    }
}

/// `POST /api/roles` — create a new role.
async fn create(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Response> {
    if payload.name.trim().is_empty() {
        return format::json(ApiResponse::<()>::validation(vec![
            "name is required".to_string()
        ]));
    }

    // Determine organization from caller
    let caller_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;
    let caller = users::Entity::find_by_id(caller_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or(Error::InternalServerError)?;

    // Check for duplicate name within the organization
    let existing = roles::Entity::find()
        .filter(roles::Column::Name.eq(&payload.name))
        .filter(roles::Column::OrganizationId.eq(caller.organization_id))
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if existing.is_some() {
        return format::json(ApiResponse::<()>::error(
            "duplicate_role",
            "A role with this name already exists in the organization",
        ));
    }

    let now = chrono::Utc::now().fixed_offset();
    let id = Uuid::new_v4();

    let model = roles::ActiveModel {
        id: Set(id),
        name: Set(payload.name),
        organization_id: Set(caller.organization_id),
        is_system_role: Set(false),
        permissions: Set(payload.permissions),
        created_by: Set(Some(caller_id)),
        is_active: Set(payload.is_active.unwrap_or(true)),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = model.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to create role");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::created(
        RoleResponse::from_model(result),
        "Role created",
    ))
}

/// `PATCH /api/roles/:id` — update an existing role.
async fn update(
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Response> {
    let existing = roles::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(r) => r,
        None => return format::json(ApiResponse::<()>::not_found("Role")),
    };

    // Prevent modification of system roles
    if existing.is_system_role {
        return format::json(ApiResponse::<()>::error(
            "system_role",
            "System roles cannot be modified",
        ));
    }

    let mut active: roles::ActiveModel = existing.into();

    if let Some(name) = payload.name {
        active.name = Set(name);
    }
    if let Some(permissions) = payload.permissions {
        active.permissions = Set(permissions);
    }
    if let Some(is_active) = payload.is_active {
        active.is_active = Set(is_active);
    }

    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to update role");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::ok(
        RoleResponse::from_model(updated),
        "Role updated",
    ))
}

/// `DELETE /api/roles/:id` — delete a role.
async fn remove(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let existing = roles::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(r) => r,
        None => return format::json(ApiResponse::<()>::not_found("Role")),
    };

    // Prevent deletion of system roles
    if existing.is_system_role {
        return format::json(ApiResponse::<()>::error(
            "system_role",
            "System roles cannot be deleted",
        ));
    }

    // Check for users assigned to this role
    let assigned_users = users::Entity::find()
        .filter(users::Column::RoleId.eq(id))
        .count(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if assigned_users > 0 {
        return format::json(ApiResponse::<()>::error(
            "role_in_use",
            "Cannot delete a role that is assigned to users",
        ));
    }

    roles::Entity::delete_by_id(id)
        .exec(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to delete role");
            Error::InternalServerError
        })?;

    format::json(ApiResponse::<()>::ok((), "Role deleted"))
}
