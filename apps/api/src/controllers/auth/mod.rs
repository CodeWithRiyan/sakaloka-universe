//! Authentication controller — login, register, refresh, and profile.

mod handlers;
pub(crate) mod helpers;
mod routes;

pub use routes::{protected_routes, public_routes};
