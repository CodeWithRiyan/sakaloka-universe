//! Role query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, uuid_to_string, PgError};
use super::PgClient;
use sakaloka_core::models::role::Role;

/// Internal row type matching the `roles` table.
#[derive(sqlx::FromRow)]
struct RoleRow {
    id: uuid::Uuid,
    name: String,
    description: Option<String>,
    permissions: Option<serde_json::Value>,
    organization_id: uuid::Uuid,
    is_system_role: bool,
    is_active: bool,
    created_by: Option<uuid::Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<RoleRow> for Role {
    fn from(r: RoleRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            name: r.name,
            description: r.description,
            permissions: r.permissions,
            organization_id: uuid_to_string(r.organization_id),
            is_system_role: r.is_system_role,
            is_active: r.is_active,
            created_by: r.created_by.map(uuid_to_string),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl PgClient {
    /// Finds a role by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_role(&self, id: &str) -> Result<Option<Role>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, RoleRow>("SELECT * FROM roles WHERE id = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Role::from))
    }

    /// Lists roles with pagination, search, and sorting.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_roles(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<Role>, PgError> {
        let sort_col = super::rows::safe_sort_column(
            sort_by,
            &["name", "is_system_role", "is_active", "created_at"],
            "created_at",
        );
        let dir = super::rows::sort_dir(sort_desc);
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let sql = format!(
            "SELECT * FROM roles \
             WHERE ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR name ILIKE $2) \
             ORDER BY {sort_col} {dir} LIMIT $3 OFFSET $4"
        );

        let rows = sqlx::query_as::<_, RoleRow>(&sql)
            .bind(org_uuid)
            .bind(search_pattern.as_deref())
            .bind(limit as i64)
            .bind(start as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Role::from).collect())
    }

    /// Counts roles with optional filters.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_roles(
        &self,
        org_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM roles \
             WHERE ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR name ILIKE $2)",
        )
        .bind(org_uuid)
        .bind(search_pattern.as_deref())
        .fetch_one(&self.pool)
        .await?;

        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }

    /// Creates a new role.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    pub async fn create_role(
        &self,
        name: &str,
        organization_id: &str,
        permissions: &serde_json::Value,
        is_system_role: bool,
    ) -> Result<Role, PgError> {
        let org_uuid = parse_uuid(organization_id)?;

        let row = sqlx::query_as::<_, RoleRow>(
            "INSERT INTO roles (name, organization_id, permissions, is_system_role, is_active) \
             VALUES ($1, $2, $3, $4, true) RETURNING *",
        )
        .bind(name)
        .bind(org_uuid)
        .bind(permissions)
        .bind(is_system_role)
        .fetch_one(&self.pool)
        .await?;

        Ok(Role::from(row))
    }

    /// Updates a role with optional fields.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_role(
        &self,
        id: &str,
        name: Option<&str>,
        permissions: Option<&serde_json::Value>,
        is_active: Option<bool>,
    ) -> Result<Role, PgError> {
        let uid = parse_uuid(id)?;

        let row = sqlx::query_as::<_, RoleRow>(
            "UPDATE roles SET \
             name = COALESCE($2, name), \
             permissions = COALESCE($3, permissions), \
             is_active = COALESCE($4, is_active) \
             WHERE id = $1 RETURNING *",
        )
        .bind(uid)
        .bind(name)
        .bind(permissions.cloned())
        .bind(is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(Role::from(row))
    }

    /// Deletes a role by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the deletion fails.
    pub async fn delete_role(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Finds a role by name within a specific organization.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_role_by_name_and_org(
        &self,
        name: &str,
        organization_id: &str,
    ) -> Result<Option<Role>, PgError> {
        let org_uuid = parse_uuid(organization_id)?;
        let row = sqlx::query_as::<_, RoleRow>(
            "SELECT * FROM roles WHERE name = $1 AND organization_id = $2 LIMIT 1",
        )
        .bind(name)
        .bind(org_uuid)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(Role::from))
    }

    /// Counts users with a specific role.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_users_with_role(&self, role_id: &str) -> Result<u64, PgError> {
        let uid = parse_uuid(role_id)?;
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE role_id = $1")
            .bind(uid)
            .fetch_one(&self.pool)
            .await?;
        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }
}
