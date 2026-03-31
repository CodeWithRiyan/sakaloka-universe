//! Category query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, uuid_to_string, PgError};
use super::PgClient;
use sakaloka_core::models::category::Category;

/// Internal row type matching the `categories` table.
#[derive(sqlx::FromRow)]
struct CategoryRow {
    id: uuid::Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    parent_id: Option<uuid::Uuid>,
    image_url: Option<String>,
    sort_order: i64,
    is_active: bool,
    organization_id: uuid::Uuid,
    created_by: Option<uuid::Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<CategoryRow> for Category {
    fn from(r: CategoryRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            name: r.name,
            slug: r.slug,
            description: r.description,
            parent_id: r.parent_id.map(uuid_to_string),
            image_url: r.image_url,
            sort_order: r.sort_order,
            is_active: r.is_active,
            organization_id: uuid_to_string(r.organization_id),
            created_by: r.created_by.map(uuid_to_string),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl PgClient {
    /// Finds a category by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_category(&self, id: &str) -> Result<Option<Category>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, CategoryRow>("SELECT * FROM categories WHERE id = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Category::from))
    }

    /// Fetches multiple categories by IDs.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_categories_by_ids(&self, ids: &[String]) -> Result<Vec<Category>, PgError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let uuids: Vec<uuid::Uuid> = ids
            .iter()
            .map(|id| parse_uuid(id))
            .collect::<Result<_, _>>()?;
        let rows = sqlx::query_as::<_, CategoryRow>("SELECT * FROM categories WHERE id = ANY($1)")
            .bind(&uuids)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Category::from).collect())
    }

    /// Lists categories with pagination, search, and sorting.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_categories(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<Category>, PgError> {
        let sort_col = super::rows::safe_sort_column(
            sort_by,
            &["name", "slug", "sort_order", "is_active", "created_at"],
            "created_at",
        );
        let dir = super::rows::sort_dir(sort_desc);
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let sql = format!(
            "SELECT * FROM categories \
             WHERE ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR name ILIKE $2) \
             ORDER BY {sort_col} {dir} LIMIT $3 OFFSET $4"
        );

        let rows = sqlx::query_as::<_, CategoryRow>(&sql)
            .bind(org_uuid)
            .bind(search_pattern.as_deref())
            .bind(limit as i64)
            .bind(start as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Category::from).collect())
    }

    /// Counts categories with optional filters.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_categories(
        &self,
        org_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM categories \
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

    /// Creates a new category.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_category(
        &self,
        name: &str,
        slug: &str,
        organization_id: &str,
        description: Option<&str>,
        parent_id: Option<&str>,
        image_url: Option<&str>,
        sort_order: i64,
        created_by: &str,
    ) -> Result<Category, PgError> {
        let org_uuid = parse_uuid(organization_id)?;
        let creator_uuid = parse_uuid(created_by)?;
        let parent_uuid = parent_id.map(parse_uuid).transpose()?;

        let row = sqlx::query_as::<_, CategoryRow>(
            "INSERT INTO categories \
             (name, slug, organization_id, description, parent_id, image_url, sort_order, created_by, is_active) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,true) RETURNING *",
        )
        .bind(name)
        .bind(slug)
        .bind(org_uuid)
        .bind(description)
        .bind(parent_uuid)
        .bind(image_url)
        .bind(sort_order)
        .bind(creator_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(Category::from(row))
    }

    /// Updates a category from a JSON patch.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_category(
        &self,
        id: &str,
        updates: &serde_json::Value,
    ) -> Result<Category, PgError> {
        let uid = parse_uuid(id)?;
        let map = updates
            .as_object()
            .ok_or_else(|| PgError::Query("expected JSON object".to_string()))?;

        let parent_uuid = map
            .get("parent_id")
            .and_then(|v| v.as_str())
            .map(parse_uuid)
            .transpose()?;

        let row = sqlx::query_as::<_, CategoryRow>(
            "UPDATE categories SET \
             name = COALESCE($2, name), \
             slug = COALESCE($3, slug), \
             description = COALESCE($4, description), \
             parent_id = COALESCE($5, parent_id), \
             image_url = COALESCE($6, image_url), \
             sort_order = COALESCE($7, sort_order), \
             is_active = COALESCE($8, is_active) \
             WHERE id = $1 RETURNING *",
        )
        .bind(uid)
        .bind(map.get("name").and_then(|v| v.as_str()))
        .bind(map.get("slug").and_then(|v| v.as_str()))
        .bind(map.get("description").and_then(|v| v.as_str()))
        .bind(parent_uuid)
        .bind(map.get("image_url").and_then(|v| v.as_str()))
        .bind(map.get("sort_order").and_then(|v| v.as_i64()))
        .bind(map.get("is_active").and_then(|v| v.as_bool()))
        .fetch_one(&self.pool)
        .await?;

        Ok(Category::from(row))
    }

    /// Deletes a category by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the deletion fails.
    pub async fn delete_category(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Counts child categories of a given parent.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_child_categories(&self, parent_id: &str) -> Result<u64, PgError> {
        let uid = parse_uuid(parent_id)?;
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories WHERE parent_id = $1")
            .bind(uid)
            .fetch_one(&self.pool)
            .await?;
        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }
}
