//! Role management controller.

use axum::{
    extract::{Extension, Path, Query, State},
    routing::get,
    Json, Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::{
    role::{CreateRoleRequest, PermissionsResponse, RoleResponse, UpdateRoleRequest},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/roles` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/roles", get(list))
        .route("/roles/permissions", get(permissions))
        .route("/roles/{id}", get(show))
        .route_layer(RequireScope::new(Scope::UserRead));

    let write_routes = Router::new()
        .route("/roles", axum::routing::post(create))
        .route("/roles/{id}", axum::routing::patch(update).delete(remove))
        .route_layer(RequireScope::new(Scope::UserManage));

    Router::new().merge(read_routes).merge(write_routes)
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
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<RoleResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let total = state
        .db
        .count_roles(params.search.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count roles");
            ApiError::Internal(anyhow::anyhow!("Failed to count roles"))
        })?;

    let items = state
        .db
        .list_roles(
            limit,
            start,
            params.search.as_deref(),
            params.sort_by.as_deref().unwrap_or("created_at"),
            params.is_desc(),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list roles");
            ApiError::Internal(anyhow::anyhow!("Failed to list roles"))
        })?;

    let responses: Vec<RoleResponse> = items.iter().map(RoleResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Roles retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/roles/permissions` — list all available permissions.
async fn permissions() -> Result<Json<ApiResponse<PermissionsResponse>>, ApiError> {
    let perms = ALL_PERMISSIONS.iter().map(|s| s.to_string()).collect();

    Ok(Json(ApiResponse::ok(
        PermissionsResponse { permissions: perms },
        "Permissions retrieved",
    )))
}

/// `GET /api/roles/:id` — fetch a single role.
async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<RoleResponse>>, ApiError> {
    let role = state.db.find_role(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find role");
        ApiError::Internal(anyhow::anyhow!("Failed to find role"))
    })?;

    match role {
        Some(r) => Ok(Json(ApiResponse::ok(
            RoleResponse::from_model(&r),
            "Role retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Role"))),
    }
}

/// `POST /api/roles` — create a new role.
async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<ApiResponse<RoleResponse>>, ApiError> {
    if payload.name.trim().is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "name is required".to_string()
        ])));
    }

    // Determine organization from caller
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

    // Check for duplicate name within the organization
    let existing = state
        .db
        .find_role_by_name_and_org(&payload.name, &org_id)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("DB error")))?;

    if existing.is_some() {
        return Ok(Json(ApiResponse::error(
            "duplicate_role",
            "A role with this name already exists in the organization",
        )));
    }

    let result = state
        .db
        .create_role(&payload.name, &org_id, &payload.permissions, false)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create role");
            ApiError::Internal(anyhow::anyhow!("Failed to create role"))
        })?;

    Ok(Json(ApiResponse::created(
        RoleResponse::from_model(&result),
        "Role created",
    )))
}

/// `PATCH /api/roles/:id` — update an existing role.
async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<ApiResponse<RoleResponse>>, ApiError> {
    let existing = state.db.find_role(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find role for update");
        ApiError::Internal(anyhow::anyhow!("Failed to find role"))
    })?;

    let existing = match existing {
        Some(r) => r,
        None => return Ok(Json(ApiResponse::not_found("Role"))),
    };

    // Prevent modification of system roles
    if existing.is_system_role {
        return Ok(Json(ApiResponse::error(
            "system_role",
            "System roles cannot be modified",
        )));
    }

    let updated = state
        .db
        .update_role(
            &id,
            payload.name.as_deref(),
            payload.permissions.as_ref(),
            payload.is_active,
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update role");
            ApiError::Internal(anyhow::anyhow!("Failed to update role"))
        })?;

    Ok(Json(ApiResponse::ok(
        RoleResponse::from_model(&updated),
        "Role updated",
    )))
}

/// `DELETE /api/roles/:id` — delete a role.
async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state.db.find_role(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find role for delete");
        ApiError::Internal(anyhow::anyhow!("Failed to find role"))
    })?;

    let existing = match existing {
        Some(r) => r,
        None => return Ok(Json(ApiResponse::not_found("Role"))),
    };

    // Prevent deletion of system roles
    if existing.is_system_role {
        return Ok(Json(ApiResponse::error(
            "system_role",
            "System roles cannot be deleted",
        )));
    }

    // Check for users assigned to this role
    let assigned_users = state.db.count_users_with_role(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to count users with role");
        ApiError::Internal(anyhow::anyhow!("Failed to count users with role"))
    })?;

    if assigned_users > 0 {
        return Ok(Json(ApiResponse::error(
            "role_in_use",
            "Cannot delete a role that is assigned to users",
        )));
    }

    state.db.delete_role(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to delete role");
        ApiError::Internal(anyhow::anyhow!("Failed to delete role"))
    })?;

    Ok(Json(ApiResponse::ok((), "Role deleted")))
}
