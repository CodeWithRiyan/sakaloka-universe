//! JWT authentication middleware for the Sakaloka API.
//!
//! Extracts the `Bearer` token from the `Authorization` header, validates it
//! using `sakaloka_secure`, and injects `UserClaims` into request extensions
//! for downstream handlers and the `RequireScope` guard.

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
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
    let token = match bearer_token(req.headers()) {
        Some(token) => token,
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

/// Extracts the bearer token from an `Authorization` header map.
fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
}

#[cfg(test)]
mod tests {
    use super::bearer_token;
    use axum::http::{HeaderMap, HeaderValue};

    #[test]
    fn bearer_token_extracts_valid_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Bearer sakaloka-token"),
        );

        assert_eq!(bearer_token(&headers), Some("sakaloka-token"));
    }

    #[test]
    fn bearer_token_rejects_missing_header() {
        let headers = HeaderMap::new();

        assert_eq!(bearer_token(&headers), None);
    }

    #[test]
    fn bearer_token_rejects_non_bearer_scheme() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("Basic abc123"));

        assert_eq!(bearer_token(&headers), None);
    }
}
