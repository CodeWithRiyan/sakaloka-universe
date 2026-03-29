//! Handler functions for role endpoints.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::views::{
    role::{
        CreateRoleRequest, PermissionActionResponse, PermissionModuleResponse, RoleResponse,
        UpdateRoleRequest,
    },
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// `GET /api/roles` — list roles with pagination and search.
pub async fn list(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<RoleResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let search = params.search.as_deref();
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_desc = params.is_desc();
    let (count_result, list_result) = tokio::join!(
        state.db.count_roles(search),
        state
            .db
            .list_roles(limit, start, search, sort_by, sort_desc),
    );
    let total = count_result.map_err(|e| db_err(e, "Failed to count roles"))?;
    let items = list_result.map_err(|e| db_err(e, "Failed to list roles"))?;

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

/// `GET /api/roles/permissions` — list all available business permissions.
pub async fn permissions() -> Result<Json<ApiResponse<Vec<PermissionModuleResponse>>>, ApiError> {
    let modules = sakaloka_secure::rbac::permission::business_permission_catalog()
        .into_iter()
        .map(|module| PermissionModuleResponse {
            key: module.key.to_string(),
            label: module.label.to_string(),
            description: module.description.to_string(),
            permissions: module
                .permissions
                .iter()
                .map(|action| PermissionActionResponse {
                    key: action.key.to_string(),
                    label: action.label.to_string(),
                })
                .collect(),
        })
        .collect();

    Ok(Json(ApiResponse::ok(modules, "Permissions retrieved")))
}

/// `GET /api/roles/:id` — fetch a single role.
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<RoleResponse>>, ApiError> {
    let role = state
        .db
        .find_role(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find role"))?;

    match role {
        Some(r) => Ok(Json(ApiResponse::ok(
            RoleResponse::from_model(&r),
            "Role retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("Role"))),
    }
}

/// `POST /api/roles` — create a new role.
pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<ApiResponse<RoleResponse>>, ApiError> {
    if payload.name.trim().is_empty() {
        return Ok(Json(ApiResponse::validation(vec![
            "name is required".to_string()
        ])));
    }

    let org_id = crate::helpers::org_resolver::resolve_caller_org(&state, &claims).await?;

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

    // Validate that every permission entry is a known canonical scope.
    if let Err(unknown) =
        sakaloka_secure::rbac::permission::validate_business_permissions(&payload.permissions)
    {
        return Ok(Json(ApiResponse::validation(vec![format!(
            "Invalid permissions payload: {unknown}"
        )])));
    }

    let result = state
        .db
        .create_role(&payload.name, &org_id, &payload.permissions, false)
        .await
        .map_err(|e| db_err(e, "Failed to create role"))?;

    Ok(Json(ApiResponse::created(
        RoleResponse::from_model(&result),
        "Role created",
    )))
}

/// `PATCH /api/roles/:id` — update an existing role.
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<ApiResponse<RoleResponse>>, ApiError> {
    let existing = state
        .db
        .find_role(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find role"))?;

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

    // Validate incoming permissions before writing to the DB.
    if let Some(perms) = payload.permissions.as_ref() {
        if let Err(unknown) =
            sakaloka_secure::rbac::permission::validate_business_permissions(perms)
        {
            return Ok(Json(ApiResponse::validation(vec![format!(
                "Invalid permissions payload: {unknown}"
            )])));
        }
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
        .map_err(|e| db_err(e, "Failed to update role"))?;

    Ok(Json(ApiResponse::ok(
        RoleResponse::from_model(&updated),
        "Role updated",
    )))
}

/// `DELETE /api/roles/:id` — delete a role.
pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let existing = state
        .db
        .find_role(&id)
        .await
        .map_err(|e| db_err(e, "Failed to find role"))?;

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
    let assigned_users = state
        .db
        .count_users_with_role(&id)
        .await
        .map_err(|e| db_err(e, "Failed to count users with role"))?;

    if assigned_users > 0 {
        return Ok(Json(ApiResponse::error(
            "role_in_use",
            "Cannot delete a role that is assigned to users",
        )));
    }

    state
        .db
        .delete_role(&id)
        .await
        .map_err(|e| db_err(e, "Failed to delete role"))?;

    Ok(Json(ApiResponse::ok((), "Role deleted")))
}
