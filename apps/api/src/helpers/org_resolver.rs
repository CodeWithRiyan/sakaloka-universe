//! Resolve the caller's organization ID from their JWT claims.

use crate::app::AppState;
use crate::error::ApiError;
use crate::views::record_id_to_string;

/// Look up the authenticated user and return their `organization_id` as a
/// string.
///
/// This pattern is repeated in almost every write handler — centralising it
/// here removes ~10 lines of boilerplate per call site.
///
/// # Errors
///
/// * `ApiError::Internal` — if the database lookup fails or the user cannot be
///   found.
/// * `ApiError::BadRequest` — if the user has no organization assigned.
pub async fn resolve_caller_org(state: &AppState, user_id: &str) -> Result<String, ApiError> {
    let caller = state
        .db
        .find_user_by_id(user_id)
        .await
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("Failed to find caller")))?
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("Caller not found")))?;

    caller
        .organization_id
        .as_ref()
        .map(record_id_to_string)
        .ok_or_else(|| ApiError::BadRequest("User has no organization assigned".to_string()))
}
