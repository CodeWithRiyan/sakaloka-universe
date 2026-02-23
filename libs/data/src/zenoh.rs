//! Eclipse Zenoh (Saturn) messaging client abstraction.
//!
//! All Zenoh event handlers MUST be idempotent — the same event delivered
//! twice must produce the same result. Use sequence numbers in payloads
//! to detect duplicates.
//!
//! Topic naming convention:
//! `sakaloka/{source_planet}/{entity}/{event_type}`

use thiserror::Error;

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
