#![allow(clippy::unwrap_used)]

use sakaloka_secure::error::SecureError;
use sakaloka_secure::jwt::{
    service_claims::{issue_service_token, validate_service_token},
    user_claims::{issue_user_token, validate_user_token},
    JwtKeys,
};
use sakaloka_secure::newtypes::{Planet, SessionId, UserId};

fn setup_keys() -> JwtKeys {
    std::env::set_var(
        "SAKALOKA_JWT_SECRET",
        "super-secret-test-key-must-be-long-enough",
    );
    JwtKeys::from_env().unwrap()
}

#[test]
fn test_user_jwt_issue_and_validate() {
    let keys = setup_keys();
    let user_id = UserId::new("user:123").unwrap();
    let session_id = SessionId::new();
    let role = "viewer";
    let scopes = vec!["product:read"];

    let token = issue_user_token(
        &keys,
        &user_id,
        role,
        &scopes,
        &session_id,
        Some("organization:test"),
    )
    .unwrap();
    let claims = validate_user_token(&keys, &token).unwrap();

    assert_eq!(claims.sub, user_id.as_str());
    assert_eq!(claims.session_id, session_id.to_string());
    assert_eq!(claims.role, "viewer");
}

#[test]
fn test_service_jwt_issue_and_validate() {
    let keys = setup_keys();
    let earth = Planet::new("earth").unwrap();
    let jupiter = Planet::new("jupiter").unwrap();

    let token = issue_service_token(&keys, &earth, &jupiter, &["db:read"]).unwrap();

    // Validating against expected target (jupiter) must succeed
    let claims = validate_service_token(&keys, &token, &jupiter).unwrap();
    assert_eq!(claims.sub, "service:earth");
    assert_eq!(claims.aud, vec!["sakaloka:jupiter".to_string()]);
}

#[test]
fn test_service_jwt_audience_mismatch() {
    let keys = setup_keys();
    let earth = Planet::new("earth").unwrap();
    let jupiter = Planet::new("jupiter").unwrap();
    let saturn = Planet::new("saturn").unwrap();

    // Issuing token explicitly for jupiter
    let token = issue_service_token(&keys, &earth, &jupiter, &[]).unwrap();

    // Validating it against saturn must fail
    let err = validate_service_token(&keys, &token, &saturn).unwrap_err();
    assert!(matches!(err, SecureError::AudienceMismatch { .. }));

    // Validating it as a user token must fail
    let user_err = validate_user_token(&keys, &token).unwrap_err();
    assert!(matches!(user_err, SecureError::JwtDecode(_)));
}
