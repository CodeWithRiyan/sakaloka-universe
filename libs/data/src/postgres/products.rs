//! Product query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, uuid_to_string, PgError};
use super::PgClient;
use sakaloka_core::models::product::Product;

/// Internal row type matching the `products` table.
#[derive(sqlx::FromRow)]
struct ProductRow {
    id: uuid::Uuid,
    name: String,
    description: Option<String>,
    sku: String,
    barcode: Option<String>,
    base_price: i64,
    cost_price: Option<i64>,
    category_id: Option<uuid::Uuid>,
    brand_id: Option<uuid::Uuid>,
    image_url: Option<String>,
    weight: Option<f64>,
    dimensions: Option<serde_json::Value>,
    track_inventory: bool,
    min_stock_level: i64,
    is_featured: bool,
    tags: Option<Vec<String>>,
    organization_id: uuid::Uuid,
    created_by: Option<uuid::Uuid>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<ProductRow> for Product {
    fn from(r: ProductRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            name: r.name,
            description: r.description,
            sku: r.sku,
            barcode: r.barcode,
            base_price: r.base_price,
            cost_price: r.cost_price,
            category_id: r.category_id.map(uuid_to_string),
            brand_id: r.brand_id.map(uuid_to_string),
            image_url: r.image_url,
            weight: r.weight,
            dimensions: r.dimensions,
            track_inventory: r.track_inventory,
            min_stock_level: r.min_stock_level,
            is_featured: r.is_featured,
            tags: r.tags,
            organization_id: uuid_to_string(r.organization_id),
            created_by: r.created_by.map(uuid_to_string),
            deleted_at: r.deleted_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl PgClient {
    /// Finds a product by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_product(&self, id: &str) -> Result<Option<Product>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, ProductRow>("SELECT * FROM products WHERE id = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Product::from))
    }

    /// Fetches multiple products by IDs.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_products_by_ids(&self, ids: &[String]) -> Result<Vec<Product>, PgError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let uuids: Vec<uuid::Uuid> = ids
            .iter()
            .map(|id| parse_uuid(id))
            .collect::<Result<_, _>>()?;
        let rows = sqlx::query_as::<_, ProductRow>("SELECT * FROM products WHERE id = ANY($1)")
            .bind(&uuids)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Product::from).collect())
    }

    /// Lists products with pagination, search, and sorting.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_products(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<Product>, PgError> {
        let sort_col = super::rows::safe_sort_column(
            sort_by,
            &["name", "sku", "base_price", "is_featured", "created_at"],
            "created_at",
        );
        let dir = super::rows::sort_dir(sort_desc);
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let sql = format!(
            "SELECT * FROM products WHERE deleted_at IS NULL \
             AND ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR name ILIKE $2 OR sku ILIKE $2 OR barcode ILIKE $2) \
             ORDER BY {sort_col} {dir} LIMIT $3 OFFSET $4"
        );

        let rows = sqlx::query_as::<_, ProductRow>(&sql)
            .bind(org_uuid)
            .bind(search_pattern.as_deref())
            .bind(limit as i64)
            .bind(start as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Product::from).collect())
    }

    /// Counts products with optional filters.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_products(
        &self,
        org_id: Option<&str>,
        search: Option<&str>,
    ) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM products WHERE deleted_at IS NULL \
             AND ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR name ILIKE $2 OR sku ILIKE $2)",
        )
        .bind(org_uuid)
        .bind(search_pattern.as_deref())
        .fetch_one(&self.pool)
        .await?;

        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }

    /// Creates a new product.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_product(
        &self,
        name: &str,
        sku: &str,
        base_price: i64,
        description: Option<&str>,
        barcode: Option<&str>,
        cost_price: Option<i64>,
        category_id: Option<&str>,
        brand_id: Option<&str>,
        image_url: Option<&str>,
        weight: Option<f64>,
        dimensions: Option<&serde_json::Value>,
        track_inventory: bool,
        min_stock_level: Option<i64>,
        is_featured: bool,
        tags: Option<&[String]>,
        created_by: &str,
        organization_id: &str,
    ) -> Result<Product, PgError> {
        let org_uuid = parse_uuid(organization_id)?;
        let creator_uuid = parse_uuid(created_by)?;
        let cat_uuid = category_id.map(parse_uuid).transpose()?;
        let brand_uuid = brand_id.map(parse_uuid).transpose()?;
        let min_stock = min_stock_level.unwrap_or(0);

        let row = sqlx::query_as::<_, ProductRow>(
            "INSERT INTO products \
             (name, sku, base_price, description, barcode, cost_price, category_id, brand_id, \
              image_url, weight, dimensions, track_inventory, min_stock_level, is_featured, tags, \
              created_by, organization_id) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17) RETURNING *",
        )
        .bind(name)
        .bind(sku)
        .bind(base_price)
        .bind(description)
        .bind(barcode)
        .bind(cost_price)
        .bind(cat_uuid)
        .bind(brand_uuid)
        .bind(image_url)
        .bind(weight)
        .bind(dimensions)
        .bind(track_inventory)
        .bind(min_stock)
        .bind(is_featured)
        .bind(tags)
        .bind(creator_uuid)
        .bind(org_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(Product::from(row))
    }

    /// Updates a product from a JSON patch.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_product(
        &self,
        id: &str,
        updates: &serde_json::Value,
    ) -> Result<Product, PgError> {
        let uid = parse_uuid(id)?;
        let map = updates
            .as_object()
            .ok_or_else(|| PgError::Query("expected JSON object".to_string()))?;

        if map.is_empty() {
            return self
                .find_product(id)
                .await?
                .ok_or_else(|| PgError::NotFound("Product".to_string()));
        }

        let cat_uuid = map
            .get("category_id")
            .and_then(|v| v.as_str())
            .map(parse_uuid)
            .transpose()?;
        let brand_uuid = map
            .get("brand_id")
            .and_then(|v| v.as_str())
            .map(parse_uuid)
            .transpose()?;

        let row = sqlx::query_as::<_, ProductRow>(
            "UPDATE products SET \
             name = COALESCE($2, name), \
             description = COALESCE($3, description), \
             sku = COALESCE($4, sku), \
             barcode = COALESCE($5, barcode), \
             base_price = COALESCE($6, base_price), \
             cost_price = COALESCE($7, cost_price), \
             category_id = COALESCE($8, category_id), \
             brand_id = COALESCE($9, brand_id), \
             image_url = COALESCE($10, image_url), \
             is_featured = COALESCE($11, is_featured), \
             track_inventory = COALESCE($12, track_inventory) \
             WHERE id = $1 AND deleted_at IS NULL RETURNING *",
        )
        .bind(uid)
        .bind(map.get("name").and_then(|v| v.as_str()))
        .bind(map.get("description").and_then(|v| v.as_str()))
        .bind(map.get("sku").and_then(|v| v.as_str()))
        .bind(map.get("barcode").and_then(|v| v.as_str()))
        .bind(map.get("base_price").and_then(|v| v.as_i64()))
        .bind(map.get("cost_price").and_then(|v| v.as_i64()))
        .bind(cat_uuid)
        .bind(brand_uuid)
        .bind(map.get("image_url").and_then(|v| v.as_str()))
        .bind(map.get("is_featured").and_then(|v| v.as_bool()))
        .bind(map.get("track_inventory").and_then(|v| v.as_bool()))
        .fetch_one(&self.pool)
        .await?;

        Ok(Product::from(row))
    }

    /// Soft-deletes a product by setting deleted_at.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn delete_product(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("UPDATE products SET deleted_at = NOW() WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
