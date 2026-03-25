//! Unified API error type for all handlers.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// API error type returned by all handlers.
#[derive(Debug)]
pub enum ApiError {
    /// 500 Internal Server Error.
    Internal(anyhow::Error),
    /// 404 Not Found.
    NotFound(String),
    /// 400 Bad Request.
    BadRequest(String),
    /// 401 Unauthorized.
    Unauthorized(String),
    /// 501 Not Implemented.
    NotImplemented,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Internal(e) => {
                tracing::error!(error = %e, "internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            Self::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string()),
        };

        let body = serde_json::json!({ "error": message });
        (status, axum::Json(body)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err)
    }
}

impl From<sakaloka_data::surreal::SurrealError> for ApiError {
    fn from(err: sakaloka_data::surreal::SurrealError) -> Self {
        Self::Internal(anyhow::anyhow!("{}", err))
    }
}
