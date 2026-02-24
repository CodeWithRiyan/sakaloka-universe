use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

/// Represents a User record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// The record ID (e.g. `user:01J...`).
    pub id: Thing,
    /// The unique username.
    pub username: String,
    /// The unique email address.
    pub email_address: String,
    /// The Argon2id password hash.
    pub password_hash: String,
    /// The user's role: admin, editor, or viewer.
    pub role: String,
    /// Creation timestamp.
    pub created_at: Datetime,
    /// Update timestamp.
    pub updated_at: Datetime,
}
