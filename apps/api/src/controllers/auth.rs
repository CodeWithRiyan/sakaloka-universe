//! Authentication controller — login, register, refresh, and profile.

use axum::{
    extract::{Extension, State},
    routing::{get, post},
    Json, Router,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::{
    auth::{
        LoginRequest, LoginResponse, OrgSummary, RefreshRequest, RegisterRequest, RoleSummary,
        UserProfile,
    },
    record_id_to_string, ApiResponse,
};

/// Public `/auth` routes — no JWT required.
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/refresh", post(refresh))
}

/// Protected `/auth` routes — require valid JWT.
pub fn protected_routes() -> Router<AppState> {
    Router::new().route("/auth/profile", get(profile))
}

/// `POST /api/auth/login` — authenticate a user and return tokens.
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    // 1. Find user by email
    let user = state
        .db
        .find_user_by_email(&payload.email)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "DB error during login lookup");
            ApiError::Internal(anyhow::anyhow!("DB error during login lookup"))
        })?;

    let user = match user {
        Some(u) => u,
        None => {
            return Ok(Json(ApiResponse::error(
                "invalid_credentials",
                "Wrong email or password",
            )))
        }
    };

    if !user.is_active {
        return Ok(Json(ApiResponse::error(
            "account_disabled",
            "Account is disabled",
        )));
    }

    // 2. Verify password with Argon2id
    let pw = match sakaloka_secure::newtypes::Password::new(&payload.password) {
        Ok(p) => p,
        Err(_) => {
            return Ok(Json(ApiResponse::error(
                "invalid_credentials",
                "Wrong email or password",
            )))
        }
    };

    let valid = match sakaloka_secure::argon2::verify_password(&pw, &user.password_hash) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(error = %e, "Password verification error");
            return Err(ApiError::Internal(anyhow::anyhow!(
                "Password verification failed"
            )));
        }
    };

    if !valid {
        return Ok(Json(ApiResponse::error(
            "invalid_credentials",
            "Wrong email or password",
        )));
    }

    // 3. Load organization and role for the profile
    let user_id_str = record_id_to_string(&user.id);

    let org_id = match user.organization_id.as_ref() {
        Some(id) => record_id_to_string(id),
        None => {
            return Ok(Json(ApiResponse::error(
                "incomplete_profile",
                "User account has no organization assigned. Contact an administrator.",
            )))
        }
    };
    let role_id = match user.role_id.as_ref() {
        Some(id) => record_id_to_string(id),
        None => {
            return Ok(Json(ApiResponse::error(
                "incomplete_profile",
                "User account has no role assigned. Contact an administrator.",
            )))
        }
    };

    let org = state.db.find_organization(&org_id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to load organization");
        ApiError::Internal(anyhow::anyhow!("Failed to load organization"))
    })?;

    let role = state.db.find_role(&role_id).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to load role");
        ApiError::Internal(anyhow::anyhow!("Failed to load role"))
    })?;

    let org = org.ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Organization not found")))?;
    let role = role.ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Role not found")))?;

    // 4. Issue JWT
    let uid = sakaloka_secure::newtypes::UserId::new(&user_id_str)
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Invalid user ID")))?;
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
        &state.jwt_keys,
        &uid,
        &role.name,
        &scope_refs,
        &session_id,
    )
    .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to issue JWT")))?;

    // 5. Generate refresh token and persist session
    let refresh_token = sakaloka_secure::newtypes::TokenId::new().to_string();
    let token_hash = sakaloka_secure::tokens::rotation::hash_refresh_token(&refresh_token);

    if let Err(e) = state
        .db
        .create_session(&uid, &session_id, &token_hash)
        .await
    {
        tracing::error!(error = %e, "Failed to persist session during login");
        return Err(ApiError::Internal(anyhow::anyhow!(
            "Failed to create session"
        )));
    }

    // 6. Update last_login_at (best-effort, log on failure)
    if let Err(e) = state.db.update_last_login(&user_id_str).await {
        tracing::warn!(error = %e, user_id = %user_id_str, "Failed to update last_login_at");
    }

    let response = LoginResponse {
        access_token,
        refresh_token,
        user: UserProfile {
            id: user_id_str,
            email: user.email,
            full_name: user.full_name.unwrap_or_default(),
            organization: OrgSummary {
                id: record_id_to_string(&org.id),
                name: org.name,
                org_type: org.org_type,
            },
            role: RoleSummary {
                id: record_id_to_string(&role.id),
                name: role.name,
                permissions: role.permissions.unwrap_or_else(|| serde_json::json!({})),
            },
            preferences: serde_json::json!({}),
        },
    };

    Ok(Json(ApiResponse::ok(response, "Login successful")))
}

