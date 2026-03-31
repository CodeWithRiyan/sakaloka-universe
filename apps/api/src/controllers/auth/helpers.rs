//! Shared helpers local to the auth controller.

use crate::app::AppState;
use crate::error::ApiError;
use crate::helpers::error_map::db_err;
use crate::views::auth::{OrgSummary, RoleSummary, UserProfile};

/// Issue an access token, generate a refresh token, and persist the session.
///
/// Scopes are derived from the `role.permissions` JSON stored in the database
/// via [`sakaloka_secure::rbac::permission::validated_scopes`]. The old
/// hardcoded role-to-matrix path has been removed; scopes now faithfully
/// reflect what the DB role actually grants.
///
/// # Errors
///
/// Returns [`ApiError::Internal`] if the user ID is invalid, JWT signing fails,
/// the role permissions are malformed, or the session cannot be persisted to
/// the database.
pub async fn issue_tokens_and_session(
    state: &AppState,
    user_id: &str,
    role_name: &str,
    db_permissions: &serde_json::Value,
    org_id: Option<&str>,
) -> Result<(String, String), ApiError> {
    let uid = sakaloka_secure::newtypes::UserId::new(user_id)
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Invalid user ID")))?;
    let session_id = sakaloka_secure::newtypes::SessionId::new();

    // Fail closed when a DB role carries malformed permissions.
    let scopes =
        sakaloka_secure::rbac::permission::validated_scopes(db_permissions).map_err(|e| {
            tracing::error!(error = %e, role = %role_name, "Role permissions are invalid");
            ApiError::Internal(anyhow::anyhow!("Role permissions are invalid"))
        })?;
    let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();

    let access_token = sakaloka_secure::jwt::user_claims::issue_user_token(
        &state.jwt_keys,
        &uid,
        role_name,
        &scope_refs,
        &session_id,
        org_id,
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
///
/// # Examples
///
/// This function is pure and always succeeds — all fields come from validated DB
/// records so no `Result` is needed.
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
            id: org.id.clone(),
            name: org.name.clone(),
            org_type: org.org_type.clone(),
        },
        role: RoleSummary {
            id: role.id.clone(),
            name: role.name.clone(),
            permissions: role
                .permissions
                .clone()
                .unwrap_or_else(|| serde_json::json!({})),
        },
        preferences: serde_json::json!({}),
    }
}
