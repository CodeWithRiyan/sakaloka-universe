//! Authentication controller — login, register, refresh, and profile.

pub(crate) mod handlers;
pub(crate) mod helpers;
mod routes;

pub use routes::{protected_routes, public_routes};
