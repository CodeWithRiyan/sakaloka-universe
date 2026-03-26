//! Concise database-error mapping helpers.

use crate::error::ApiError;

/// Map a database error into an [`ApiError::Internal`], logging the original
/// error at `ERROR` level with the supplied `context` message.
///
/// # Examples
///
/// ```rust,ignore
/// state.db.list_brands(10, 0, None, "name", false)
///     .await
///     .map_err(|e| db_err(e, "Failed to list brands"))?;
/// ```
pub fn db_err(err: impl std::fmt::Display, context: &str) -> ApiError {
    tracing::error!(error = %err, "{context}");
    ApiError::Internal(anyhow::anyhow!("{context}"))
}
