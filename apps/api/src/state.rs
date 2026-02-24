//! Shared application state for Earth API.

use sakaloka_data::surreal::SurrealClient;
use sakaloka_data::zenoh::ZenohClient;
use sakaloka_secure::jwt::JwtKeys;
use std::sync::Arc;

/// The `AppState` contains globally shared resources like DB clients and keys.
#[derive(Clone)]
pub struct AppState {
    /// The authenticated SurrealDB client.
    #[allow(dead_code)]
    pub db: Option<SurrealClient>,
    /// The authenticated Zenoh client.
    #[allow(dead_code)]
    pub zenoh: Option<ZenohClient>,
    /// The loaded JWT encoding/decoding keys.
    pub keys: Arc<JwtKeys>,
}

#[cfg(test)]
impl AppState {
    /// Creates a mock state for integration testing. Does not connect to the database.
    #[allow(clippy::unwrap_used)]
    pub async fn new_for_test() -> Self {
        std::env::set_var(
            "SAKALOKA_JWT_SECRET",
            "super-secret-test-key-must-be-long-enough",
        );
        let keys = Arc::new(JwtKeys::from_env().unwrap());
        Self {
            db: None,
            zenoh: None,
            keys,
        }
    }
}
