//! Shared row types and error definitions for the PostgreSQL backend.

/// Errors returned by the PostgreSQL client.
#[derive(Debug, thiserror::Error)]
pub enum PgError {
    /// Connection pool could not be established.
    #[error("connection error: {0}")]
    Connection(String),
    /// A SQL query failed.
    #[error("query error: {0}")]
    Query(String),
    /// A migration failed.
    #[error("migration error: {0}")]
    Migration(String),
    /// A requested record was not found.
    #[error("not found: {0}")]
    NotFound(String),
}

impl From<sqlx::Error> for PgError {
    fn from(e: sqlx::Error) -> Self {
        Self::Query(e.to_string())
    }
}

/// Helper to convert a UUID to its string representation.
pub(crate) fn uuid_to_string(u: uuid::Uuid) -> String {
    u.to_string()
}

/// Helper to parse a string ID to UUID, returning [`PgError`] on failure.
pub(crate) fn parse_uuid(id: &str) -> Result<uuid::Uuid, PgError> {
    uuid::Uuid::parse_str(id).map_err(|e| PgError::Query(format!("invalid UUID '{id}': {e}")))
}

/// Helper to safely parse a sort column from user input.
pub(crate) fn safe_sort_column<'a>(input: &str, allowed: &[&'a str], default: &'a str) -> &'a str {
    allowed
        .iter()
        .find(|&&col| col == input)
        .copied()
        .unwrap_or(default)
}

/// Helper to build sort direction string.
pub(crate) fn sort_dir(desc: bool) -> &'static str {
    if desc {
        "DESC"
    } else {
        "ASC"
    }
}
