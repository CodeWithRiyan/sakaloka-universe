//! Authentication view DTOs.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request body for `POST /api/auth/login`.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// User email address.
    #[schema(example = "admin@sakaloka.id")]
    pub email: String,
    /// Plaintext password.
    #[schema(example = "sakaloka-dev-01")]
    pub password: String,
}

/// Request body for `POST /api/auth/register`.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    /// User email address.
    #[schema(example = "newuser@sakaloka.id")]
    pub email: String,
    /// Plaintext password (must meet complexity rules).
    #[schema(example = "S3cureP@ssw0rd!")]
    pub password: String,
    /// Full name of the user.
    #[schema(example = "Budi Santoso")]
    pub full_name: String,
    /// Name of the organization to create or join.
    #[schema(example = "Warung Budi")]
    pub organization_name: String,
}

/// Request body for `POST /api/auth/refresh`.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    /// The refresh token issued during login or a previous refresh.
    pub refresh_token: String,
}

/// Successful login / refresh response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    /// Short-lived access JWT.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Long-lived refresh token.
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    /// Authenticated user profile.
    pub user: UserProfile,
}

/// User profile embedded in the login response.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    /// User primary key.
    pub id: String,
    /// Email address.
    pub email: String,
    /// Full name.
    pub full_name: String,
    /// Summary of the user's organization.
    pub organization: OrgSummary,
    /// Summary of the user's role.
    pub role: RoleSummary,
    /// Arbitrary user preferences (JSON).
    pub preferences: serde_json::Value,
}

/// Abbreviated organization data included in auth responses.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrgSummary {
    /// Organization ID.
    pub id: String,
    /// Organization name.
    pub name: String,
    /// Organization type.
    #[serde(rename = "type")]
    pub org_type: String,
}

/// Abbreviated role data included in auth responses.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleSummary {
    /// Role ID.
    pub id: String,
    /// Role name.
    pub name: String,
    /// List of permission strings.
    pub permissions: serde_json::Value,
}
