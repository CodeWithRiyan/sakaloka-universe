//! Inventory and stock movement query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, uuid_to_string, PgError};
use super::PgClient;
use sakaloka_core::models::inventory::{InventoryItem, StockMovement};

/// Internal row type matching the `inventory_items` table.
#[derive(sqlx::FromRow)]
struct InventoryRow {
    id: uuid::Uuid,
    product_id: uuid::Uuid,
    organization_id: uuid::Uuid,
    sku: Option<String>,
    location: Option<String>,
    quantity_on_hand: i32,
    quantity_reserved: i32,
    quantity_available: i32,
    min_stock_level: i32,
    max_stock_level: Option<i32>,
    reorder_point: Option<i32>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<InventoryRow> for InventoryItem {
    fn from(r: InventoryRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            product_id: uuid_to_string(r.product_id),
            organization_id: uuid_to_string(r.organization_id),
            sku: r.sku,
            location: r.location,
            quantity_on_hand: r.quantity_on_hand,
            quantity_reserved: r.quantity_reserved,
            quantity_available: r.quantity_available,
            min_stock_level: r.min_stock_level,
            max_stock_level: r.max_stock_level,
            reorder_point: r.reorder_point,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

/// Internal row type matching the `stock_movements` table.
#[derive(sqlx::FromRow)]
struct MovementRow {
    id: uuid::Uuid,
    inventory_item_id: uuid::Uuid,
    movement_type: String,
    quantity: i32,
    reference_type: Option<String>,
    reference_id: Option<String>,
    notes: Option<String>,
    created_by: Option<uuid::Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<MovementRow> for StockMovement {
    fn from(r: MovementRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            inventory_item_id: uuid_to_string(r.inventory_item_id),
            movement_type: r.movement_type,
            quantity: r.quantity,
            reference_type: r.reference_type,
            reference_id: r.reference_id,
            notes: r.notes,
            created_by: r.created_by.map(uuid_to_string),
            created_at: r.created_at,
        }
    }
}

impl PgClient {
    /// Finds an inventory item by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_inventory_item(&self, id: &str) -> Result<Option<InventoryItem>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, InventoryRow>("SELECT * FROM inventory_items WHERE id = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(InventoryItem::from))
    }

    /// Finds an inventory item by product and organization.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_inventory_by_product_org(
        &self,
        product_id: &str,
        organization_id: &str,
    ) -> Result<Option<InventoryItem>, PgError> {
        let pid = parse_uuid(product_id)?;
        let oid = parse_uuid(organization_id)?;
        let row = sqlx::query_as::<_, InventoryRow>(
            "SELECT * FROM inventory_items \
             WHERE product_id = $1 AND organization_id = $2 LIMIT 1",
        )
        .bind(pid)
        .bind(oid)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(InventoryItem::from))
    }

    /// Lists inventory items with pagination and sorting.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_inventory(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<InventoryItem>, PgError> {
        let sort_col = super::rows::safe_sort_column(
            sort_by,
            &[
                "quantity_on_hand",
                "quantity_available",
                "sku",
                "created_at",
            ],
            "created_at",
        );
        let dir = super::rows::sort_dir(sort_desc);
        let org_uuid = org_id.map(parse_uuid).transpose()?;

        let sql = format!(
            "SELECT * FROM inventory_items \
             WHERE ($1::uuid IS NULL OR organization_id = $1) \
             ORDER BY {sort_col} {dir} LIMIT $2 OFFSET $3"
        );

