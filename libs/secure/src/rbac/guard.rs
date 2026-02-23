//! Scope enforcement guard for Axum routes.
//!
//! `RequireScope` is an Axum middleware layer that extracts validated
//! [`UserClaims`] from the request extensions and rejects requests missing
//! the required scope with a `403 Forbidden` response.
//!
//! Usage: applied at the **router level**, never inside handler bodies.

use crate::rbac::Scope;

/// Axum middleware that enforces a specific [`Scope`] on a route.
///
/// Apply at router level:
/// ```rust,ignore
/// router.route(
///     "/entity",
///     post(create_handler).layer(RequireScope::new(Scope::EntityWrite)),
/// );
/// ```
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
