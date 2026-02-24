//! Eclipse Zenoh (Saturn) messaging client abstraction.
//!
//! All Zenoh event handlers MUST be idempotent — the same event delivered
//! twice must produce the same result. Use sequence numbers in payloads
//! to detect duplicates.
//!
//! Topic naming convention:
//! `sakaloka/{source_planet}/{entity}/{event_type}`

use std::sync::Arc;
use thiserror::Error;
use zenoh::Session;

/// Errors produced by the Zenoh client layer.
#[derive(Debug, Error)]
pub enum ZenohError {
    /// Connection to the Zenoh router failed.
    #[error("Zenoh connection failed: {0}")]
    Connection(String),
    /// Publishing a message failed.
    #[error("Zenoh publish error: {0}")]
    Publish(String),
    /// Subscribing to a topic failed.
    #[error("Zenoh subscribe error: {0}")]
    Subscribe(String),
}

/// A client for the Saturn Zenoh messaging bus.
#[derive(Clone)]
pub struct ZenohClient {
    session: Arc<Session>,
}

impl ZenohClient {
    /// Opens a new Zenoh session with optional authentication.
    ///
    /// # Errors
    /// Returns [`ZenohError::Connection`] if the session cannot be opened.
    pub async fn new(
        connect: &str,
        user: Option<&str>,
        password: Option<&str>,
    ) -> Result<Self, ZenohError> {
        let mut config = zenoh::Config::default();

        if !connect.is_empty() {
            config
                .insert_json5("connect/endpoints", &format!("[\"{}\"]", connect))
                .map_err(|e| ZenohError::Connection(e.to_string()))?;
        }

        if let (Some(u), Some(p)) = (user, password) {
            config
                .insert_json5("transport/auth/usrpwd/user", &format!("\"{}\"", u))
                .map_err(|e| ZenohError::Connection(e.to_string()))?;
            config
                .insert_json5("transport/auth/usrpwd/password", &format!("\"{}\"", p))
                .map_err(|e| ZenohError::Connection(e.to_string()))?;
        }

        let session = zenoh::open(config)
            .await
            .map_err(|e| ZenohError::Connection(e.to_string()))?;

        Ok(Self {
            session: Arc::new(session),
        })
    }

    /// Publishes a payload to a specific key expression.
    pub async fn put(&self, key: &str, payload: String) -> Result<(), ZenohError> {
        self.session
            .put(key, payload)
            .await
            .map_err(|e| ZenohError::Publish(e.to_string()))
    }
}
