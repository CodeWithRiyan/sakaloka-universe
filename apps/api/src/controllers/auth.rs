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

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Map a DB role name to the RBAC `Role` enum.
fn map_rbac_role(role_name: &str) -> sakaloka_secure::rbac::Role {
    match role_name.to_lowercase().as_str() {
        "admin" | "owner" => sakaloka_secure::rbac::Role::Admin,
        "editor" | "manager" | "cashier" => sakaloka_secure::rbac::Role::Editor,
        other => {
            tracing::warn!(role_name = %other, "Unknown role mapped to Viewer — add explicit mapping if this is intentional");
            sakaloka_secure::rbac::Role::Viewer
        }
    }
}

/// Issue an access token, generate a refresh token, and persist the session.
///
/// Returns `(access_token, refresh_token)`.
async fn issue_tokens_and_session(
    state: &AppState,
    user_id: &str,
    role_name: &str,
) -> Result<(String, String), ApiError> {
    let uid = sakaloka_secure::newtypes::UserId::new(user_id)
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Invalid user ID")))?;
    let session_id = sakaloka_secure::newtypes::SessionId::new();

    let rbac_role = map_rbac_role(role_name);
    let scopes: Vec<String> = sakaloka_secure::rbac::matrix::allowed_scopes(&rbac_role)
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();

    let access_token = sakaloka_secure::jwt::user_claims::issue_user_token(
        &state.jwt_keys,
        &uid,
        role_name,
        &scope_refs,
        &session_id,
    )
    .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to issue JWT")))?;

    let refresh_token = sakaloka_secure::newtypes::TokenId::new().to_string();
    let token_hash = sakaloka_secure::tokens::rotation::hash_refresh_token(&refresh_token);

    state
        .db
        .create_session(&uid, &session_id, &token_hash)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to persist session");
            ApiError::Internal(anyhow::anyhow!("Failed to create session"))
        })?;

    Ok((access_token, refresh_token))
}

