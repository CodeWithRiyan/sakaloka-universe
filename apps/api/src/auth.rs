//! Authentication endpoints for Venus users.

use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use sakaloka_secure::jwt::user_claims::issue_user_token;
use sakaloka_secure::newtypes::{Password, SessionId, UserId};
use serde::{Deserialize, Serialize};

/// Sets up the nested `/auth` router.
pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(login_handler))
}

/// Request body for `POST /auth/login`.
#[derive(Deserialize)]
pub struct LoginRequest {
    /// The user's account name.
    pub username: String,
    /// The plaintext password.
    pub password: String,
}

/// Response payload on successful login.
#[derive(Serialize)]
pub struct LoginResponse {
    /// The short-lived User JWT to be stored in React state.
    pub access_token: String,
    /// The long-lived refresh token to be stored securely by Tauri.
    pub refresh_token: String,
}

/// A structured error response.
#[derive(Serialize)]
pub struct ErrorResponse {
    /// E.g. "invalid_credentials"
    pub error: String,
    /// Human readable message
    pub message: String,
}

/// Handles `POST /auth/login` from Venus.
///
/// In a real implementation this queries SurrealDB for the user's `$argon2id$` hash
/// and calls `verify_password`. It then issues the User JWT.
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // 1. Convert to newtypes to run basic validation
    if Password::new(&payload.password).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "invalid_format".into(),
                message: "Password does not meet complexity rules".into(),
            }),
        )
            .into_response();
    }

    // 2. Lookup user in SurrealDB using the active connection
    // For Sprint 3 scaffold, we will simulate a failed or successful lookup generically.
    // In Sprint 4 this will physically integrate with the user table data.
    if payload.username == "admin" && payload.password == "Sakaloka123!" {
        let user_id = match UserId::new("user:admin") {
            Ok(id) => id,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "internal_error".into(),
                        message: e.to_string(),
                    }),
                )
                    .into_response()
            }
        };
        let session_id = SessionId::new();

        // 3. Issue the JWT
        let access_token =
            match issue_user_token(&state.keys, &user_id, "admin", &["entity:read"], &session_id) {
                Ok(token) => token,
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "internal_error".into(),
                            message: e.to_string(),
                        }),
                    )
                        .into_response()
                }
            };

        // Simulated refresh token (base64 or random string in reality)
        let refresh_token = format!("rt_{}", session_id);

        (
            StatusCode::OK,
            Json(LoginResponse {
                access_token,
                refresh_token,
            }),
        )
            .into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "invalid_credentials".into(),
                message: "Wrong username or password".into(),
            }),
        )
            .into_response()
    }
}
