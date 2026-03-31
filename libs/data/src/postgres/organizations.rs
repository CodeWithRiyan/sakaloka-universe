//! Organization query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, uuid_to_string, PgError};
use super::PgClient;
use sakaloka_core::models::organization::Organization;

/// Internal row type matching the `organizations` table.
#[derive(sqlx::FromRow)]
struct OrgRow {
    id: uuid::Uuid,
    name: String,
    #[sqlx(rename = "type")]
    org_type: String,
    code: Option<String>,
    description: Option<String>,
    parent_id: Option<uuid::Uuid>,
    email: Option<String>,
    phone: Option<String>,
    website: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
    postal_code: Option<String>,
    tax_number: Option<String>,
    registration_number: Option<String>,
    logo: Option<String>,
    settings: Option<serde_json::Value>,
    is_active: bool,
    owner_id: Option<uuid::Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<OrgRow> for Organization {
    fn from(r: OrgRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            name: r.name,
            org_type: r.org_type,
            code: r.code,
            description: r.description,
            parent_id: r.parent_id.map(uuid_to_string),
            email: r.email,
            phone: r.phone,
            website: r.website,
            address: r.address,
            city: r.city,
            state: r.state,
            country: r.country,
            postal_code: r.postal_code,
            tax_number: r.tax_number,
            registration_number: r.registration_number,
            logo: r.logo,
            settings: r.settings,
            is_active: r.is_active,
            owner_id: r.owner_id.map(uuid_to_string),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl PgClient {
    /// Finds an organization by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_organization(&self, id: &str) -> Result<Option<Organization>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, OrgRow>("SELECT * FROM organizations WHERE id = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Organization::from))
    }

    /// Lists organizations with pagination, search, and sorting.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_organizations(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<Organization>, PgError> {
        let sort_col = super::rows::safe_sort_column(
            sort_by,
            &["name", "type", "code", "is_active", "created_at"],
            "created_at",
        );
        let dir = super::rows::sort_dir(sort_desc);
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let sql = format!(
            "SELECT * FROM organizations \
             WHERE ($1::uuid IS NULL OR id = $1) \
             AND ($2::text IS NULL OR name ILIKE $2) \
             ORDER BY {sort_col} {dir} LIMIT $3 OFFSET $4"
        );

        let rows = sqlx::query_as::<_, OrgRow>(&sql)
            .bind(org_uuid)
            .bind(search_pattern.as_deref())
            .bind(limit as i64)
            .bind(start as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Organization::from).collect())
    }

    /// Counts organizations with optional filters.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_organizations(
        &self,
        org_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM organizations \
             WHERE ($1::uuid IS NULL OR id = $1) \
             AND ($2::text IS NULL OR name ILIKE $2)",
        )
        .bind(org_uuid)
        .bind(search_pattern.as_deref())
        .fetch_one(&self.pool)
        .await?;

        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }

    /// Creates a new organization.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    pub async fn create_organization(
        &self,
        name: &str,
        org_type: &str,
        owner_id: Option<&str>,
    ) -> Result<Organization, PgError> {
        let owner_uuid = owner_id.map(parse_uuid).transpose()?;

        let row = sqlx::query_as::<_, OrgRow>(
            "INSERT INTO organizations (name, type, owner_id, is_active) \
             VALUES ($1, $2, $3, true) RETURNING *",
        )
        .bind(name)
        .bind(org_type)
        .bind(owner_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(Organization::from(row))
    }

    /// Updates an organization from a JSON patch object.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_organization(
        &self,
        id: &str,
        updates: &serde_json::Value,
    ) -> Result<Organization, PgError> {
        let uid = parse_uuid(id)?;
        let map = updates.as_object().ok_or_else(|| {
            PgError::Query("update_organization: expected JSON object".to_string())
        })?;

        if map.is_empty() {
            return self
                .find_organization(id)
                .await?
                .ok_or_else(|| PgError::NotFound("Organization".to_string()));
        }

        // Build dynamic SET clause
        let row = sqlx::query_as::<_, OrgRow>(
            "UPDATE organizations SET \
             name = COALESCE($2, name), \
             type = COALESCE($3, type), \
             code = CASE WHEN $4::text = '__null__' THEN code ELSE COALESCE($4, code) END, \
             description = CASE WHEN $5::text = '__null__' THEN description ELSE COALESCE($5, description) END, \
             email = CASE WHEN $6::text = '__null__' THEN email ELSE COALESCE($6, email) END, \
             phone = CASE WHEN $7::text = '__null__' THEN phone ELSE COALESCE($7, phone) END, \
             website = CASE WHEN $8::text = '__null__' THEN website ELSE COALESCE($8, website) END, \
             address = CASE WHEN $9::text = '__null__' THEN address ELSE COALESCE($9, address) END, \
             city = CASE WHEN $10::text = '__null__' THEN city ELSE COALESCE($10, city) END, \
             state = CASE WHEN $11::text = '__null__' THEN state ELSE COALESCE($11, state) END, \
             country = CASE WHEN $12::text = '__null__' THEN country ELSE COALESCE($12, country) END, \
             postal_code = CASE WHEN $13::text = '__null__' THEN postal_code ELSE COALESCE($13, postal_code) END, \
             tax_number = CASE WHEN $14::text = '__null__' THEN tax_number ELSE COALESCE($14, tax_number) END, \
             registration_number = CASE WHEN $15::text = '__null__' THEN registration_number ELSE COALESCE($15, registration_number) END, \
             logo = CASE WHEN $16::text = '__null__' THEN logo ELSE COALESCE($16, logo) END, \
             settings = COALESCE($17, settings), \
             is_active = COALESCE($18, is_active) \
             WHERE id = $1 RETURNING *",
        )
        .bind(uid)
        .bind(map.get("name").and_then(|v| v.as_str()))
        .bind(map.get("type").and_then(|v| v.as_str()))
        .bind(map.get("code").and_then(|v| v.as_str()))
        .bind(map.get("description").and_then(|v| v.as_str()))
        .bind(map.get("email").and_then(|v| v.as_str()))
        .bind(map.get("phone").and_then(|v| v.as_str()))
        .bind(map.get("website").and_then(|v| v.as_str()))
        .bind(map.get("address").and_then(|v| v.as_str()))
        .bind(map.get("city").and_then(|v| v.as_str()))
        .bind(map.get("state").and_then(|v| v.as_str()))
        .bind(map.get("country").and_then(|v| v.as_str()))
        .bind(map.get("postal_code").and_then(|v| v.as_str()))
        .bind(map.get("tax_number").and_then(|v| v.as_str()))
        .bind(map.get("registration_number").and_then(|v| v.as_str()))
        .bind(map.get("logo").and_then(|v| v.as_str()))
        .bind(map.get("settings").cloned())
        .bind(map.get("is_active").and_then(|v| v.as_bool()))
        .fetch_one(&self.pool)
        .await?;

        Ok(Organization::from(row))
    }

    /// Updates the owner of an organization.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_organization_owner(&self, id: &str, owner_id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        let owner_uuid = parse_uuid(owner_id)?;
        sqlx::query("UPDATE organizations SET owner_id = $2 WHERE id = $1")
            .bind(uid)
            .bind(owner_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Deletes an organization.
    ///
    /// # Errors
    /// Returns [`PgError`] if the deletion fails.
    pub async fn delete_organization(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("DELETE FROM organizations WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
