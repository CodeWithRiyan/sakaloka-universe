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
///     "/entity",
///     post(create_handler).layer(RequireScope::new(Scope::EntityWrite)),
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
    /// let guard = RequireScope::new(Scope::EntityWrite);
    /// assert_eq!(guard.required_scope, Scope::EntityWrite);
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
