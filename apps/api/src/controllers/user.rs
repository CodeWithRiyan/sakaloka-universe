//! User management controller.

use axum::{
    extract::{Extension, Path, Query, State},
    routing::get,
    Json, Router,
};
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::{
    user::{CreateUserRequest, UpdateUserRequest, UserResponse},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/users` routes (nested under `/api` by the top-level
/// router).
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/users", get(list))
        .route("/users/{id}", get(show))
        .route_layer(RequireScope::new(Scope::UserRead));

    let write_routes = Router::new()
        .route("/users", axum::routing::post(create))
        .route("/users/{id}", axum::routing::patch(update).delete(remove))
        .route_layer(RequireScope::new(Scope::UserManage));

    Router::new().merge(read_routes).merge(write_routes)
}

/// `GET /api/users` — list users with pagination and search.
async fn list(
    State(state): State<AppState>,
    Extension(_claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<UserResponse>>, ApiError> {
    let page = params.page();
    let limit = params.limit();
    let start = (page - 1) * limit;

    let total = state
        .db
        .count_users(params.search.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count users");
            ApiError::Internal(anyhow::anyhow!("Failed to count users"))
        })?;

    let items = state
        .db
        .list_users(
            limit,
            start,
            params.search.as_deref(),
            params.sort_by.as_deref().unwrap_or("created_at"),
            params.is_desc(),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list users");
            ApiError::Internal(anyhow::anyhow!("Failed to list users"))
        })?;

    let responses: Vec<UserResponse> = items.iter().map(UserResponse::from_model).collect();

    Ok(Json(PaginatedResponse {
        success: true,
        message: "Users retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    }))
}

/// `GET /api/users/:id` — fetch a single user.
async fn show(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    let user = state.db.find_user_by_id(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find user");
        ApiError::Internal(anyhow::anyhow!("Failed to find user"))
    })?;

    match user {
        Some(u) => Ok(Json(ApiResponse::ok(
            UserResponse::from_model(&u),
            "User retrieved",
        ))),
        None => Ok(Json(ApiResponse::not_found("User"))),
    }
}

/// `POST /api/users` — create a new user within the current organization.
async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    // Validate required fields
    let mut errors = Vec::new();
    if payload.email.trim().is_empty() {
        errors.push("email is required".to_string());
    }
    if payload.full_name.trim().is_empty() {
        errors.push("full_name is required".to_string());
    }
    if payload.password.is_empty() {
        errors.push("password is required".to_string());
    }
    if !errors.is_empty() {
        return Ok(Json(ApiResponse::validation(errors)));
    }

    // Check for duplicate email
    let existing = state
        .db
        .find_user_by_email(&payload.email)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("DB error")))?;

    if existing.is_some() {
        return Ok(Json(ApiResponse::error(
            "email_taken",
            "A user with this email already exists",
        )));
    }

    // Hash password
    let pw = sakaloka_secure::newtypes::Password::new(&payload.password).map_err(|e| {
        tracing::warn!(error = %e, "Password validation failed");
        ApiError::BadRequest("Password does not meet complexity requirements".to_string())
    })?;

    let password_hash = sakaloka_secure::argon2::hash_password(&pw)
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to hash password")))?;

    // Determine organization from the caller's context
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
        .create_user(
            &payload.email,
            &payload.full_name,
            &password_hash,
            &org_id,
            &payload.role_id,
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create user");
            ApiError::Internal(anyhow::anyhow!("Failed to create user"))
        })?;

    Ok(Json(ApiResponse::created(
        UserResponse::from_model(&result),
        "User created",
    )))
}

/// `PATCH /api/users/:id` — update an existing user.
async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    let existing = state.db.find_user_by_id(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find user for update");
        ApiError::Internal(anyhow::anyhow!("Failed to find user"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("User")));
    }

    // Check email uniqueness if being changed
    if let Some(ref email) = payload.email {
        let dup = state
            .db
            .find_user_by_email(email)
            .await
            .map_err(|_| ApiError::Internal(anyhow::anyhow!("DB error")))?;
        if let Some(ref dup_user) = dup {
            let dup_id = crate::views::record_id_to_string(&dup_user.id);
            if dup_id != id {
                return Ok(Json(ApiResponse::error(
                    "email_taken",
                    "A user with this email already exists",
                )));
            }
        }
    }

    // Hash new password if provided
    let password_hash = if let Some(ref password) = payload.password {
        let pw = sakaloka_secure::newtypes::Password::new(password).map_err(|_| {
            ApiError::BadRequest("Password does not meet complexity requirements".to_string())
        })?;
        Some(
            sakaloka_secure::argon2::hash_password(&pw)
                .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to hash password")))?,
        )
    } else {
        None
    };

    let updated = state
        .db
        .update_user(
            &id,
            payload.email.as_deref(),
            payload.full_name.as_deref(),
            password_hash.as_deref(),
            payload.role_id.as_deref(),
            payload.is_active,
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update user");
            ApiError::Internal(anyhow::anyhow!("Failed to update user"))
        })?;

    Ok(Json(ApiResponse::ok(
        UserResponse::from_model(&updated),
        "User updated",
    )))
}

/// `DELETE /api/users/:id` — deactivate a user.
async fn remove(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // Prevent self-deletion
    if claims.sub == id {
        return Ok(Json(ApiResponse::error(
            "self_deletion",
            "You cannot delete your own account",
        )));
    }

    let existing = state.db.find_user_by_id(&id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find user for deactivation");
        ApiError::Internal(anyhow::anyhow!("Failed to find user"))
    })?;

    if existing.is_none() {
        return Ok(Json(ApiResponse::not_found("User")));
    }

    // Soft-deactivate instead of hard delete
    state
        .db
        .update_user(&id, None, None, None, None, Some(false))
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to deactivate user");
            ApiError::Internal(anyhow::anyhow!("Failed to deactivate user"))
        })?;

    Ok(Json(ApiResponse::ok((), "User deactivated")))
}
