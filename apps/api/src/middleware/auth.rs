//! JWT authentication middleware for the Sakaloka API.
//!
//! Extracts the `Bearer` token from the `Authorization` header, validates it
//! using `sakaloka_secure`, and injects `UserClaims` into request extensions
//! for downstream handlers and the `RequireScope` guard.

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use loco_rs::app::AppContext;
use sakaloka_secure::jwt::{user_claims::validate_user_token, JwtKeys};
use std::sync::Arc;

/// Middleware to extract the `Bearer` token, validate the User JWT, and insert
/// [`sakaloka_secure::jwt::user_claims::UserClaims`] into the request
/// extensions.
///
/// # Errors
///
/// Returns `401 Unauthorized` if the header is missing, malformed, or if the
/// JWT fails validation.
pub async fn auth_middleware(
    _ctx: axum::extract::State<AppContext>,
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

    // Load JWT keys from environment.
    // In production the keys are initialised once during boot; here we
    // reconstruct from env so the middleware is self-contained.
    let keys = match JwtKeys::from_env() {
        Ok(k) => Arc::new(k),
        Err(e) => {
            tracing::error!(error = %e, "Failed to load JWT keys");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "error": "internal_error",
                    "message": "Server configuration error"
                })),
            )
                .into_response();
        }
    };

    match validate_user_token(&keys, token) {
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
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    }
}
