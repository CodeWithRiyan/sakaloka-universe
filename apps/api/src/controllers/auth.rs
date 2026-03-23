//! Authentication controller — login, register, refresh, and profile.

use axum::extract::Extension;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::_entities::{organizations, roles, users};
use crate::views::{
    auth::{
        LoginRequest, LoginResponse, OrgSummary, RefreshRequest, RegisterRequest, RoleSummary,
        UserProfile,
    },
    ApiResponse,
};

/// Registers all `/api/auth` routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/auth")
        .add("/login", post(login))
        .add("/register", post(register))
        .add("/refresh", post(refresh))
        .add("/profile", get(profile))
}

/// `POST /api/auth/login` — authenticate a user and return tokens.
async fn login(
    State(ctx): State<AppContext>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response> {
    // 1. Find user by email
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(&payload.email))
        .one(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "DB error during login lookup");
            Error::InternalServerError
        })?;

    let user = match user {
        Some(u) => u,
        None => {
            return format::json(ApiResponse::<()>::error(
                "invalid_credentials",
                "Wrong email or password",
            ))
        }
    };

    if !user.is_active {
        return format::json(ApiResponse::<()>::error(
            "account_disabled",
            "Account is disabled",
        ));
    }

    // 2. Verify password with Argon2id
    let pw = match sakaloka_secure::newtypes::Password::new(&payload.password) {
        Ok(p) => p,
        Err(_) => {
            return format::json(ApiResponse::<()>::error(
                "invalid_credentials",
                "Wrong email or password",
            ))
        }
    };

    let valid = sakaloka_secure::argon2::verify_password(&pw, &user.password_hash).unwrap_or(false);

    if !valid {
        return format::json(ApiResponse::<()>::error(
            "invalid_credentials",
            "Wrong email or password",
        ));
    }

    // 3. Load organization and role for the profile
    let org = organizations::Entity::find_by_id(user.organization_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let role = roles::Entity::find_by_id(user.role_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let org = org.ok_or(Error::InternalServerError)?;
    let role = role.ok_or(Error::InternalServerError)?;

    // 4. Issue JWT
    let jwt_keys =
        sakaloka_secure::jwt::JwtKeys::from_env().map_err(|_| Error::InternalServerError)?;
    let user_id = sakaloka_secure::newtypes::UserId::new(&user.id.to_string())
        .map_err(|_| Error::InternalServerError)?;
    let session_id = sakaloka_secure::newtypes::SessionId::new();

    let rbac_role = match role.name.to_lowercase().as_str() {
        "admin" | "owner" => sakaloka_secure::rbac::Role::Admin,
        "editor" | "manager" | "cashier" => sakaloka_secure::rbac::Role::Editor,
        _ => sakaloka_secure::rbac::Role::Viewer,
    };

    let scopes: Vec<String> = sakaloka_secure::rbac::matrix::allowed_scopes(&rbac_role)
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();

    let access_token = sakaloka_secure::jwt::user_claims::issue_user_token(
        &jwt_keys,
        &user_id,
        &role.name,
        &scope_refs,
        &session_id,
    )
    .map_err(|_| Error::InternalServerError)?;

    // 5. Generate refresh token
    let refresh_token = sakaloka_secure::newtypes::TokenId::new().to_string();

    // 6. Update last_login_at
    let mut active: users::ActiveModel = user.clone().into();
    active.last_login_at = Set(Some(chrono::Utc::now().into()));
    users::Entity::update(active)
        .exec(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let response = LoginResponse {
        access_token,
        refresh_token,
        user: UserProfile {
            id: user.id.to_string(),
            email: user.email,
            full_name: user.full_name,
            organization: OrgSummary {
                id: org.id.to_string(),
                name: org.name,
                org_type: org.r#type,
            },
            role: RoleSummary {
                id: role.id.to_string(),
                name: role.name,
                permissions: role.permissions,
            },
            preferences: serde_json::json!({}),
        },
    };

    format::json(ApiResponse::ok(response, "Login successful"))
}

/// `POST /api/auth/register` — create a new user and organization.
async fn register(
    State(ctx): State<AppContext>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Response> {
    // 1. Check if email already exists
    let existing = users::Entity::find()
        .filter(users::Column::Email.eq(&payload.email))
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if existing.is_some() {
        return format::json(ApiResponse::<()>::error(
            "email_taken",
            "An account with this email already exists",
        ));
    }

    // 2. Validate password complexity
    let pw = match sakaloka_secure::newtypes::Password::new(&payload.password) {
        Ok(p) => p,
        Err(e) => {
            return format::json(ApiResponse::<()>::error("validation_error", &e.to_string()))
        }
    };

    // 3. Hash the password
    let password_hash =
        sakaloka_secure::argon2::hash_password(&pw).map_err(|_| Error::InternalServerError)?;

    // 4. Create the organization
    let org_id = Uuid::new_v4();
    let now = chrono::Utc::now().fixed_offset();
    let org = organizations::ActiveModel {
        id: Set(org_id),
        name: Set(payload.organization_name.clone()),
        r#type: Set("default".to_string()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    organizations::Entity::insert(org)
        .exec(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create organization");
            Error::InternalServerError
        })?;

    // 5. Create a default admin role for the organization
    let role_id = Uuid::new_v4();
    let admin_permissions = serde_json::json!({
        "entity:read": true,
        "entity:write": true,
        "entity:delete": true,
        "user:read": true,
        "user:manage": true,
    });
    let role = roles::ActiveModel {
        id: Set(role_id),
        name: Set("admin".to_string()),
        organization_id: Set(org_id),
        is_system_role: Set(true),
        permissions: Set(admin_permissions.clone()),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    roles::Entity::insert(role)
        .exec(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create role");
            Error::InternalServerError
        })?;

    // 6. Create the user
    let user_id = Uuid::new_v4();
    let user = users::ActiveModel {
        id: Set(user_id),
        email: Set(payload.email.clone()),
        full_name: Set(payload.full_name.clone()),
        password_hash: Set(password_hash),
        organization_id: Set(org_id),
        role_id: Set(role_id),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    users::Entity::insert(user)
        .exec(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create user");
            Error::InternalServerError
        })?;

    // 7. Set organization owner
    let org_update = organizations::ActiveModel {
        id: Set(org_id),
        owner_id: Set(Some(user_id)),
        ..Default::default()
    };
    organizations::Entity::update(org_update)
        .exec(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    // 8. Issue tokens
    let jwt_keys =
        sakaloka_secure::jwt::JwtKeys::from_env().map_err(|_| Error::InternalServerError)?;
    let uid = sakaloka_secure::newtypes::UserId::new(&user_id.to_string())
        .map_err(|_| Error::InternalServerError)?;
    let session_id = sakaloka_secure::newtypes::SessionId::new();

    let scopes: Vec<String> =
        sakaloka_secure::rbac::matrix::allowed_scopes(&sakaloka_secure::rbac::Role::Admin)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
    let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();

    let access_token = sakaloka_secure::jwt::user_claims::issue_user_token(
        &jwt_keys,
        &uid,
        "admin",
        &scope_refs,
        &session_id,
    )
    .map_err(|_| Error::InternalServerError)?;

    let refresh_token = sakaloka_secure::newtypes::TokenId::new().to_string();

    let response = LoginResponse {
        access_token,
        refresh_token,
        user: UserProfile {
            id: user_id.to_string(),
            email: payload.email,
            full_name: payload.full_name,
            organization: OrgSummary {
                id: org_id.to_string(),
                name: payload.organization_name,
                org_type: "default".to_string(),
            },
            role: RoleSummary {
                id: role_id.to_string(),
                name: "admin".to_string(),
                permissions: admin_permissions,
            },
            preferences: serde_json::json!({}),
        },
    };

    format::json(ApiResponse::created(response, "Registration successful"))
}

/// `POST /api/auth/refresh` — exchange a refresh token for new tokens.
async fn refresh(
    State(_ctx): State<AppContext>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Response> {
    // In a full implementation, this would:
    // 1. Hash the incoming refresh token
    // 2. Look up the session by hashed token
    // 3. Check for reuse (token already rotated => terminate all sessions)
    // 4. Rotate the token
    // 5. Issue a new access token

    // For now, validate that the token is non-empty and return a structured
    // error.  The full rotation logic lives in sakaloka_secure::tokens::rotation.
    if payload.refresh_token.is_empty() {
        return format::json(ApiResponse::<()>::error(
            "invalid_token",
            "Refresh token is required",
        ));
    }

    // Hash the incoming token to look up the session
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(payload.refresh_token.as_bytes());
    let _token_hash = format!("{:x}", hasher.finalize());

    // TODO: Complete refresh token rotation once session persistence is wired
    // up via SeaORM.  The sakaloka_secure::tokens::rotation module handles
    // reuse detection and session termination.

    format::json(ApiResponse::<()>::error(
        "not_implemented",
        "Refresh token rotation is not yet implemented via SeaORM",
    ))
}

/// `GET /api/auth/profile` — return the authenticated user's profile.
async fn profile(
    State(ctx): State<AppContext>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
) -> Result<Response> {
    // Parse user ID from claims subject
    let user_id: Uuid = claims.sub.parse().map_err(|_| Error::InternalServerError)?;

    let user = users::Entity::find_by_id(user_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    let user = match user {
        Some(u) => u,
        None => return format::json(ApiResponse::<()>::not_found("User")),
    };

    let org = organizations::Entity::find_by_id(user.organization_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or(Error::InternalServerError)?;

    let role = roles::Entity::find_by_id(user.role_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or(Error::InternalServerError)?;

    let profile = UserProfile {
        id: user.id.to_string(),
        email: user.email,
        full_name: user.full_name,
        organization: OrgSummary {
            id: org.id.to_string(),
            name: org.name,
            org_type: org.r#type,
        },
        role: RoleSummary {
            id: role.id.to_string(),
            name: role.name,
            permissions: role.permissions,
        },
        preferences: serde_json::json!({}),
    };

    format::json(ApiResponse::ok(profile, "Profile loaded"))
}
