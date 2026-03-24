use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId as Thing};
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

/// Represents a User record from SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema, SurrealValueMacro)]
pub struct User {
    /// The record ID (e.g. `user:01J...`).
    #[schema(value_type = String)]
    pub id: Thing,
    /// The unique username (optional).
    pub username: Option<String>,
    /// The unique email address.
    pub email: String,
    /// Full display name.
    pub full_name: Option<String>,
    /// The Argon2id password hash.
    pub password_hash: String,
    /// Link to the user's organization.
    #[schema(value_type = Option<String>)]
    pub organization_id: Option<Thing>,
    /// Link to the user's role.
    #[schema(value_type = Option<String>)]
    pub role_id: Option<Thing>,
    /// Whether the account is active.
    pub is_active: bool,
    /// Last login timestamp.
    #[schema(value_type = Option<String>)]
    pub last_login_at: Option<Datetime>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: Datetime,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: Datetime,
}
