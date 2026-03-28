use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, Result};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use sakaloka_api::app::{self, AppState};
use sakaloka_data::surreal::SurrealClient;
use sakaloka_secure::{argon2, jwt::JwtKeys, newtypes::Password};
use serde_json::{json, Value};
use tower::util::ServiceExt;
use uuid::Uuid;

static TEST_DB_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
static TEST_TRACING: OnceLock<()> = OnceLock::new();

fn db_lock() -> &'static tokio::sync::Mutex<()> {
    TEST_DB_LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

fn unique(label: &str) -> String {
    format!("{label}-{}", Uuid::new_v4().simple())
}

fn load_env_defaults() {
    TEST_TRACING.get_or_init(|| {
        let _ = tracing_subscriber::fmt::try_init();
    });

    dotenvy::dotenv().ok();

    if std::env::var("SAKALOKA_JWT_SECRET").is_err() {
        std::env::set_var("SAKALOKA_JWT_SECRET", "test-secret-at-least-32-chars-long!");
    }
    if std::env::var("SURREALDB_URL").is_err() {
        std::env::set_var("SURREALDB_URL", "ws://127.0.0.1:58000");
    }
    if std::env::var("SURREALDB_USER").is_err() {
        std::env::set_var("SURREALDB_USER", "root");
    }
    if std::env::var("SURREALDB_PASS").is_err() {
        std::env::set_var("SURREALDB_PASS", "sakaloka-dev-password");
    }
}

fn surreal_password_candidates() -> Vec<String> {
    let mut candidates = Vec::new();

    for password in [
        std::env::var("SURREALDB_PASS").ok(),
        Some("sakaloka-dev-password".to_string()),
        Some("sakaloka-dev".to_string()),
        Some("root".to_string()),
    ]
    .into_iter()
    .flatten()
    {
        if !candidates.iter().any(|existing| existing == &password) {
            candidates.push(password);
        }
    }

    candidates
}

async fn build_state() -> Result<AppState> {
    load_env_defaults();

    let db_url = std::env::var("SURREALDB_URL")?;
    let db_user = std::env::var("SURREALDB_USER")?;
    let db_passwords = surreal_password_candidates();

    let db = SurrealClient::connect(&db_url)
        .await
        .map_err(|error| anyhow!("failed to connect surrealdb for tests: {error}"))?;

    let mut sign_in_error = None;
    let mut signed_in = false;
    for password in &db_passwords {
        match db.signin(&db_user, password).await {
            Ok(()) => {
                signed_in = true;
                break;
            }
            Err(error) => {
                sign_in_error = Some(error);
            }
        }
    }

    if !signed_in {
        let last_error = match sign_in_error {
            Some(error) => error.to_string(),
            None => "no credentials attempted".to_string(),
        };
        return Err(anyhow!(
            "failed to sign in surrealdb for tests with user `{db_user}` at `{db_url}`; tried passwords {:?}; last error: {last_error}. If your local container predates the repo credential standardization, restart it with `make infra-down && make infra-up`.",
            db_passwords
        ));
    }

    app::run_migrations(&db).await?;

    Ok(AppState {
        db,
        jwt_keys: Arc::new(JwtKeys::from_env()?),
    })
}

async fn json_request(
    app: &axum::Router,
    method: &str,
    uri: &str,
    body: Value,
    bearer_token: Option<&str>,
) -> Result<(StatusCode, Value)> {
    let body = Body::from(body.to_string());
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");

    if let Some(token) = bearer_token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }

    let request = builder.body(body)?;
    let response = app
        .clone()
        .oneshot(request)
        .await
        .map_err(|error| anyhow!("router request failed: {error:?}"))?;
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .map_err(|error| anyhow!("failed to read response body: {error}"))?;
    let json = serde_json::from_slice(&bytes).map_err(|error| {
        let raw_body = String::from_utf8_lossy(&bytes);
        anyhow!("failed to decode response JSON ({status}): {error}; raw body: {raw_body}")
    })?;

    Ok((status, json))
}

