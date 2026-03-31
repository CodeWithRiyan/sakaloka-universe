//! Handler functions for auth endpoints.

use axum::{
    extract::{Extension, State},
    Json,
};

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::views::{
    auth::{
        LoginRequest, LoginResponse, OrgSummary, RefreshRequest, RegisterRequest, RoleSummary,
        UserProfile,
    },
    ApiResponse,
};

use super::helpers::{build_user_profile, issue_tokens_and_session};

/// `POST /api/auth/login` — authenticate a user and return tokens.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    // 1. Find user by email
    let user = state
        .db
        .find_user_by_email(&payload.email)
        .await
        .map_err(|e| db_err(e, "DB error during login lookup"))?;

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

    let valid = match sakaloka_secure::argon2::verify_password(&pw, &user.password_hash).await {
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
    let user_id_str = user.id.clone();

    let org_id = match user.organization_id.as_ref() {
        Some(id) => id.clone(),
        None => {
            return Ok(Json(ApiResponse::error(
                "incomplete_profile",
                "User account has no organization assigned. Contact an administrator.",
            )))
        }
    };
    let role_id = match user.role_id.as_ref() {
        Some(id) => id.clone(),
        None => {
            return Ok(Json(ApiResponse::error(
                "incomplete_profile",
                "User account has no role assigned. Contact an administrator.",
            )))
        }
    };

    let (org_result, role_result) = tokio::join!(
        state.db.find_organization(&org_id),
        state.db.find_role(&role_id),
    );
    let org = org_result
        .map_err(|e| db_err(e, "Failed to load organization"))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Organization not found")))?;
    let role = role_result
        .map_err(|e| db_err(e, "Failed to load role"))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Role not found")))?;

    // 4. Issue tokens + persist session — scopes are derived from the DB role's
    //    permissions object, not the static RBAC matrix.
    let role_permissions = match role.permissions.clone() {
        Some(perms) => perms,
        None => {
            return Ok(Json(ApiResponse::error(
                "incomplete_profile",
                "User role has no permissions assigned. Contact an administrator.",
            )))
        }
    };
    let (access_token, refresh_token) = issue_tokens_and_session(
        &state,
        &user_id_str,
        &role.name,
        &role_permissions,
        Some(&org_id),
    )
    .await?;

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
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    // 1. Check if email already exists
    let existing = state
        .db
        .find_user_by_email(&payload.email)
        .await
        .map_err(|e| db_err(e, "DB error checking email"))?;

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
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to hash password")))?;

    // 3. Create account entities (org → role → user → set owner)
    let (org_id_str, role, user_id_str) =
        create_account_entities(&state, &payload, &password_hash).await?;
    let role_id_str = role.id.clone();

    // 4. Issue tokens + persist session — use the persisted admin permissions that were
    //    persisted on the newly-created DB role so the token always reflects
    //    the actual source of truth.
    let role_permissions = role.permissions.clone().ok_or_else(|| {
        ApiError::Internal(anyhow::anyhow!(
            "Created admin role is missing persisted permissions"
        ))
    })?;
    let (access_token, refresh_token) = issue_tokens_and_session(
        &state,
        &user_id_str,
        &role.name,
        &role_permissions,
        Some(&org_id_str),
    )
    .await?;

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
                name: role.name,
                permissions: role_permissions,
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
) -> Result<(String, sakaloka_core::models::role::Role, String), ApiError> {
    // All admin permissions derived from the business permission vocabulary.
    let admin_permissions = sakaloka_secure::rbac::permission::full_business_permissions();

    // Step 1: Create organization
    let org = state
        .db
        .create_organization(&payload.organization_name, "company", None)
        .await
        .map_err(|e| db_err(e, "Failed to create organization"))?;
    let org_id = org.id.clone();

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
    let role_id = role.id.clone();

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
    let user_id = user.id.clone();

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

    Ok((org_id, role, user_id))
}

/// `POST /api/auth/refresh` — exchange a refresh token for new tokens.
///
/// Returns 501 Not Implemented until the full rotation flow is wired up.
pub async fn refresh(
    State(_state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    if payload.refresh_token.is_empty() {
        return Err(ApiError::BadRequest(
            "Refresh token is required".to_string(),
        ));
    }

    // TODO: Complete refresh token rotation once the RefreshStore trait
    // is implemented against SurrealDB.
    Err(ApiError::NotImplemented)
}

/// `GET /api/auth/profile` — return the authenticated user's profile.
pub async fn profile(
    State(state): State<AppState>,
    Extension(claims): Extension<sakaloka_secure::jwt::user_claims::UserClaims>,
) -> Result<Json<ApiResponse<UserProfile>>, ApiError> {
    let user = state
        .db
        .find_user_by_id(&claims.sub)
        .await
        .map_err(|e| db_err(e, "Failed to find user"))?;

    let user = match user {
        Some(u) => u,
        None => return Ok(Json(ApiResponse::not_found("User"))),
    };

    let user_id_str = user.id.clone();

    let org_id = user
        .organization_id
        .clone()
        .ok_or_else(|| ApiError::BadRequest("User has no organization assigned".to_string()))?;
    let role_id = user
        .role_id
        .clone()
        .ok_or_else(|| ApiError::BadRequest("User has no role assigned".to_string()))?;

    let (org_result, role_result) = tokio::join!(
        state.db.find_organization(&org_id),
        state.db.find_role(&role_id),
    );
    let org = org_result
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to load org")))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Organization not found")))?;
    let role = role_result
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
