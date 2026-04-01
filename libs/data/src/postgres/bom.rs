//! PostgreSQL queries for Bill of Materials (BOM).

use crate::postgres::rows::{parse_uuid, PgError};
use crate::postgres::PgClient;
use sakaloka_core::models::bom::{
    Bom, BomItem, BomStatus, Consumption, ProductionRun, ProductionRunStatus,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

/// Filters for listing BOMs.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListBomFilters {
    /// Filter by organization ID.
    pub organization_id: Option<String>,
    /// Filter by product ID.
    pub product_id: Option<String>,
    /// Filter by status (draft, active, archived).
    pub status: Option<String>,
    /// Search by name.
    pub search: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct BomRow {
    id: uuid::Uuid,
    name: String,
    product_id: uuid::Uuid,
    organization_id: uuid::Uuid,
    version: String,
    status: String,
    effective_from: Option<chrono::DateTime<chrono::Utc>>,
    effective_to: Option<chrono::DateTime<chrono::Utc>>,
    total_cost: Option<i64>,
    notes: Option<String>,
    created_by: Option<uuid::Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<BomRow> for Bom {
    fn from(r: BomRow) -> Self {
        Self {
            id: format!("bom:{}", r.id),
            name: r.name,
            product_id: format!("product:{}", r.product_id),
            organization_id: format!("org:{}", r.organization_id),
            version: r.version,
            status: r.status.parse().unwrap_or(BomStatus::Draft),
            effective_from: r.effective_from,
            effective_to: r.effective_to,
            total_cost: r.total_cost,
            notes: r.notes,
            created_by: r.created_by.map(|u| format!("user:{}", u)),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BomItemRow {
    id: uuid::Uuid,
    bom_id: uuid::Uuid,
    component_product_id: uuid::Uuid,
    quantity: f64,
    unit: String,
    waste_percent: f64,
    yield_percent: f64,
    unit_cost: Option<i64>,
    line_cost: Option<i64>,
    notes: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<BomItemRow> for BomItem {
    fn from(r: BomItemRow) -> Self {
        Self {
            id: format!("bom_item:{}", r.id),
            bom_id: format!("bom:{}", r.bom_id),
            component_product_id: format!("product:{}", r.component_product_id),
            quantity: r.quantity,
            unit: r.unit,
            waste_percent: r.waste_percent,
            yield_percent: r.yield_percent,
            unit_cost: r.unit_cost,
            line_cost: r.line_cost,
            notes: r.notes,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ProductionRunRow {
    id: uuid::Uuid,
    bom_id: uuid::Uuid,
    organization_id: uuid::Uuid,
    quantity_produced: f64,
    estimated_cost: Option<i64>,
    actual_cost: Option<i64>,
    status: String,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
    created_by: Option<uuid::Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<ProductionRunRow> for ProductionRun {
    fn from(r: ProductionRunRow) -> Self {
        Self {
            id: format!("production_run:{}", r.id),
            bom_id: format!("bom:{}", r.bom_id),
            organization_id: format!("org:{}", r.organization_id),
            quantity_produced: r.quantity_produced,
            estimated_cost: r.estimated_cost,
            actual_cost: r.actual_cost,
            status: r.status.parse().unwrap_or(ProductionRunStatus::Planned),
            started_at: r.started_at,
            completed_at: r.completed_at,
            created_by: r.created_by.map(|u| format!("user:{}", u)),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ConsumptionRow {
    id: uuid::Uuid,
    production_run_id: uuid::Uuid,
    component_product_id: uuid::Uuid,
    planned_quantity: f64,
    actual_quantity: f64,
    unit: String,
    waste_quantity: f64,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<ConsumptionRow> for Consumption {
    fn from(r: ConsumptionRow) -> Self {
        Self {
            id: format!("consumption:{}", r.id),
            production_run_id: format!("production_run:{}", r.production_run_id),
            component_product_id: format!("product:{}", r.component_product_id),
            planned_quantity: r.planned_quantity,
            actual_quantity: r.actual_quantity,
            unit: r.unit,
            waste_quantity: r.waste_quantity,
            created_at: r.created_at,
        }
    }
}

impl PgClient {
    /// Find a BOM by ID.
    pub async fn find_bom(&self, id: &str) -> Result<Option<Bom>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, BomRow>("SELECT * FROM boms WHERE id = $1 LIMIT 1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Bom::from))
    }

    /// Find the active BOM for a product.
    pub async fn find_bom_by_product(&self, product_id: &str) -> Result<Option<Bom>, PgError> {
        let uid = parse_uuid(product_id)?;
        let row = sqlx::query_as::<_, BomRow>(
            "SELECT * FROM boms WHERE product_id = $1 AND status = 'active' LIMIT 1",
        )
        .bind(uid)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(Bom::from))
    }

    /// List BOMs with pagination and filters.
    pub async fn list_boms(
        &self,
        filters: &ListBomFilters,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<Bom>, i64), PgError> {
        let org_uuid = filters.organization_id.as_ref().and_then(|s| {
            s.strip_prefix("org:")
                .and_then(|s| uuid::Uuid::parse_str(s).ok())
        });

        let product_uuid = filters.product_id.as_ref().and_then(|s| {
            s.strip_prefix("product:")
                .and_then(|s| uuid::Uuid::parse_str(s).ok())
        });

        let count_sql = if filters.search.is_some() {
            "SELECT COUNT(*) as cnt FROM boms WHERE organization_id = $1 AND name ILIKE $2"
        } else if filters.product_id.is_some() {
            "SELECT COUNT(*) as cnt FROM boms WHERE organization_id = $1 AND product_id = $2"
        } else if filters.status.is_some() {
            "SELECT COUNT(*) as cnt FROM boms WHERE organization_id = $1 AND status = $2"
        } else {
            "SELECT COUNT(*) as cnt FROM boms WHERE organization_id = $1"
        };

        let count_row = if let Some(ref search) = filters.search {
            if let Some(ref org) = org_uuid {
                sqlx::query(count_sql)
                    .bind(org)
                    .bind(format!("%{}%", search))
                    .fetch_one(&self.pool)
                    .await?
            } else {
                return Ok((vec![], 0));
            }
        } else if let Some(ref prod) = product_uuid {
            if let Some(ref org) = org_uuid {
                sqlx::query(count_sql)
                    .bind(org)
                    .bind(prod)
                    .fetch_one(&self.pool)
                    .await?
            } else {
                return Ok((vec![], 0));
            }
        } else if let Some(ref status) = filters.status {
            if let Some(ref org) = org_uuid {
                sqlx::query(count_sql)
                    .bind(org)
                    .bind(status)
                    .fetch_one(&self.pool)
                    .await?
            } else {
                return Ok((vec![], 0));
            }
        } else if let Some(ref org) = org_uuid {
            sqlx::query(count_sql)
                .bind(org)
                .fetch_one(&self.pool)
                .await?
        } else {
            return Ok((vec![], 0));
        };

        let total: i64 = count_row.get("cnt");

        let offset = (page - 1) * per_page;

        let Some(ref org) = org_uuid else {
            return Ok((vec![], total));
        };

        let rows = if let Some(ref search) = filters.search {
            sqlx::query_as::<_, BomRow>(
                "SELECT * FROM boms WHERE organization_id = $1 AND name ILIKE $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            )
            .bind(org)
            .bind(format!("%{}%", search))
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else if let Some(ref prod) = product_uuid {
            sqlx::query_as::<_, BomRow>(
                "SELECT * FROM boms WHERE organization_id = $1 AND product_id = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            )
            .bind(org)
            .bind(prod)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else if let Some(ref status) = filters.status {
            sqlx::query_as::<_, BomRow>(
                "SELECT * FROM boms WHERE organization_id = $1 AND status = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
            )
            .bind(org)
            .bind(status)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, BomRow>(
                "SELECT * FROM boms WHERE organization_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(org)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        Ok((rows.into_iter().map(Bom::from).collect(), total))
    }

    /// Create a new BOM.
    pub async fn create_bom(&self, bom: &Bom) -> Result<Bom, PgError> {
        let product_id = parse_uuid(&bom.product_id)?;
        let org_id = parse_uuid(&bom.organization_id)?;
        let created_by = bom.created_by.as_ref().and_then(|s| {
            s.strip_prefix("user:")
                .and_then(|s| uuid::Uuid::parse_str(s).ok())
        });

        let row = sqlx::query_as::<_, BomRow>(
            r#"INSERT INTO boms (name, product_id, organization_id, version, status, 
               effective_from, effective_to, total_cost, notes, created_by)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
               RETURNING *"#,
        )
        .bind(&bom.name)
        .bind(product_id)
        .bind(org_id)
        .bind(&bom.version)
        .bind(bom.status.to_string())
        .bind(bom.effective_from)
        .bind(bom.effective_to)
        .bind(bom.total_cost)
        .bind(&bom.notes)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(Bom::from(row))
    }

    /// Update an existing BOM.
    pub async fn update_bom(&self, id: &str, bom: &Bom) -> Result<Bom, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, BomRow>(
            r#"UPDATE boms SET name = $1, version = $2, status = $3, 
               effective_from = $4, effective_to = $5, total_cost = $6, notes = $7
               WHERE id = $8 RETURNING *"#,
        )
        .bind(&bom.name)
        .bind(&bom.version)
        .bind(bom.status.to_string())
        .bind(bom.effective_from)
        .bind(bom.effective_to)
        .bind(bom.total_cost)
        .bind(&bom.notes)
        .bind(uid)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => PgError::NotFound("Row not found".to_string()),
            _ => PgError::Query(e.to_string()),
        })?;

        Ok(Bom::from(row))
    }

    /// Delete a BOM.
    pub async fn delete_bom(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("DELETE FROM boms WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// List items in a BOM.
    pub async fn find_bom_items(&self, bom_id: &str) -> Result<Vec<BomItem>, PgError> {
        let uid = parse_uuid(bom_id)?;
        let rows = sqlx::query_as::<_, BomItemRow>(
            "SELECT * FROM bom_items WHERE bom_id = $1 ORDER BY created_at",
        )
        .bind(uid)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(BomItem::from).collect())
    }

    /// Create a BOM item.
    pub async fn create_bom_item(&self, item: &BomItem) -> Result<BomItem, PgError> {
        let bom_id = parse_uuid(&item.bom_id)?;
        let prod_id = parse_uuid(&item.component_product_id)?;

        let row = sqlx::query_as::<_, BomItemRow>(
            r#"INSERT INTO bom_items (bom_id, component_product_id, quantity, unit,
               waste_percent, yield_percent, unit_cost, line_cost, notes)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               RETURNING *"#,
        )
        .bind(bom_id)
        .bind(prod_id)
        .bind(item.quantity)
        .bind(&item.unit)
        .bind(item.waste_percent)
        .bind(item.yield_percent)
        .bind(item.unit_cost)
        .bind(item.line_cost)
        .bind(&item.notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(BomItem::from(row))
    }

    /// Update a BOM item.
    pub async fn update_bom_item(&self, id: &str, item: &BomItem) -> Result<BomItem, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, BomItemRow>(
            r#"UPDATE bom_items SET quantity = $1, unit = $2, waste_percent = $3,
               yield_percent = $4, unit_cost = $5, line_cost = $6, notes = $7
               WHERE id = $8 RETURNING *"#,
        )
        .bind(item.quantity)
        .bind(&item.unit)
        .bind(item.waste_percent)
        .bind(item.yield_percent)
        .bind(item.unit_cost)
        .bind(item.line_cost)
        .bind(&item.notes)
        .bind(uid)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => PgError::NotFound("Row not found".to_string()),
            _ => PgError::Query(e.to_string()),
        })?;

        Ok(BomItem::from(row))
    }

    /// Delete a BOM item.
    pub async fn delete_bom_item(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("DELETE FROM bom_items WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Find a BOM item by ID.
    pub async fn find_bom_item_by_id(&self, id: &str) -> Result<Option<BomItem>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, BomItemRow>("SELECT * FROM bom_items WHERE id = $1 LIMIT 1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(BomItem::from))
    }

    /// List production runs for a BOM.
    pub async fn list_production_runs(
        &self,
        bom_id: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<ProductionRun>, i64), PgError> {
        let uid = parse_uuid(bom_id)?;

        let count_row =
            sqlx::query("SELECT COUNT(*) as cnt FROM bom_production_runs WHERE bom_id = $1")
                .bind(uid)
                .fetch_one(&self.pool)
                .await?;
        let total: i64 = count_row.get("cnt");

        let offset = (page - 1) * per_page;
        let rows = sqlx::query_as::<_, ProductionRunRow>(
            "SELECT * FROM bom_production_runs WHERE bom_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(uid)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((rows.into_iter().map(ProductionRun::from).collect(), total))
    }

    /// Find a production run by ID.
    pub async fn find_production_run(&self, id: &str) -> Result<Option<ProductionRun>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, ProductionRunRow>(
            "SELECT * FROM bom_production_runs WHERE id = $1 LIMIT 1",
        )
        .bind(uid)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(ProductionRun::from))
    }

    /// Create a production run.
    pub async fn create_production_run(
        &self,
        run: &ProductionRun,
    ) -> Result<ProductionRun, PgError> {
        let bom_id = parse_uuid(&run.bom_id)?;
        let org_id = parse_uuid(&run.organization_id)?;
        let created_by = run.created_by.as_ref().and_then(|s| {
            s.strip_prefix("user:")
                .and_then(|s| uuid::Uuid::parse_str(s).ok())
        });

        let row = sqlx::query_as::<_, ProductionRunRow>(
            r#"INSERT INTO bom_production_runs (bom_id, organization_id, quantity_produced,
               estimated_cost, actual_cost, status, started_at, completed_at, created_by)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               RETURNING *"#,
        )
        .bind(bom_id)
        .bind(org_id)
        .bind(run.quantity_produced)
        .bind(run.estimated_cost)
        .bind(run.actual_cost)
        .bind(run.status.to_string())
        .bind(run.started_at)
        .bind(run.completed_at)
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(ProductionRun::from(row))
    }

    /// Update a production run.
    pub async fn update_production_run(
        &self,
        id: &str,
        run: &ProductionRun,
    ) -> Result<ProductionRun, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, ProductionRunRow>(
            r#"UPDATE bom_production_runs SET quantity_produced = $1, estimated_cost = $2,
               actual_cost = $3, status = $4, started_at = $5, completed_at = $6
               WHERE id = $7 RETURNING *"#,
        )
        .bind(run.quantity_produced)
        .bind(run.estimated_cost)
        .bind(run.actual_cost)
        .bind(run.status.to_string())
        .bind(run.started_at)
        .bind(run.completed_at)
        .bind(uid)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => PgError::NotFound("Row not found".to_string()),
            _ => PgError::Query(e.to_string()),
        })?;

        Ok(ProductionRun::from(row))
    }

    /// Delete a production run.
    pub async fn delete_production_run(&self, id: &str) -> Result<(), PgError> {
        let uid = parse_uuid(id)?;
        sqlx::query("DELETE FROM bom_production_runs WHERE id = $1")
            .bind(uid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// List consumptions for a production run.
    pub async fn list_consumptions(&self, run_id: &str) -> Result<Vec<Consumption>, PgError> {
        let uid = parse_uuid(run_id)?;
        let rows = sqlx::query_as::<_, ConsumptionRow>(
            "SELECT * FROM bom_consumption WHERE production_run_id = $1 ORDER BY created_at",
        )
        .bind(uid)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(Consumption::from).collect())
    }

    /// Create a consumption record.
    pub async fn create_consumption(&self, cons: &Consumption) -> Result<Consumption, PgError> {
        let run_id = parse_uuid(&cons.production_run_id)?;
        let prod_id = parse_uuid(&cons.component_product_id)?;

        let row = sqlx::query_as::<_, ConsumptionRow>(
            r#"INSERT INTO bom_consumption (production_run_id, component_product_id,
               planned_quantity, actual_quantity, unit, waste_quantity)
               VALUES ($1, $2, $3, $4, $5, $6)
               RETURNING *"#,
        )
        .bind(run_id)
        .bind(prod_id)
        .bind(cons.planned_quantity)
        .bind(cons.actual_quantity)
        .bind(&cons.unit)
        .bind(cons.waste_quantity)
        .fetch_one(&self.pool)
        .await?;

        Ok(Consumption::from(row))
    }

    /// Calculate total cost of a BOM from its items.
    pub async fn calculate_bom_cost(&self, bom_id: &str) -> Result<i64, PgError> {
        let uid = parse_uuid(bom_id)?;
        let row = sqlx::query(
            "SELECT COALESCE(SUM(line_cost), 0) as total FROM bom_items WHERE bom_id = $1",
        )
        .bind(uid)
        .fetch_one(&self.pool)
        .await?;
        let total: i64 = row.get("total");
        Ok(total)
    }
}
