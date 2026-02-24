//! SurrealDB (Jupiter) client abstraction.
//!
//! Always use the authenticated client from this module.
//! Never embed raw SurrealDB credentials in application code.

use thiserror::Error;

/// Errors produced by the SurrealDB client layer.
#[derive(Debug, Error)]
pub enum SurrealError {
    /// Connection to SurrealDB failed.
    #[error("SurrealDB connection failed: {0}")]
    Connection(String),
    /// A query execution error.
    #[error("SurrealDB query error: {0}")]
    Query(String),
}

/// A client for interacting with the Jupiter SurrealDB instance.
#[derive(Clone)]
pub struct SurrealClient {
    db: surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
}

impl SurrealClient {
    /// Initialize a new SurrealDB connection to a specific endpoint.
    ///
    /// # Errors
    /// Returns a [`SurrealError`] if the connection fails or `USE` fails.
    pub async fn connect(url: &str) -> Result<Self, SurrealError> {
        let db = surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>(url)
            .await
            .map_err(|e| SurrealError::Connection(e.to_string()))?;
        
        db.use_ns("sakaloka")
            .use_db("universe")
            .await
            .map_err(|e| SurrealError::Connection(e.to_string()))?;

        Ok(Self { db })
    }
}

impl sakaloka_secure::tokens::rotation::JtiStore for SurrealClient {
    async fn is_jti_blocked(&self, jti: &str) -> Result<bool, sakaloka_secure::error::SecureError> {
        let mut result = self
            .db
            .query("SELECT * FROM jti_blocklist WHERE jti = $jti LIMIT 1")
            .bind(("jti", jti.to_string()))
            .await
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e)))?;
        
        let rows: Vec<serde_json::Value> = result
            .take(0)
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("Query error: {}", e)))?;

        Ok(!rows.is_empty())
    }

    async fn block_jti(&self, jti: &str, exp: u64) -> Result<(), sakaloka_secure::error::SecureError> {
        self.db
            .query("INSERT INTO jti_blocklist (jti, exp) VALUES ($jti, $exp)")
            .bind(("jti", jti.to_string()))
            .bind(("exp", exp))
            .await
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e)))?
            .check()
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("Execution error: {}", e)))?;
        Ok(())
    }
}