/// Build a [`UserProfile`] view from core models.
fn build_user_profile(
    user_id: String,
    email: String,
    full_name: String,
    org: &sakaloka_core::models::organization::Organization,
    role: &sakaloka_core::models::role::Role,
) -> UserProfile {
    UserProfile {
        id: user_id,
        email,
        full_name,
        organization: OrgSummary {
            id: record_id_to_string(&org.id),
            name: org.name.clone(),
            org_type: org.org_type.clone(),
        },
        role: RoleSummary {
            id: record_id_to_string(&role.id),
            name: role.name.clone(),
            permissions: role
                .permissions
                .clone()
                .unwrap_or_else(|| serde_json::json!({})),
        },
        preferences: serde_json::json!({}),
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

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

    // 3. Load organization and role
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

    let org = state
        .db
        .find_organization(&org_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to load organization");
            ApiError::Internal(anyhow::anyhow!("Failed to load organization"))
        })?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Organization not found")))?;

    let role = state
        .db
        .find_role(&role_id)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to load role");
            ApiError::Internal(anyhow::anyhow!("Failed to load role"))
        })?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Role not found")))?;

    // 4. Issue tokens + persist session
    let (access_token, refresh_token) =
        issue_tokens_and_session(&state, &user_id_str, &role.name).await?;

    // 5. Update last_login_at (best-effort)
    if let Err(e) = state.db.update_last_login(&user_id_str).await {
        tracing::warn!(error = %e, user_id = %user_id_str, "Failed to update last_login_at");
    }

    let response = LoginResponse {
        access_token,
        refresh_token,
        user: build_user_profile(
            user_id_str,
            user.email,
            user.full_name.unwrap_or_default(),
            &org,
            &role,
        ),
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

    // 2. Validate and hash password
    let pw = match sakaloka_secure::newtypes::Password::new(&payload.password) {
        Ok(p) => p,
        Err(e) => return Ok(Json(ApiResponse::error("validation_error", &e.to_string()))),
    };
    let password_hash = sakaloka_secure::argon2::hash_password(&pw)
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to hash password")))?;

    // 3. Create account entities (org → role → user → set owner)
    //    On failure after org creation, attempt best-effort cleanup.
    let (org_id_str, role_id_str, user_id_str) =
        create_account_entities(&state, &payload, &password_hash).await?;

    // 4. Issue tokens + persist session
    let (access_token, refresh_token) =
        issue_tokens_and_session(&state, &user_id_str, "admin").await?;

    let admin_permissions = serde_json::json!({
        "entity:read": true,
        "entity:write": true,
        "entity:delete": true,
        "user:read": true,
        "user:manage": true,
    });

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

/// Create organization, role, user, and link owner — with best-effort cleanup
/// on partial failure.
async fn create_account_entities(
    state: &AppState,
    payload: &RegisterRequest,
    password_hash: &str,
) -> Result<(String, String, String), ApiError> {
    let admin_permissions = serde_json::json!({
        "entity:read": true,
        "entity:write": true,
        "entity:delete": true,
        "user:read": true,
        "user:manage": true,
    });

    // Step 1: Create organization
    let org = state
        .db
        .create_organization(&payload.organization_name, "company", None)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to create organization");
            ApiError::Internal(anyhow::anyhow!("Failed to create organization"))
        })?;
    let org_id = record_id_to_string(&org.id);

    // Step 2: Create role (cleanup org on failure)
    let role = match state
        .db
        .create_role("admin", &org_id, &admin_permissions, true)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(error = %e, "Failed to create role, cleaning up org");
            let _ = state.db.delete_organization(&org_id).await;
            return Err(ApiError::Internal(anyhow::anyhow!("Failed to create role")));
        }
    };
    let role_id = record_id_to_string(&role.id);

    // Step 3: Create user (cleanup role + org on failure)
    let user = match state
        .db
        .create_user(
            &payload.email,
            &payload.full_name,
            password_hash,
            &org_id,
            &role_id,
        )
        .await
    {
        Ok(u) => u,
        Err(e) => {
            tracing::error!(error = %e, "Failed to create user, cleaning up role and org");
            let _ = state.db.delete_role(&role_id).await;
            let _ = state.db.delete_organization(&org_id).await;
            return Err(ApiError::Internal(anyhow::anyhow!("Failed to create user")));
        }
    };
    let user_id = record_id_to_string(&user.id);

    // Step 4: Link owner (cleanup user + role + org on failure)
    if let Err(e) = state.db.update_organization_owner(&org_id, &user_id).await {
        tracing::error!(error = %e, "Failed to set org owner, cleaning up");
        let _ = state.db.delete_user(&user_id).await;
        let _ = state.db.delete_role(&role_id).await;
        let _ = state.db.delete_organization(&org_id).await;
        return Err(ApiError::Internal(anyhow::anyhow!(
            "Failed to set organization owner"
        )));
    }

    Ok((org_id, role_id, user_id))
}

/// `POST /api/auth/refresh` — exchange a refresh token for new tokens.
///
/// Returns 501 Not Implemented until the full rotation flow is wired up.
async fn refresh(
    State(_state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    if payload.refresh_token.is_empty() {
        return Err(ApiError::BadRequest(
            "Refresh token is required".to_string(),
        ));
    }

    // TODO: Complete refresh token rotation once the RefreshStore trait
    // is implemented against SurrealDB.  The rotation module in
    // sakaloka_secure::tokens::rotation handles reuse detection, token
    // hashing, and session termination.
    Err(ApiError::NotImplemented)
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
        .ok_or_else(|| ApiError::BadRequest("User has no organization assigned".to_string()))?;
    let role_id = user
        .role_id
        .as_ref()
        .map(record_id_to_string)
        .ok_or_else(|| ApiError::BadRequest("User has no role assigned".to_string()))?;

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

    let profile = build_user_profile(
        user_id_str,
        user.email,
        user.full_name.unwrap_or_default(),
        &org,
        &role,
    );

    Ok(Json(ApiResponse::ok(profile, "Profile loaded")))
}