async fn register_user(app: &axum::Router) -> Result<Value> {
    let payload = json!({
        "email": format!("{}@example.test", unique("register")),
        "password": "Sakaloka123!",
        "fullName": "Register Test",
        "organizationName": unique("org"),
    });

    let (status, json) = json_request(app, "POST", "/api/auth/register", payload, None).await?;
    if status != StatusCode::OK && status != StatusCode::CREATED {
        return Err(anyhow!("register expected 200/201, got {status}: {json}"));
    }

    Ok(json)
}

async fn seed_limited_user(state: &AppState) -> Result<(String, String)> {
    let org = state
        .db
        .create_organization(&unique("limited-org"), "company", None)
        .await
        .map_err(|error| anyhow!("failed to create organization for test user: {error}"))?;
    let org_id = sakaloka_api::views::record_id_to_string(&org.id);

    let role = state
        .db
        .create_role(
            &unique("limited-role"),
            &org_id,
            &json!({ "product": ["read"] }),
            false,
        )
        .await
        .map_err(|error| anyhow!("failed to create limited role: {error}"))?;
    let role_id = sakaloka_api::views::record_id_to_string(&role.id);

    let password = "Sakaloka123!";
    let password_hash = argon2::hash_password(&Password::new(password)?)
        .map_err(|error| anyhow!("failed to hash test password: {error}"))?;
    let email = format!("{}@example.test", unique("limited-user"));

    state
        .db
        .create_user(&email, "Limited User", &password_hash, &org_id, &role_id)
        .await
        .map_err(|error| anyhow!("failed to create limited test user: {error}"))?;

    Ok((email, password.to_string()))
}

#[tokio::test]
async fn register_uses_persisted_role_permissions_and_reaches_guarded_route() -> Result<()> {
    let _guard = db_lock().lock().await;
    let state = build_state().await?;
    let app = app::router(state.clone());

    let register_json = register_user(&app).await?;
    let access_token = register_json["data"]["access_token"]
        .as_str()
        .ok_or_else(|| anyhow!("register response did not include access token"))?;
    let role_id = register_json["data"]["user"]["role"]["id"]
        .as_str()
        .ok_or_else(|| anyhow!("register response did not include role id"))?;
    let response_permissions = register_json["data"]["user"]["role"]["permissions"].clone();

    let persisted_role = state
        .db
        .find_role(role_id)
        .await
        .map_err(|error| anyhow!("failed to reload role from db: {error}"))?
        .ok_or_else(|| anyhow!("persisted role not found after registration"))?;
    let persisted_permissions = persisted_role
        .permissions
        .ok_or_else(|| anyhow!("persisted role has no permissions"))?;

    assert_eq!(response_permissions, persisted_permissions);

    let (status, json) = json_request(
        &app,
        "GET",
        "/api/roles/permissions",
        Value::Null,
        Some(access_token),
    )
    .await?;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["success"], Value::Bool(true));
    Ok(())
}

#[tokio::test]
async fn limited_role_login_is_blocked_by_scope_guard() -> Result<()> {
    let _guard = db_lock().lock().await;
    let state = build_state().await?;
    let app = app::router(state.clone());
    let (email, password) = seed_limited_user(&state).await?;

    let (login_status, login_json) = json_request(
        &app,
        "POST",
        "/api/auth/login",
        json!({
            "email": email,
            "password": password,
        }),
        None,
    )
    .await?;

    assert_eq!(login_status, StatusCode::OK);
    assert_eq!(login_json["success"], Value::Bool(true));

    let access_token = login_json["data"]["access_token"]
        .as_str()
        .ok_or_else(|| anyhow!("login response did not include access token"))?;

    let (status, json) = json_request(
        &app,
        "GET",
        "/api/roles/permissions",
        Value::Null,
        Some(access_token),
    )
    .await?;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(json["error"], Value::String("forbidden".to_string()));
    assert_eq!(
        json["message"],
        Value::String("Missing required scope: role:read".to_string())
    );

    Ok(())
}
