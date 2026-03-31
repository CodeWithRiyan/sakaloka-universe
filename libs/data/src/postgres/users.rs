//! User query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, uuid_to_string, PgError};
use super::PgClient;
use sakaloka_core::models::user::User;

/// Internal row type matching the `users` table.
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

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            username: r.username,
            email: r.email,
            full_name: r.full_name,
            password_hash: r.password_hash,
            organization_id: r.organization_id.map(uuid_to_string),
            role_id: r.role_id.map(uuid_to_string),
            is_active: r.is_active,
            last_login_at: r.last_login_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl PgClient {
    /// Finds a user by email address.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, PgError> {
        let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = $1 LIMIT 1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(User::from))
    }

    /// Finds a user by username.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, PgError> {
        let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE username = $1 LIMIT 1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(User::from))
    }

    /// Finds a user by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_user_by_id(&self, id: &str) -> Result<Option<User>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(User::from))
    }

    /// Lists users with pagination, optional search, and sorting.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_users(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<User>, PgError> {
        let sort_col = super::rows::safe_sort_column(
            sort_by,
            &["email", "full_name", "username", "is_active", "created_at"],
            "created_at",
        );
        let dir = super::rows::sort_dir(sort_desc);

        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let sql = format!(
            "SELECT * FROM users WHERE ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR email ILIKE $2 OR full_name ILIKE $2 OR username ILIKE $2) \
             ORDER BY {sort_col} {dir} LIMIT $3 OFFSET $4"
        );

        let rows = sqlx::query_as::<_, UserRow>(&sql)
            .bind(org_uuid)
            .bind(search_pattern.as_deref())
            .bind(limit as i64)
            .bind(start as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(User::from).collect())
    }

    /// Counts users with optional org and search filters.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_users(
        &self,
        org_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR email ILIKE $2 OR full_name ILIKE $2 OR username ILIKE $2)",
        )
        .bind(org_uuid)
        .bind(search_pattern.as_deref())
        .fetch_one(&self.pool)
        .await?;

        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }

    /// Creates a new user.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    pub async fn create_user(
        &self,
        email: &str,
        full_name: &str,
        password_hash: &str,
        organization_id: &str,
        role_id: &str,
    ) -> Result<User, PgError> {
        let org_uuid = parse_uuid(organization_id)?;
        let role_uuid = parse_uuid(role_id)?;

        let row = sqlx::query_as::<_, UserRow>(
            "INSERT INTO users (email, full_name, password_hash, organization_id, role_id, is_active) \
             VALUES ($1, $2, $3, $4, $5, true) RETURNING *",
        )
        .bind(email)
        .bind(full_name)
        .bind(password_hash)
        .bind(org_uuid)
        .bind(role_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(row))
    }

    /// Updates an existing user with optional fields.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_user(
        &self,
        id: &str,
        email: Option<&str>,
        full_name: Option<&str>,
        password_hash: Option<&str>,
        role_id: Option<&str>,
        is_active: Option<bool>,
    ) -> Result<User, PgError> {
        let uid = parse_uuid(id)?;
        let role_uuid = role_id.map(parse_uuid).transpose()?;

        let row = sqlx::query_as::<_, UserRow>(
            "UPDATE users SET \
             email = COALESCE($2, email), \
             full_name = COALESCE($3, full_name), \
             password_hash = COALESCE($4, password_hash), \
             role_id = COALESCE($5, role_id), \
             is_active = COALESCE($6, is_active) \
             WHERE id = $1 RETURNING *",
        )
        .bind(uid)
        .bind(email)
        .bind(full_name)
        .bind(password_hash)
        .bind(role_uuid)
        .bind(is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(row))
    }

    /// Switches a user's active organization.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_user_organization(
        &self,
        user_id: &str,
        organization_id: &str,
    ) -> Result<(), PgError> {
        let uid = parse_uuid(user_id)?;
        let org_uuid = parse_uuid(organization_id)?;

        sqlx::query("UPDATE users SET organization_id = $2 WHERE id = $1")
            .bind(uid)
            .bind(org_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Updates last_login_at to NOW().
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_last_login(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Deletes a user record.
    ///
    /// # Errors
    /// Returns [`PgError`] if the deletion fails.
    pub async fn delete_user(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
