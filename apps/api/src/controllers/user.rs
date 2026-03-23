//! User management controller.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use crate::models::_entities::users;
use crate::views::{
    user::{CreateUserRequest, UpdateUserRequest, UserResponse},
    ApiResponse, ListFilters, PageMeta, PaginatedData, PaginatedResponse, PaginationParams,
};

/// Registers all `/api/users` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/users")
        .add("/", get(list))
        .add("/", post(create))
        .add("/:id", get(show))
        .add("/:id", patch(update))
        .add("/:id", delete(remove))
}

/// `GET /api/users` — list users with pagination and search.
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
                    .add(users::Column::Email.contains(search))
                    .add(users::Column::FullName.contains(search)),
            );
        }
    }

    let total = users::Entity::find()
        .filter(condition.clone())
        .count(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to count users");
            Error::InternalServerError
        })?;

    let mut query = users::Entity::find().filter(condition);
    query = match params.sort_by.as_deref() {
        Some("email") if params.is_desc() => query.order_by_desc(users::Column::Email),
        Some("email") => query.order_by_asc(users::Column::Email),
        Some("full_name") if params.is_desc() => query.order_by_desc(users::Column::FullName),
        Some("full_name") => query.order_by_asc(users::Column::FullName),
        _ => query.order_by_desc(users::Column::CreatedAt),
    };

    let items = query
        .paginate(&ctx.db, limit)
        .fetch_page(params.offset())
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to list users");
            Error::InternalServerError
        })?;

    let responses: Vec<UserResponse> = items.into_iter().map(UserResponse::from_model).collect();

    format::json(PaginatedResponse {
        success: true,
        message: "Users retrieved".to_string(),
        data: PaginatedData {
            data: responses,
            pagination: PageMeta::new(page, limit, total),
            filters: ListFilters::from_params(&params),
        },
    })
}

/// `GET /api/users/:id` — fetch a single user.
async fn show(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<Response> {
    let user = users::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    match user {
        Some(u) => format::json(ApiResponse::ok(
            UserResponse::from_model(u),
            "User retrieved",
        )),
        None => format::json(ApiResponse::<()>::not_found("User")),
    }
}

/// `POST /api/users` — create a new user within the current organization.
async fn create(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Response> {
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
        return format::json(ApiResponse::<()>::validation(errors));
    }

    // Check for duplicate email
    let existing = users::Entity::find()
        .filter(users::Column::Email.eq(&payload.email))
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if existing.is_some() {
        return format::json(ApiResponse::<()>::error(
            "email_taken",
            "A user with this email already exists",
        ));
    }

    // Hash password
    let pw = sakaloka_secure::newtypes::Password::new(&payload.password).map_err(|e| {
        tracing::warn!(error = %e, "Password validation failed");
        Error::BadRequest("Password does not meet complexity requirements".to_string())
    })?;

    let password_hash =
        sakaloka_secure::argon2::hash_password(&pw).map_err(|_| Error::InternalServerError)?;

    // Determine organization from the caller's context
    let caller_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;
    let caller = users::Entity::find_by_id(caller_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or(Error::InternalServerError)?;

    let now = chrono::Utc::now().fixed_offset();
    let id = Uuid::new_v4();

    let model = users::ActiveModel {
        id: Set(id),
        email: Set(payload.email),
        full_name: Set(payload.full_name),
        password_hash: Set(password_hash),
        organization_id: Set(caller.organization_id),
        role_id: Set(payload.role_id),
        is_active: Set(payload.is_active.unwrap_or(true)),
        last_login_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let result = model.insert(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to create user");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::created(
        UserResponse::from_model(result),
        "User created",
    ))
}

/// `PATCH /api/users/:id` — update an existing user.
async fn update(
    State(ctx): State<AppContext>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Response> {
    let existing = users::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(u) => u,
        None => return format::json(ApiResponse::<()>::not_found("User")),
    };

    let mut active: users::ActiveModel = existing.into();

    if let Some(email) = payload.email {
        // Check uniqueness
        let dup = users::Entity::find()
            .filter(users::Column::Email.eq(&email))
            .filter(users::Column::Id.ne(id))
            .one(&ctx.db)
            .await
            .map_err(|_| Error::InternalServerError)?;
        if dup.is_some() {
            return format::json(ApiResponse::<()>::error(
                "email_taken",
                "A user with this email already exists",
            ));
        }
        active.email = Set(email);
    }
    if let Some(full_name) = payload.full_name {
        active.full_name = Set(full_name);
    }
    if let Some(password) = payload.password {
        let pw = sakaloka_secure::newtypes::Password::new(&password).map_err(|_| {
            Error::BadRequest("Password does not meet complexity requirements".to_string())
        })?;
        let hash =
            sakaloka_secure::argon2::hash_password(&pw).map_err(|_| Error::InternalServerError)?;
        active.password_hash = Set(hash);
    }
    if let Some(role_id) = payload.role_id {
        active.role_id = Set(role_id);
    }
    if let Some(is_active) = payload.is_active {
        active.is_active = Set(is_active);
    }

    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to update user");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::ok(
        UserResponse::from_model(updated),
        "User updated",
    ))
}

/// `DELETE /api/users/:id` — deactivate a user.
async fn remove(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
    Path(id): Path<Uuid>,
) -> Result<Response> {
    // Prevent self-deletion
    let caller_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;

    if caller_id == id {
        return format::json(ApiResponse::<()>::error(
            "self_deletion",
            "You cannot delete your own account",
        ));
    }

    let existing = users::Entity::find_by_id(id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let existing = match existing {
        Some(u) => u,
        None => return format::json(ApiResponse::<()>::not_found("User")),
    };

    // Soft-deactivate instead of hard delete
    let mut active: users::ActiveModel = existing.into();
    active.is_active = Set(false);
    active.updated_at = Set(chrono::Utc::now().fixed_offset());

    active.update(&ctx.db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to deactivate user");
        Error::InternalServerError
    })?;

    format::json(ApiResponse::<()>::ok((), "User deactivated"))
}
