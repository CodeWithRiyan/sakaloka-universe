//! User view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::record_id_to_string;

/// Full user response DTO (password hash is never serialized).
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    /// User ID.
    pub id: String,
    /// Email address.
    pub email: String,
    /// Full name.
    pub full_name: Option<String>,
    /// Organization ID.
    pub organization_id: Option<String>,
    /// Role ID.
    pub role_id: Option<String>,
    /// Whether the user is active.
    pub is_active: bool,
    /// Last login timestamp.
    pub last_login_at: Option<String>,
    /// Record creation timestamp.
    pub created_at: String,
    /// Record last-update timestamp.
    pub updated_at: String,
}

/// Request body for creating a user.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    /// Email address.
    pub email: String,
    /// Full name.
    pub full_name: String,
    /// Plaintext password (will be hashed before storage).
    pub password: String,
    /// Role ID to assign.
    pub role_id: String,
    /// Whether the user should be active immediately.
    pub is_active: Option<bool>,
}

/// Request body for updating a user (all fields optional).
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    /// Updated email.
    pub email: Option<String>,
    /// Updated full name.
    pub full_name: Option<String>,
    /// Updated plaintext password.
    pub password: Option<String>,
    /// Updated role ID.
    pub role_id: Option<String>,
    /// Updated active flag.
    pub is_active: Option<bool>,
}

impl UserResponse {
    /// Convert a domain [`User`](sakaloka_core::models::user::User) model
    /// into a response DTO. The password hash is intentionally excluded.
    pub fn from_model(model: &sakaloka_core::models::user::User) -> Self {
        Self {
            id: record_id_to_string(&model.id),
            email: model.email.clone(),
            full_name: model.full_name.clone(),
            organization_id: model.organization_id.as_ref().map(record_id_to_string),
            role_id: model.role_id.as_ref().map(record_id_to_string),
            is_active: model.is_active,
            last_login_at: model.last_login_at.as_ref().map(|dt| dt.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