        let rows = sqlx::query_as::<_, InventoryRow>(&sql)
            .bind(org_uuid)
            .bind(limit as i64)
            .bind(start as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(InventoryItem::from).collect())
    }

    /// Counts all inventory items.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_inventory(&self, org_id: Option<&str>) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM inventory_items \
             WHERE ($1::uuid IS NULL OR organization_id = $1)",
        )
        .bind(org_uuid)
        .fetch_one(&self.pool)
        .await?;

        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }

    /// Lists items where available stock is at or below minimum level.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_low_stock(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
    ) -> Result<Vec<InventoryItem>, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;

        let rows = sqlx::query_as::<_, InventoryRow>(
            "SELECT * FROM inventory_items \
             WHERE quantity_available <= min_stock_level \
             AND ($1::uuid IS NULL OR organization_id = $1) \
             ORDER BY quantity_available ASC LIMIT $2 OFFSET $3",
        )
        .bind(org_uuid)
        .bind(limit as i64)
        .bind(start as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(InventoryItem::from).collect())
    }

    /// Counts low-stock items.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_low_stock(&self, org_id: Option<&str>) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM inventory_items \
             WHERE quantity_available <= min_stock_level \
             AND ($1::uuid IS NULL OR organization_id = $1)",
        )
        .bind(org_uuid)
        .fetch_one(&self.pool)
        .await?;

        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }

    /// Creates a new inventory item for a product.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    pub async fn create_inventory_item(
        &self,
        product_id: &str,
        organization_id: &str,
    ) -> Result<InventoryItem, PgError> {
        let pid = parse_uuid(product_id)?;
        let oid = parse_uuid(organization_id)?;

        let row = sqlx::query_as::<_, InventoryRow>(
            "INSERT INTO inventory_items \
             (product_id, organization_id, quantity_on_hand, quantity_reserved, \
              quantity_available, min_stock_level) \
             VALUES ($1,$2,0,0,0,0) RETURNING *",
        )
        .bind(pid)
        .bind(oid)
        .fetch_one(&self.pool)
        .await?;

        Ok(InventoryItem::from(row))
    }

    /// Updates stock quantities for an inventory item.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    pub async fn update_inventory_stock(
        &self,
        id: &str,
        quantity_on_hand: i32,
        quantity_available: i32,
    ) -> Result<InventoryItem, PgError> {
        let uid = parse_uuid(id)?;

        let row = sqlx::query_as::<_, InventoryRow>(
            "UPDATE inventory_items SET \
             quantity_on_hand = $2, quantity_available = $3 \
             WHERE id = $1 RETURNING *",
        )
        .bind(uid)
        .bind(quantity_on_hand)
        .bind(quantity_available)
        .fetch_one(&self.pool)
        .await?;

        Ok(InventoryItem::from(row))
    }

    /// Creates a stock movement record.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_stock_movement(
        &self,
        inventory_item_id: &str,
        movement_type: &str,
        quantity: i32,
        reference_type: Option<&str>,
        reference_id: Option<&str>,
        notes: Option<&str>,
        created_by: Option<&str>,
    ) -> Result<StockMovement, PgError> {
        let inv_uuid = parse_uuid(inventory_item_id)?;
        let creator_uuid = created_by.map(parse_uuid).transpose()?;

        let row = sqlx::query_as::<_, MovementRow>(
            "INSERT INTO stock_movements \
             (inventory_item_id, movement_type, quantity, reference_type, reference_id, notes, created_by) \
             VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *",
        )
        .bind(inv_uuid)
        .bind(movement_type)
        .bind(quantity)
        .bind(reference_type)
        .bind(reference_id)
        .bind(notes)
        .bind(creator_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(StockMovement::from(row))
    }

    /// Lists stock movements for an inventory item.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_stock_movements(
        &self,
        inventory_item_id: &str,
        limit: u64,
        start: u64,
    ) -> Result<Vec<StockMovement>, PgError> {
        let uid = parse_uuid(inventory_item_id)?;
        let rows = sqlx::query_as::<_, MovementRow>(
            "SELECT * FROM stock_movements \
             WHERE inventory_item_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(uid)
        .bind(limit as i64)
        .bind(start as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(StockMovement::from).collect())
    }

    /// Counts stock movements for an inventory item.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_stock_movements(&self, inventory_item_id: &str) -> Result<u64, PgError> {
        let uid = parse_uuid(inventory_item_id)?;
        let row: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM stock_movements WHERE inventory_item_id = $1")
                .bind(uid)
                .fetch_one(&self.pool)
                .await?;

        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }
}
