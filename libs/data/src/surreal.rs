//! SurrealDB (Jupiter) client abstraction.
//!
//! Always use the authenticated client from this module.
//! Never embed raw SurrealDB credentials in application code.

use thiserror::Error;
use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;

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
    db: surrealdb::Surreal<surrealdb::engine::remote::http::Client>,
}

impl SurrealClient {
    /// Initialize a new SurrealDB connection to a specific endpoint.
    ///
    /// # Errors
    /// Returns a [`SurrealError`] if the connection fails or `USE` fails.
    pub async fn connect(url: &str) -> Result<Self, SurrealError> {
        let clean_url = url.trim_start_matches("http://").trim_start_matches("ws://");
        tracing::info!(url = %clean_url, "🧵 SurrealClient connecting with HTTP engine");
        
        // Add a timeout to avoid hanging indefinitely
        let db = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            surrealdb::Surreal::new::<surrealdb::engine::remote::http::Http>(clean_url)
        ).await
        .map_err(|_| SurrealError::Connection("Connection timeout at engine init".to_string()))?
        .map_err(|e| SurrealError::Connection(e.to_string()))?;
        
        tracing::info!("🧵 SurrealClient engine initialized");

        tracing::info!("🧵 SurrealClient switching to sakaloka/universe");
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            db.use_ns("sakaloka").use_db("universe")
        ).await
        .map_err(|_| SurrealError::Connection("Timeout switching namespace/db".to_string()))?
        .map_err(|e| SurrealError::Connection(e.to_string()))?;
        
        tracing::info!("🧵 SurrealClient namespace/db set");

        Ok(Self { db })
    }

    /// Authenticate the global client connection using a Service JWT.
    ///
    /// The token must be signed correctly and have the proper audience (`sakaloka:jupiter`).
    pub async fn authenticate(&self, token: &str) -> Result<(), SurrealError> {
        self.db
            .authenticate(token)
            .await
            .map(|_| ())
            .map_err(|e| SurrealError::Connection(format!("Token auth failed: {}", e)))
    }

    /// Sign in using root/user credentials.
    pub async fn signin(&self, user: &str, pass: &str) -> Result<(), SurrealError> {
        self.db
            .signin(surrealdb::opt::auth::Root {
                username: user.to_string(),
                password: pass.to_string(),
            })
            .await
            .map(|_| ())
            .map_err(|e| SurrealError::Connection(format!("Signin failed: {}", e)))
    }

    /// Finds a user by their username.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<sakaloka_core::models::user::User>, SurrealError> {
        let mut result = self
            .db
            .query("SELECT * FROM user WHERE username = $username LIMIT 1")
            .bind(("username", username.to_string()))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let user: Option<sakaloka_core::models::user::User> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(user)
    }

    /// Finds the user associated with a given session.
    pub async fn find_user_by_session(
        &self,
        session_id: &sakaloka_secure::newtypes::SessionId,
    ) -> Result<Option<sakaloka_core::models::user::User>, SurrealError> {
        let sid = surrealdb_types::RecordId::new(
            "session",
            surrealdb_types::RecordIdKey::String(session_id.as_str().replace("session:", "")),
        );

        let mut result = self
            .db
            .query("SELECT user_id.* AS user FROM session WHERE id = $sid LIMIT 1")
            .bind(("sid", sid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        #[derive(serde::Deserialize, SurrealValue)]
        struct SessionUser {
            user: Vec<sakaloka_core::models::user::User>,
        }

        let session: Option<SessionUser> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(session.and_then(|s| s.user.into_iter().next()))
    }

    /// Creates a new session and its initial refresh token.
    pub async fn create_session(
        &self,
        user_id: &sakaloka_secure::newtypes::UserId,
        session_id: &sakaloka_secure::newtypes::SessionId,
        token_hash: &str,
    ) -> Result<(), SurrealError> {
        let uid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(user_id.as_str().replace("user:", "")),
        );
        let sid = surrealdb_types::RecordId::new(
            "session",
            surrealdb_types::RecordIdKey::String(session_id.as_str().replace("session:", "")),
        );

        self.db.query("
            BEGIN TRANSACTION;
            LET $expires = time::now() + 7d;
            INSERT INTO session (id, user_id, expires_at) VALUES ($sid, $uid, $expires);
            INSERT INTO refresh_token (session_id, token_hash, expires_at) VALUES ($sid, $token_hash, $expires);
            COMMIT TRANSACTION;
        ")
            .bind(("sid", sid))
            .bind(("uid", uid))
            .bind(("token_hash", token_hash.to_string()))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    // --- Product CRUD ---

    /// Finds a product by its record ID.
    pub async fn find_product(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::product::Product>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(id.replace("product:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let product: Option<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(product)
    }

    /// Lists all products.
    pub async fn list_products(
        &self,
    ) -> Result<Vec<sakaloka_core::models::product::Product>, SurrealError> {
        let mut result = self
            .db
            .query("SELECT * FROM product ORDER BY created_at DESC")
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let products: Vec<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(products)
    }

    /// Creates a new product.
    pub async fn create_product(
        &self,
        name: &str,
        sku: &str,
        price: u64,
        description: Option<&str>,
    ) -> Result<sakaloka_core::models::product::Product, SurrealError> {
        let mut result = self.db.query("
            CREATE product SET 
                name = $name,
                sku = $sku,
                price = $price,
                description = $description
        ")
            .bind(("name", name.to_string()))
            .bind(("sku", sku.to_string()))
            .bind(("price", price))
            .bind(("description", description.map(|s| s.to_string())))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let product: Option<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        product.ok_or_else(|| SurrealError::Query("Failed to create product".into()))
    }

    /// Updates an existing product.
    pub async fn update_product(
        &self,
        id: &str,
        name: Option<&str>,
        price: Option<u64>,
        description: Option<&str>,
    ) -> Result<sakaloka_core::models::product::Product, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(id.replace("product:", "")),
        );

        let mut result = self.db.query("
            UPDATE $id MERGE {
                name: IF $name != NONE THEN $name ELSE name END,
                price: IF $price != NONE THEN $price ELSE price END,
                description: IF $description != NONE THEN $description ELSE description END
            }
        ")
            .bind(("id", tid))
            .bind(("name", name.map(String::from)))
            .bind(("price", price))
            .bind(("description", description.map(String::from)))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let product: Option<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        product.ok_or_else(|| SurrealError::Query("Product not found or update failed".into()))
    }

    /// Deletes a product.
    pub async fn delete_product(&self, id: &str) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(id.replace("product:", "")),
        );

        self.db
            .query("DELETE $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }
}

impl sakaloka_secure::tokens::rotation::JtiStore for SurrealClient {
    async fn is_jti_blocked(&self, jti: &str) -> Result<bool, sakaloka_secure::error::SecureError> {
        let mut result = self
            .db
            .query("SELECT * FROM jti_blocklist WHERE jti = $jti LIMIT 1")
            .bind(("jti", jti.to_string()))
            .await
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e))
            })?;

        let rows: Vec<serde_json::Value> = result.take(0).map_err(|e| {
            sakaloka_secure::error::SecureError::JwtDecode(format!("Query error: {}", e))
        })?;

        Ok(!rows.is_empty())
    }

    async fn block_jti(
        &self,
        jti: &str,
        exp: u64,
    ) -> Result<(), sakaloka_secure::error::SecureError> {
        self.db
            .query("INSERT INTO jti_blocklist (jti, exp) VALUES ($jti, $exp)")
            .bind(("jti", jti.to_string()))
            .bind(("exp", exp))
            .await
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e))
            })?
            .check()
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("Execution error: {}", e))
            })?;
        Ok(())
    }
}
impl sakaloka_secure::tokens::rotation::RefreshStore for SurrealClient {
    async fn find_token(
        &self,
        hash: &str,
    ) -> Result<
        Option<sakaloka_secure::tokens::rotation::RefreshTokenRecord>,
        sakaloka_secure::error::SecureError,
    > {
        let mut result = self
            .db
            .query("SELECT session_id, expires_at, rotated_at FROM refresh_token WHERE token_hash = $hash LIMIT 1")
            .bind(("hash", hash.to_string()))
            .await
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e)))?;

        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct TokenRow {
            session_id: surrealdb_types::RecordId,
            expires_at: surrealdb_types::Datetime,
            rotated_at: Option<surrealdb_types::Datetime>,
        }

        let token: Option<TokenRow> = result.take(0).map_err(|e| {
            sakaloka_secure::error::SecureError::JwtDecode(format!("Query error: {}", e))
        })?;

        Ok(
            token.map(|t| {
                let expires_at: chrono::DateTime<chrono::Utc> = t.expires_at.into();
                let rotated_at: Option<chrono::DateTime<chrono::Utc>> = t.rotated_at.map(|r| r.into());
                let sid_raw = match &t.session_id.key {
                    surrealdb_types::RecordIdKey::String(s) => s.clone(),
                    surrealdb_types::RecordIdKey::Uuid(u) => u.to_string(),
                    _ => format!("{:?}", t.session_id.key),
                };
                sakaloka_secure::tokens::rotation::RefreshTokenRecord {
                    session_id: sakaloka_secure::newtypes::SessionId::new_with_raw(&sid_raw),
                    expires_at: expires_at.timestamp() as u64,
                    rotated_at: rotated_at.map(|r| r.timestamp() as u64),
                }
            }),
        )
    }

    async fn rotate_token(
        &self,
        old_hash: &str,
        new_hash: &str,
        session_id: &sakaloka_secure::newtypes::SessionId,
        expires_at: u64,
    ) -> Result<(), sakaloka_secure::error::SecureError> {
        let sid = surrealdb_types::RecordIdKey::String(session_id.as_str().to_string());
        let session_thing = surrealdb_types::RecordId::new("session", sid);
        let expiry = chrono::DateTime::from_timestamp(expires_at as i64, 0).unwrap_or_default();

        self.db.query("
            BEGIN TRANSACTION;
            UPDATE refresh_token SET rotated_at = time::now() WHERE token_hash = $old_hash;
            INSERT INTO refresh_token (session_id, token_hash, expires_at) VALUES ($session_id, $new_hash, $expires_at);
            COMMIT TRANSACTION;
        ")
            .bind(("old_hash", old_hash.to_string()))
            .bind(("session_id", session_thing))
            .bind(("new_hash", new_hash.to_string()))
            .bind(("expires_at", expiry))
            .await
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e)))?
            .check()
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("Execution error: {}", e)))?;

        Ok(())
    }

    async fn terminate_session(
        &self,
        session_id: &sakaloka_secure::newtypes::SessionId,
    ) -> Result<(), sakaloka_secure::error::SecureError> {
        let sid = surrealdb_types::RecordIdKey::String(session_id.as_str().to_string());
        let session_thing = surrealdb_types::RecordId::new("session", sid);

        self.db
            .query("DELETE $session_id")
            .bind(("session_id", session_thing))
            .await
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e))
            })?
            .check()
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("Execution error: {}", e))
            })?;

        Ok(())
    }
}
