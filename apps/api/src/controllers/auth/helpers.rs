//! Shared helpers local to the auth controller.

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::views::{
    auth::{OrgSummary, RoleSummary, UserProfile},
    record_id_to_string,
};

/// Map a DB role name to the RBAC `Role` enum.
pub fn map_rbac_role(role_name: &str) -> sakaloka_secure::rbac::Role {
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
pub async fn issue_tokens_and_session(
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
        .map_err(|e| db_err(e, "Failed to persist session"))?;

    Ok((access_token, refresh_token))
}

/// Build a [`UserProfile`] view from core models.
pub fn build_user_profile(
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
