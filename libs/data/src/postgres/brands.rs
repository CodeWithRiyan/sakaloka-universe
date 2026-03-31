//! Brand query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, uuid_to_string, PgError};
use super::PgClient;
use sakaloka_core::models::brand::Brand;

/// Internal row type matching the `brands` table.
#[derive(sqlx::FromRow)]
struct BrandRow {
    id: uuid::Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    logo: Option<String>,
    website: Option<String>,
    is_active: bool,
    organization_id: uuid::Uuid,
    created_by: Option<uuid::Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<BrandRow> for Brand {
    fn from(r: BrandRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            name: r.name,
            slug: r.slug,
            description: r.description,
            logo: r.logo,
            website: r.website,
            is_active: r.is_active,
            organization_id: uuid_to_string(r.organization_id),
            created_by: r.created_by.map(uuid_to_string),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl PgClient {
    /// Finds a brand by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_brand(&self, id: &str) -> Result<Option<Brand>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, BrandRow>("SELECT * FROM brands WHERE id = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Brand::from))
    }

    /// Fetches multiple brands by IDs.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_brands_by_ids(&self, ids: &[String]) -> Result<Vec<Brand>, PgError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let uuids: Vec<uuid::Uuid> = ids
            .iter()
            .map(|id| parse_uuid(id))
            .collect::<Result<_, _>>()?;
        let rows = sqlx::query_as::<_, BrandRow>("SELECT * FROM brands WHERE id = ANY($1)")
            .bind(&uuids)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Brand::from).collect())
    }

    /// Lists brands with pagination, search, and sorting.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_brands(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<Brand>, PgError> {
        let sort_col = super::rows::safe_sort_column(
            sort_by,
            &["name", "slug", "is_active", "created_at"],
            "created_at",
        );
        let dir = super::rows::sort_dir(sort_desc);
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let sql = format!(
            "SELECT * FROM brands \
             WHERE ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR name ILIKE $2) \
             ORDER BY {sort_col} {dir} LIMIT $3 OFFSET $4"
        );

        let rows = sqlx::query_as::<_, BrandRow>(&sql)
            .bind(org_uuid)
            .bind(search_pattern.as_deref())
            .bind(limit as i64)
            .bind(start as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Brand::from).collect())
    }

    /// Counts brands with optional filters.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_brands(
        &self,
        org_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM brands \
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

    /// Creates a new brand.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_brand(
        &self,
        name: &str,
        slug: &str,
        organization_id: &str,
        description: Option<&str>,
        logo: Option<&str>,
        website: Option<&str>,
        created_by: &str,
    ) -> Result<Brand, PgError> {
        let org_uuid = parse_uuid(organization_id)?;
        let creator_uuid = parse_uuid(created_by)?;

        let row = sqlx::query_as::<_, BrandRow>(
            "INSERT INTO brands (name, slug, organization_id, description, logo, website, created_by, is_active) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,true) RETURNING *",
        )
        .bind(name)
        .bind(slug)
        .bind(org_uuid)
        .bind(description)
        .bind(logo)
        .bind(website)
        .bind(creator_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(Brand::from(row))
    }

    /// Updates a brand from a JSON patch.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_brand(
        &self,
        id: &str,
        updates: &serde_json::Value,
    ) -> Result<Brand, PgError> {
        let uid = parse_uuid(id)?;
        let map = updates
            .as_object()
            .ok_or_else(|| PgError::Query("expected JSON object".to_string()))?;

        let row = sqlx::query_as::<_, BrandRow>(
            "UPDATE brands SET \
             name = COALESCE($2, name), \
             slug = COALESCE($3, slug), \
             description = COALESCE($4, description), \
             logo = COALESCE($5, logo), \
             website = COALESCE($6, website), \
             is_active = COALESCE($7, is_active) \
             WHERE id = $1 RETURNING *",
        )
        .bind(uid)
        .bind(map.get("name").and_then(|v| v.as_str()))
        .bind(map.get("slug").and_then(|v| v.as_str()))
        .bind(map.get("description").and_then(|v| v.as_str()))
        .bind(map.get("logo").and_then(|v| v.as_str()))
        .bind(map.get("website").and_then(|v| v.as_str()))
        .bind(map.get("is_active").and_then(|v| v.as_bool()))
        .fetch_one(&self.pool)
        .await?;

        Ok(Brand::from(row))
    }

    /// Deletes a brand by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the deletion fails.
    pub async fn delete_brand(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("DELETE FROM brands WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
