use serde::{Deserialize, Serialize};

/// Represents a User record.
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct User {
    /// The record ID.
    pub id: String,
    /// The unique username (optional).
    pub username: Option<String>,
    /// The unique email address.
    pub email: String,
    /// Full display name.
    pub full_name: Option<String>,
    /// The Argon2id password hash.
    pub password_hash: String,
    /// Link to the user's organization.
    pub organization_id: Option<String>,
    /// Link to the user's role.
    pub role_id: Option<String>,
    /// Whether the account is active.
    pub is_active: bool,
    /// Last login timestamp.
    #[schema(value_type = Option<String>)]
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Creation timestamp.
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Update timestamp.
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
