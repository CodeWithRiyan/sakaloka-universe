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
}
