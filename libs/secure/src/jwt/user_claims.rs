//! User JWT claims — issued by Earth, held by Venus.

use crate::error::SecureError;
use crate::jwt::JwtKeys;
use crate::newtypes::{SessionId, TokenId, UserId};
use jsonwebtoken::{Algorithm, Header, Validation};
use sakaloka_core::constants::{AUD_EARTH, JWT_ISSUER, USER_TOKEN_TTL_SECS};
use serde::{Deserialize, Serialize};

/// The payload of a User JWT.
///
/// Issued by Earth after a successful login. Held in React state (memory only)
/// on Venus. Validated on every request to Earth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    /// Subject: the user's record ID (e.g. `user:01Jxxx`).
    pub sub: String,
    /// Issuer: always `sakaloka:iam`.
    pub iss: String,
    /// Audience: always `["sakaloka:earth"]`.
    pub aud: Vec<String>,
    /// Expiry timestamp (Unix seconds).
    pub exp: u64,
    /// Issued-at timestamp (Unix seconds).
    pub iat: u64,
    /// Unique token ID (for JTI blocklist).
    pub jti: String,
    /// The user's role: `admin`, `editor`, or `viewer`.
    pub role: String,
    /// The scopes granted to this user.
    pub scopes: Vec<String>,
    /// The associated session ID.
    pub session_id: String,
}

/// Issues a new User JWT signed with the workspace HS256 secret.
///
/// # Errors
///
/// Returns [`SecureError::JwtEncode`] if signing fails.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::jwt::{JwtKeys, user_claims::issue_user_token};
/// use sakaloka_secure::newtypes::{UserId, SessionId};
///
/// std::env::set_var("SAKALOKA_JWT_SECRET", "test-secret-at-least-32-chars-long!");
/// let keys = JwtKeys::from_env().unwrap();
/// let user_id = UserId::new("user:01JKXYZ").unwrap();
/// let session = SessionId::new();
/// let token = issue_user_token(&keys, &user_id, "editor", &["product:read"], &session).unwrap();
/// assert!(!token.is_empty());
/// ```
pub fn issue_user_token(
    keys: &JwtKeys,
    user_id: &UserId,
    role: &str,
    scopes: &[&str],
    session_id: &SessionId,
) -> Result<String, SecureError> {
    let now = current_unix_secs();
    let claims = UserClaims {
        sub: user_id.to_string(),
        iss: JWT_ISSUER.to_string(),
        aud: vec![AUD_EARTH.to_string()],
        exp: now + USER_TOKEN_TTL_SECS,
        iat: now,
        jti: TokenId::new().to_string(),
        role: role.to_string(),
        scopes: scopes.iter().map(|s| s.to_string()).collect(),
        session_id: session_id.to_string(),
    };
    jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, keys.encoding())
        .map_err(|e| SecureError::JwtEncode(e.to_string()))
}

/// Validates a User JWT and returns the decoded [`UserClaims`].
///
/// # Errors
///
/// Returns [`SecureError::TokenExpired`] if the token has expired.
/// Returns [`SecureError::JwtDecode`] for any other validation failure.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::jwt::{JwtKeys, user_claims::{issue_user_token, validate_user_token}};
/// use sakaloka_secure::newtypes::{UserId, SessionId};
///
/// std::env::set_var("SAKALOKA_JWT_SECRET", "test-secret-at-least-32-chars-long!");
/// let keys = JwtKeys::from_env().unwrap();
/// let user_id = UserId::new("user:01JKXYZ").unwrap();
/// let session = SessionId::new();
/// let token = issue_user_token(&keys, &user_id, "editor", &["product:read"], &session).unwrap();
/// let claims = validate_user_token(&keys, &token).unwrap();
/// assert_eq!(claims.role, "editor");
/// ```
pub fn validate_user_token(keys: &JwtKeys, token: &str) -> Result<UserClaims, SecureError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[AUD_EARTH]);
    validation.set_issuer(&[JWT_ISSUER]);

    jsonwebtoken::decode::<UserClaims>(token, keys.decoding(), &validation)
        .map(|data| data.claims)
        .map_err(|e| {
            if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                SecureError::TokenExpired
            } else {
                SecureError::JwtDecode(e.to_string())
            }
        })
}

use super::current_unix_secs;
