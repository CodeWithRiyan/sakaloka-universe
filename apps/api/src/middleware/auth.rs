//! JWT authentication middleware for the Sakaloka API.
//!
//! Extracts the `Bearer` token from the `Authorization` header, validates it
//! using `sakaloka_secure`, and injects `UserClaims` into request extensions
//! for downstream handlers and the `RequireScope` guard.

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use sakaloka_secure::jwt::user_claims::validate_user_token;

use crate::app::AppState;

/// Middleware to extract the `Bearer` token, validate the User JWT, and insert
/// [`sakaloka_secure::jwt::user_claims::UserClaims`] into the request
/// extensions.
///
/// # Errors
///
/// Returns `401 Unauthorized` if the header is missing, malformed, or if the
/// JWT fails validation.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "error": "missing_token",
                    "message": "Authorization header missing or malformed"
                })),
            )
                .into_response()
        }
    };

    match validate_user_token(&state.jwt_keys, token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            next.run(req).await
        }
        Err(e) => {
            tracing::warn!(error = %e, "JWT validation failed");
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "error": "invalid_token",
                    "message": "Token is invalid or expired"
                })),
            )
                .into_response()
        }
    }
}
