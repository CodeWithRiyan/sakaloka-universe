//! Custom middleware for authentication and claim extraction.

use crate::state::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use sakaloka_secure::jwt::user_claims::validate_user_token;

/// Middleware to extract the `Bearer` token, validate the User JWT,
/// and insert `UserClaims` into the request extensions.
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
                    "error": "missing_token",
                    "message": "Authorization header missing or malformed"
                })),
            )
                .into_response()
        }
    };

    match validate_user_token(&state.keys, token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            next.run(req).await
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "invalid_token",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}