/// `POST /api/auth/register` — create a new user and organization.
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    // 1. Check if email already exists
    let existing = state
        .db
        .find_user_by_email(&payload.email)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "DB error checking email");
            ApiError::Internal(anyhow::anyhow!("DB error"))
        })?;

    if existing.is_some() {
        return Ok(Json(ApiResponse::error(
            "email_taken",
            "An account with this email already exists",
        )));
    }

    // 2. Validate password complexity
    let pw = match sakaloka_secure::newtypes::Password::new(&payload.password) {
        Ok(p) => p,
        Err(e) => return Ok(Json(ApiResponse::error("validation_error", &e.to_string()))),
    };

    // 3. Hash the password
    let password_hash = sakaloka_secure::argon2::hash_password(&pw)
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to hash password")))?;

    // 4. Create the organization
    let org = state
        .db
        .create_organization(&payload.organization_name, "company", None)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create organization");
            ApiError::Internal(anyhow::anyhow!("Failed to create organization"))
        })?;

    let org_id_str = record_id_to_string(&org.id);

    // 5. Create a default admin role for the organization
    let admin_permissions = serde_json::json!({
        "entity:read": true,
        "entity:write": true,
        "entity:delete": true,
        "user:read": true,
        "user:manage": true,
    });
    let role = state
        .db
        .create_role("admin", &org_id_str, &admin_permissions, true)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create role");
            ApiError::Internal(anyhow::anyhow!("Failed to create role"))
        })?;

    let role_id_str = record_id_to_string(&role.id);

    // 6. Create the user
    let user = state
        .db
        .create_user(
            &payload.email,
            &payload.full_name,
            &password_hash,
            &org_id_str,
            &role_id_str,
        )
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create user");
            ApiError::Internal(anyhow::anyhow!("Failed to create user"))
        })?;

    let user_id_str = record_id_to_string(&user.id);

    // 7. Set organization owner
    if let Err(e) = state
        .db
        .update_organization_owner(&org_id_str, &user_id_str)
        .await
    {
        tracing::error!(error = %e, org_id = %org_id_str, "Failed to set organization owner during registration");
        return Err(ApiError::Internal(anyhow::anyhow!(
            "Failed to set organization owner"
        )));
    }

    // 8. Issue tokens
    let uid = sakaloka_secure::newtypes::UserId::new(&user_id_str)
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Invalid user ID")))?;
    let session_id = sakaloka_secure::newtypes::SessionId::new();

    let scopes: Vec<String> =
        sakaloka_secure::rbac::matrix::allowed_scopes(&sakaloka_secure::rbac::Role::Admin)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
    let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();

    let access_token = sakaloka_secure::jwt::user_claims::issue_user_token(
        &state.jwt_keys,
        &uid,
        "admin",
        &scope_refs,
        &session_id,
    )
    .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to issue JWT")))?;

    let refresh_token = sakaloka_secure::newtypes::TokenId::new().to_string();
    let token_hash = sakaloka_secure::tokens::rotation::hash_refresh_token(&refresh_token);

    if let Err(e) = state
        .db
        .create_session(&uid, &session_id, &token_hash)
        .await
    {
        tracing::error!(error = %e, "Failed to persist session during registration");
        return Err(ApiError::Internal(anyhow::anyhow!(
            "Failed to create session"
        )));
    }

    let response = LoginResponse {
        access_token,
        refresh_token,
        user: UserProfile {
            id: user_id_str,
            email: payload.email,
            full_name: payload.full_name,
            organization: OrgSummary {
                id: org_id_str,
                name: payload.organization_name,
                org_type: "company".to_string(),
            },
            role: RoleSummary {
                id: role_id_str,
                name: "admin".to_string(),
                permissions: admin_permissions,
            },
            preferences: serde_json::json!({}),
        },
    };

    Ok(Json(ApiResponse::created(
        response,
        "Registration successful",
    )))
}

/// `POST /api/auth/refresh` — exchange a refresh token for new tokens.
async fn refresh(
    State(_state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // In a full implementation, this would:
    // 1. Hash the incoming refresh token
    // 2. Look up the session by hashed token
    // 3. Check for reuse (token already rotated => terminate all sessions)
    // 4. Rotate the token
    // 5. Issue a new access token

    // For now, validate that the token is non-empty and return a structured
    // error.  The full rotation logic lives in sakaloka_secure::tokens::rotation.
    if payload.refresh_token.is_empty() {
        return Ok(Json(ApiResponse::error(
            "invalid_token",
            "Refresh token is required",
        )));
    }

    // TODO: Complete refresh token rotation once session persistence is wired
    // up via SurrealDB.  The sakaloka_secure::tokens::rotation module handles
    // reuse detection, token hashing, and session termination.

    Ok(Json(ApiResponse::error(
        "not_implemented",
        "Refresh token rotation is not yet implemented",
    )))
}

/// `GET /api/auth/profile` — return the authenticated user's profile.
async fn profile(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
) -> Result<Json<ApiResponse<UserProfile>>, ApiError> {
    let user = state.db.find_user_by_id(&claims.sub).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to find user");
        ApiError::Internal(anyhow::anyhow!("Failed to find user"))
    })?;

    let user = match user {
        Some(u) => u,
        None => return Ok(Json(ApiResponse::not_found("User"))),
    };

    let user_id_str = record_id_to_string(&user.id);

    let org_id = user
        .organization_id
        .as_ref()
        .map(record_id_to_string)
        .unwrap_or_default();
    let role_id = user
        .role_id
        .as_ref()
        .map(record_id_to_string)
        .unwrap_or_default();

    let org = state
        .db
        .find_organization(&org_id)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to load org")))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Organization not found")))?;

    let role = state
        .db
        .find_role(&role_id)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to load role")))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Role not found")))?;

    let profile = UserProfile {
        id: user_id_str,
        email: user.email,
        full_name: user.full_name.unwrap_or_default(),
        organization: OrgSummary {
            id: record_id_to_string(&org.id),
            name: org.name,
            org_type: org.org_type,
        },
        role: RoleSummary {
            id: record_id_to_string(&role.id),
            name: role.name,
            permissions: role.permissions.unwrap_or_else(|| serde_json::json!({})),
        },
        preferences: serde_json::json!({}),
    };

    Ok(Json(ApiResponse::ok(profile, "Profile loaded")))
}
