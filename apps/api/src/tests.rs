/// Integration tests for the Sakaloka Earth API.
///
/// Per `.cursorrules`: `.unwrap()` IS allowed inside `#[test]` blocks.
/// We suppress the global deny at module level for test code only.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::module_inception)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`

    use crate::router::build_router;

    /// US-03 — Verifies GET /health returns 200 OK with correct JSON payload.
    ///
    /// Tests the handler directly via Axum's `oneshot` — no real TCP socket required.
    #[tokio::test]
    async fn health_returns_200_with_json() {
        let state = crate::state::AppState::new_for_test().await;
        let app = build_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body["status"], "ok");
        assert_eq!(body["planet"], "earth");
        assert_eq!(body["service"], "sakaloka-api");
    }

    #[tokio::test]
    async fn rbac_enforcement_test() {
        use sakaloka_secure::jwt::user_claims::issue_user_token;
        use sakaloka_secure::newtypes::{SessionId, UserId};

        let state = crate::state::AppState::new_for_test().await;
        let app = build_router(state.clone());

        // 1. NO TOKEN -> 401
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/entity")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        // 2. VALID TOKEN, WRONG SCOPE -> 403
        let user_id = UserId::new("user:01JTEST").unwrap();
        let session = SessionId::new();
        let token_no_scope = issue_user_token(
            &state.keys,
            &user_id,
            "viewer",
            &["search:read"], // has search but not entity:read
            &session,
        )
        .unwrap();

        let app = build_router(state.clone());
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/entity")
                    .method("GET")
                    .header("Authorization", format!("Bearer {}", token_no_scope))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::FORBIDDEN);

        // 3. VALID TOKEN, CORRECT SCOPE -> 200
        let token_with_scope =
            issue_user_token(&state.keys, &user_id, "viewer", &["entity:read"], &session).unwrap();

        let app = build_router(state.clone());
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/entity")
                    .method("GET")
                    .header("Authorization", format!("Bearer {}", token_with_scope))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // 4. POST REQUIRES entity:write -> 403 if only read
        let app = build_router(state.clone());
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/entity")
                    .method("POST")
                    .header("Authorization", format!("Bearer {}", token_with_scope))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
    }
}
