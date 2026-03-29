//! Scope enforcement guard for Axum routes.
//!
//! `RequireScope` is an Axum middleware layer that extracts validated
//! `UserClaims` from the request extensions and rejects requests missing
//! the required scope with a `403 Forbidden` response.
//!
//! Usage: applied at the **router level**, never inside handler bodies.

use crate::rbac::Scope;
use axum::{
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Axum middleware that enforces a specific [`Scope`] on a route.
///
/// Apply at router level:
/// ```rust,ignore
/// router.route(
///     "/products",
///     post(create_handler).layer(RequireScope::new(Scope::ProductCreate)),
/// );
/// ```
#[derive(Clone)]
pub struct RequireScope {
    /// The scope that must be present in the user's JWT claims.
    pub required_scope: Scope,
}

impl RequireScope {
    /// Creates a new [`RequireScope`] middleware requiring the given scope.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sakaloka_secure::rbac::{guard::RequireScope, Scope};
    ///
    /// let guard = RequireScope::new(Scope::ProductCreate);
    /// assert_eq!(guard.required_scope, Scope::ProductCreate);
    /// ```
    pub fn new(scope: Scope) -> Self {
        Self {
            required_scope: scope,
        }
    }
}

impl<S> Layer<S> for RequireScope {
    type Service = RequireScopeService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequireScopeService {
            inner,
            required_scope: self.required_scope.clone(),
        }
    }
}

/// The actual tower service running the `RequireScope` logic.
#[derive(Clone)]
pub struct RequireScopeService<S> {
    inner: S,
    required_scope: Scope,
}

impl<S, ReqBody> Service<Request<ReqBody>> for RequireScopeService<S>
where
    S: Service<Request<ReqBody>, Response = Response> + Send + 'static + Clone,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let required_scope = self.required_scope.clone();
        // Cloning the inner service protects against concurrency issues during resolution.
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let claims = match req
                .extensions()
                .get::<crate::jwt::user_claims::UserClaims>()
            {
                Some(c) => c,
                None => {
                    return Ok((
                        StatusCode::FORBIDDEN,
                        Json(serde_json::json!({
                            "error": "forbidden",
                            "message": "Missing JWT claims in request extension"
                        })),
                    )
                        .into_response())
                }
            };

            let scope_str = required_scope.to_string();
            if !claims.scopes.contains(&scope_str) {
                return Ok((
                    StatusCode::FORBIDDEN,
                    Json(serde_json::json!({
                        "error": "forbidden",
                        "message": format!("Missing required scope: {}", scope_str)
                    })),
                )
                    .into_response());
            }

            inner.call(req).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RequireScope;
    use crate::jwt::user_claims::UserClaims;
    use crate::rbac::Scope;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::util::ServiceExt;

    fn claims_with_scopes(scopes: &[&str]) -> UserClaims {
        UserClaims {
            sub: "user:test".to_string(),
            iss: "sakaloka:iam".to_string(),
            aud: vec!["sakaloka:earth".to_string()],
            exp: u64::MAX,
            iat: 1,
            jti: "token:test".to_string(),
            role: "tester".to_string(),
            scopes: scopes.iter().map(|scope| (*scope).to_string()).collect(),
            session_id: "session:test".to_string(),
            org_id: Some("organization:test".to_string()),
        }
    }

    fn scoped_router(required_scope: Scope) -> Router {
        Router::new().route(
            "/guarded",
            get(|| async { "ok" }).layer(RequireScope::new(required_scope)),
        )
    }

    #[tokio::test]
    async fn require_scope_allows_request_with_matching_scope() {
        let app = scoped_router(Scope::ProductRead);
        let mut request = match Request::builder().uri("/guarded").body(Body::empty()) {
            Ok(request) => request,
            Err(error) => panic!("request builder should succeed: {error}"),
        };
        request
            .extensions_mut()
            .insert(claims_with_scopes(&["product:read"]));

        let response = match app.oneshot(request).await {
            Ok(response) => response,
            Err(error) => panic!("router service should return a response: {error:?}"),
        };

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn require_scope_rejects_request_with_missing_scope() {
        let app = scoped_router(Scope::ProductDelete);
        let mut request = match Request::builder().uri("/guarded").body(Body::empty()) {
            Ok(request) => request,
            Err(error) => panic!("request builder should succeed: {error}"),
        };
        request
            .extensions_mut()
            .insert(claims_with_scopes(&["product:read"]));

        let response = match app.oneshot(request).await {
            Ok(response) => response,
            Err(error) => panic!("router service should return a response: {error:?}"),
        };
        let body = match to_bytes(response.into_body(), usize::MAX).await {
            Ok(body) => body,
            Err(error) => panic!("response body should be readable: {error}"),
        };
        let json: serde_json::Value = match serde_json::from_slice(&body) {
            Ok(json) => json,
            Err(error) => panic!("response body should be valid JSON: {error}"),
        };

        assert_eq!(json["error"], "forbidden");
        assert_eq!(json["message"], "Missing required scope: product:delete");
    }

    #[tokio::test]
    async fn require_scope_rejects_request_without_claims() {
        let app = scoped_router(Scope::ProductRead);
        let request = match Request::builder().uri("/guarded").body(Body::empty()) {
            Ok(request) => request,
            Err(error) => panic!("request builder should succeed: {error}"),
        };

        let response = match app.oneshot(request).await {
            Ok(response) => response,
            Err(error) => panic!("router service should return a response: {error:?}"),
        };
        let status = response.status();
        let body = match to_bytes(response.into_body(), usize::MAX).await {
            Ok(body) => body,
            Err(error) => panic!("response body should be readable: {error}"),
        };
        let json: serde_json::Value = match serde_json::from_slice(&body) {
            Ok(json) => json,
            Err(error) => panic!("response body should be valid JSON: {error}"),
        };

        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(json["error"], "forbidden");
        assert_eq!(json["message"], "Missing JWT claims in request extension");
    }
}
