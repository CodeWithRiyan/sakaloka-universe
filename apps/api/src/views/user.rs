//! User view DTOs.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::_entities::users;

/// Full user response DTO (password hash is never serialized).
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    /// User ID.
    pub id: Uuid,
    /// Email address.
    pub email: String,
    /// Full name.
    pub full_name: String,
    /// Organization ID.
    pub organization_id: Uuid,
    /// Role ID.
    pub role_id: Uuid,
    /// Whether the user is active.
    pub is_active: bool,
    /// Last login timestamp.
    pub last_login_at: Option<DateTime<FixedOffset>>,
    /// Record creation timestamp.
    pub created_at: DateTime<FixedOffset>,
    /// Record last-update timestamp.
    pub updated_at: DateTime<FixedOffset>,
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
    pub role_id: Uuid,
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
    pub role_id: Option<Uuid>,
    /// Updated active flag.
    pub is_active: Option<bool>,
}

impl UserResponse {
    /// Convert a SeaORM user model into a response DTO.
    /// The password hash is intentionally excluded.
    pub fn from_model(model: users::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            full_name: model.full_name,
            organization_id: model.organization_id,
            role_id: model.role_id,
            is_active: model.is_active,
            last_login_at: model.last_login_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
