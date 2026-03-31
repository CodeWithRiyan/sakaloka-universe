//! Session query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, PgError};
use super::PgClient;
use sakaloka_core::models::user::User;

impl PgClient {
    /// Finds the user associated with a session by session ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_user_by_session(
        &self,
        session_id: &sakaloka_secure::newtypes::SessionId,
    ) -> Result<Option<User>, PgError> {
        let sid_str = session_id.to_record_id_string().replace("session:", "");
        let sid = uuid::Uuid::parse_str(&sid_str)
            .map_err(|e| PgError::Query(format!("invalid session UUID: {e}")))?;

        #[derive(sqlx::FromRow)]
        struct UserRow {
            id: uuid::Uuid,
            username: Option<String>,
            email: String,
            full_name: Option<String>,
            password_hash: String,
            organization_id: Option<uuid::Uuid>,
            role_id: Option<uuid::Uuid>,
            is_active: bool,
            last_login_at: Option<chrono::DateTime<chrono::Utc>>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }

        let row = sqlx::query_as::<_, UserRow>(
            "SELECT u.* FROM users u \
             JOIN sessions s ON s.user_id = u.id \
             WHERE s.id = $1 AND s.expires_at > NOW() LIMIT 1",
        )
        .bind(sid)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: super::rows::uuid_to_string(r.id),
            username: r.username,
            email: r.email,
            full_name: r.full_name,
            password_hash: r.password_hash,
            organization_id: r.organization_id.map(super::rows::uuid_to_string),
            role_id: r.role_id.map(super::rows::uuid_to_string),
            is_active: r.is_active,
            last_login_at: r.last_login_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    /// Creates a new session with a hashed refresh token.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    pub async fn create_session(
        &self,
        user_id: &sakaloka_secure::newtypes::UserId,
        session_id: &sakaloka_secure::newtypes::SessionId,
        token_hash: &str,
    ) -> Result<(), PgError> {
        let uid_str = user_id.as_str().replace("user:", "");
        let uid = parse_uuid(&uid_str)?;
        let sid_str = session_id.to_record_id_string().replace("session:", "");
        let sid = parse_uuid(&sid_str)?;

        sqlx::query(
            "INSERT INTO sessions (id, user_id, token_hash, expires_at) \
             VALUES ($1, $2, $3, NOW() + INTERVAL '7 days')",
        )
        .bind(sid)
        .bind(uid)
        .bind(token_hash)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
