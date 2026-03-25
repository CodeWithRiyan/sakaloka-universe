//! Service JWT claims — for planet-to-planet communication.
//!
//! Service tokens are auto-managed by [`crate::tokens::store::ServiceTokenStore`].
//! Each token carries an `aud` claim targeting the specific destination planet.
//! A Jupiter token is rejected by Saturn even if the signature is valid.

use crate::error::SecureError;
use crate::jwt::JwtKeys;
use crate::newtypes::{Planet, TokenId};
use jsonwebtoken::{Algorithm, Header, Validation};
use sakaloka_core::constants::{JWT_ISSUER, SERVICE_TOKEN_TTL_SECS};
use serde::{Deserialize, Serialize};

/// The payload of a Service JWT (planet-to-planet).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceClaims {
    /// Subject: the calling planet's service identity (e.g. `service:earth`).
    pub sub: String,
    /// Issuer: always `sakaloka:iam`.
    pub iss: String,
    /// Audience: the target planet (e.g. `["sakaloka:jupiter"]`).
    pub aud: Vec<String>,
    /// Expiry timestamp (Unix seconds).
    pub exp: u64,
    /// Issued-at timestamp (Unix seconds).
    pub iat: u64,
    /// Unique token ID (for JTI blocklist).
    pub jti: String,
    /// The scopes granted to this service token.
    pub service_scopes: Vec<String>,
    /// The issuing planet name.
    pub planet: String,
}

/// Issues a new Service JWT targeting a specific planet.
///
/// # Errors
///
/// Returns [`SecureError::JwtEncode`] if signing fails.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::jwt::{JwtKeys, service_claims::issue_service_token};
/// use sakaloka_secure::newtypes::Planet;
///
/// std::env::set_var("SAKALOKA_JWT_SECRET", "test-secret-at-least-32-chars-long!");
/// let keys = JwtKeys::from_env().unwrap();
/// let issuer = Planet::new("earth").unwrap();
/// let target = Planet::new("jupiter").unwrap();
/// let token = issue_service_token(&keys, &issuer, &target, &["db:read"]).unwrap();
/// assert!(!token.is_empty());
/// ```
pub fn issue_service_token(
    keys: &JwtKeys,
    issuing_planet: &Planet,
    target_planet: &Planet,
    service_scopes: &[&str],
) -> Result<String, SecureError> {
    let now = current_unix_secs();
    let claims = ServiceClaims {
        sub: format!("service:{}", issuing_planet.as_str()),
        iss: JWT_ISSUER.to_string(),
        aud: vec![target_planet.audience()],
        exp: now + SERVICE_TOKEN_TTL_SECS,
        iat: now,
        jti: TokenId::new().to_string(),
        service_scopes: service_scopes.iter().map(|s| s.to_string()).collect(),
        planet: issuing_planet.to_string(),
    };
    jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, keys.encoding())
        .map_err(|e| SecureError::JwtEncode(e.to_string()))
}

/// Validates a Service JWT and checks that `aud` matches the expected planet.
///
/// # Errors
///
/// Returns [`SecureError::AudienceMismatch`] if the `aud` does not match
/// the expected planet's audience value.
/// Returns [`SecureError::TokenExpired`] if the token has expired.
/// Returns [`SecureError::JwtDecode`] for any other validation failure.
///
/// # Examples
///
/// ```rust
/// use sakaloka_secure::jwt::{JwtKeys, service_claims::{issue_service_token, validate_service_token}};
/// use sakaloka_secure::newtypes::Planet;
///
/// std::env::set_var("SAKALOKA_JWT_SECRET", "test-secret-at-least-32-chars-long!");
/// let keys = JwtKeys::from_env().unwrap();
/// let earth = Planet::new("earth").unwrap();
/// let jupiter = Planet::new("jupiter").unwrap();
/// let token = issue_service_token(&keys, &earth, &jupiter, &["db:read"]).unwrap();
/// let claims = validate_service_token(&keys, &token, &jupiter).unwrap();
/// assert_eq!(claims.planet, "earth");
/// ```
pub fn validate_service_token(
    keys: &JwtKeys,
    token: &str,
    expected_aud: &Planet,
) -> Result<ServiceClaims, SecureError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[expected_aud.audience()]);
    validation.set_issuer(&[JWT_ISSUER]);

    let claims = jsonwebtoken::decode::<ServiceClaims>(token, keys.decoding(), &validation)
        .map(|data| data.claims)
        .map_err(|e| {
            if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                SecureError::TokenExpired
            } else if e.kind() == &jsonwebtoken::errors::ErrorKind::InvalidAudience {
                SecureError::AudienceMismatch {
                    expected: expected_aud.audience(),
                    got: "unknown".to_string(),
                }
            } else {
                SecureError::JwtDecode(e.to_string())
            }
        })?;

    // Extra audience check — ensure the decoded aud matches our expectation.
    let expected = expected_aud.audience();
    if !claims.aud.contains(&expected) {
        return Err(SecureError::AudienceMismatch {
            expected,
            got: claims.aud.join(", "),
        });
    }

    Ok(claims)
}

use super::current_unix_secs;
