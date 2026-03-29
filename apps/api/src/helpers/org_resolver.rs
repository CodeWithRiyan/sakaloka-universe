//! Resolve the caller's organization ID from their JWT claims.

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::record_id_to_string;

/// Return the caller's organization ID, preferring the JWT claim and falling
/// back to a DB lookup for tokens issued before `org_id` was embedded.
///
/// # Errors
///
/// * `ApiError::Internal` — if the database lookup fails or the user cannot be
///   found.
/// * `ApiError::BadRequest` — if the user has no organization assigned.
pub async fn resolve_caller_org(
    state: &AppState,
    claims: &sakaloka_secure::jwt::user_claims::UserClaims,
) -> Result<String, ApiError> {
    // Fast path: org_id is already in the JWT (v2+ tokens).
    if let Some(ref org_id) = claims.org_id {
        return Ok(org_id.clone());
    }

    // Slow path: fall back to DB for legacy tokens without org_id.
    let caller = state
        .db
        .find_user_by_id(&claims.sub)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to find caller")))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Caller not found")))?;

    caller
        .organization_id
        .as_ref()
        .map(record_id_to_string)
        .ok_or_else(|| ApiError::BadRequest("User has no organization assigned".to_string()))
}
