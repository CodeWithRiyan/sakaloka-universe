//! Authentication endpoints for Venus users.

use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use sakaloka_secure::jwt::user_claims::issue_user_token;
use sakaloka_secure::newtypes::{Password, SessionId, UserId};
use serde::{Deserialize, Serialize};

/// Sets up the nested `/auth` router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
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
    let db = match &state.db {
        Some(db) => db,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal_error".into(),
                    message: "Database connection not initialized".into(),
                }),
            )
                .into_response();
        }
    };

    let user = match db.find_user_by_username(&payload.username).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "invalid_credentials".into(),
                    message: "Wrong username or password".into(),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal_error".into(),
                    message: e.to_string(),
                }),
            )
                .into_response();
        }
    };

    // 3. Verify password hash
    let pw = match Password::new(&payload.password) {
        Ok(p) => p,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "invalid_format".into(),
                    message: "Invalid password format".into(),
                }),
            )
                .into_response();
        }
    };
    let matches = sakaloka_secure::argon2::verify_password(&pw, &user.password_hash);

    if !matches.unwrap_or(false) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "invalid_credentials".into(),
                message: "Wrong username or password".into(),
            }),
        )
            .into_response();
    }

    // 4. Issue the JWT
    let user_id = match UserId::new(&user.id.to_string()) {
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

    let role = match user.role.to_lowercase().as_str() {
        "admin" => sakaloka_secure::rbac::Role::Admin,
        "editor" => sakaloka_secure::rbac::Role::Editor,
        _ => sakaloka_secure::rbac::Role::Viewer,
    };

    let scopes: Vec<String> = sakaloka_secure::rbac::matrix::allowed_scopes(&role)
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let session_id = SessionId::new();

    let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();

    let access_token =
        match issue_user_token(&state.keys, &user_id, &user.role, &scope_refs, &session_id) {
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

    // 5. Generate high-entropy refresh token and hash it
    let refresh_token = sakaloka_secure::newtypes::TokenId::new().to_string();
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(refresh_token.as_bytes());
    let refresh_hash = format!("{:x}", hasher.finalize());

    // 6. Persist session and initial refresh token
    if let Err(e) = db
        .create_session(&user_id, &session_id, &refresh_hash)
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "internal_error".into(),
                message: format!("Failed to create session: {}", e),
            }),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(LoginResponse {
            access_token,
            refresh_token,
        }),
    )
        .into_response()
}

/// Request body for `POST /auth/refresh`.
#[derive(Deserialize)]
pub struct RefreshRequest {
    /// The refresh token issued during login or previous refresh.
    pub refresh_token: String,
}

/// Handles `POST /auth/refresh` from Venus.
///
/// Validates the refresh token, checks for reuse (terminating session if found),
/// rotates the token, and issues a new User JWT.
pub async fn refresh_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> impl IntoResponse {
    let db = match &state.db {
        Some(db) => db,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // 1. Rotate the refresh token (logic in libs/secure enforces reuse detection)
    let (new_refresh_token, record) =
        match sakaloka_secure::tokens::rotation::rotate_refresh_token(db, &payload.refresh_token)
            .await
        {
            Ok(res) => res,
            Err(sakaloka_secure::error::SecureError::TokenReused) => {
                return (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "token_reused".into(),
                    message:
                        "Security violation: refresh token already used. All sessions terminated."
                            .into(),
                }),
            )
                .into_response();
            }
            Err(e) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "invalid_token".into(),
                        message: e.to_string(),
                    }),
                )
                    .into_response();
            }
        };

    // 2. Find associated user via session_id
    let user = match db.find_user_by_session(&record.session_id).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "session_invalid".into(),
                    message: "Associated user not found".into(),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal_error".into(),
                    message: e.to_string(),
                }),
            )
                .into_response();
        }
    };

    // 3. Issue new User JWT
    let user_id = match UserId::new(&user.id.to_string()) {
        Ok(id) => id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let role = match user.role.to_lowercase().as_str() {
        "admin" => sakaloka_secure::rbac::Role::Admin,
        "editor" => sakaloka_secure::rbac::Role::Editor,
        _ => sakaloka_secure::rbac::Role::Viewer,
    };

    let scopes: Vec<String> = sakaloka_secure::rbac::matrix::allowed_scopes(&role)
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();

    let access_token = match issue_user_token(
        &state.keys,
        &user_id,
        &user.role,
        &scope_refs,
        &record.session_id,
    ) {
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

    (
        StatusCode::OK,
        Json(LoginResponse {
            access_token,
            refresh_token: new_refresh_token,
        }),
    )
        .into_response()
}
