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

    /// Authenticate the global client connection using a Service JWT.
    ///
    /// The token must be signed correctly and have the proper audience (`sakaloka:jupiter`).
    pub async fn authenticate(&self, token: &str) -> Result<(), SurrealError> {
        self.db
            .authenticate(token)
            .await
            .map_err(|e| SurrealError::Connection(format!("Token auth failed: {}", e)))
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
        let sid = surrealdb::sql::Thing::from((
            "session",
            surrealdb::sql::Id::String(session_id.as_str().replace("session:", "")),
        ));

        let mut result = self
            .db
            .query("SELECT user_id.* AS user FROM session WHERE id = $sid LIMIT 1")
            .bind(("sid", sid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        #[derive(serde::Deserialize)]
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
        let uid = surrealdb::sql::Thing::from((
            "user",
            surrealdb::sql::Id::String(user_id.as_str().replace("user:", "")),
        ));
        let sid = surrealdb::sql::Thing::from((
            "session",
            surrealdb::sql::Id::String(session_id.as_str().replace("session:", "")),
        ));

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

        #[derive(serde::Deserialize)]
        struct TokenRow {
            session_id: surrealdb::sql::Thing,
            expires_at: surrealdb::sql::Datetime,
            rotated_at: Option<surrealdb::sql::Datetime>,
        }

        let token: Option<TokenRow> = result.take(0).map_err(|e| {
            sakaloka_secure::error::SecureError::JwtDecode(format!("Query error: {}", e))
        })?;

        Ok(
            token.map(|t| sakaloka_secure::tokens::rotation::RefreshTokenRecord {
                session_id: sakaloka_secure::newtypes::SessionId::new_with_raw(
                    &t.session_id.to_raw(),
                ),
                expires_at: t.expires_at.0.timestamp() as u64,
                rotated_at: t.rotated_at.map(|r| r.0.timestamp() as u64),
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
        let sid = surrealdb::sql::Id::String(session_id.as_str().to_string());
        let session_thing = surrealdb::sql::Thing::from(("session", sid));
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
        let sid = surrealdb::sql::Id::String(session_id.as_str().to_string());
        let session_thing = surrealdb::sql::Thing::from(("session", sid));

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
