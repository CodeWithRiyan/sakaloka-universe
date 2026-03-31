//! Order query methods for the PostgreSQL backend.

use super::rows::{parse_uuid, uuid_to_string, PgError};
use super::PgClient;
use sakaloka_core::models::order::{Order, OrderItem};

/// Internal row type matching the `orders` table.
#[derive(sqlx::FromRow)]
struct OrderRow {
    id: uuid::Uuid,
    order_number: String,
    customer_id: Option<uuid::Uuid>,
    organization_id: uuid::Uuid,
    status: String,
    #[sqlx(rename = "type")]
    order_type: String,
    subtotal: i64,
    tax_amount: i64,
    discount_amount: i64,
    total_amount: i64,
    payment_method: Option<String>,
    payment_status: String,
    paid_amount: i64,
    notes: Option<String>,
    table_number: Option<String>,
    customer_name: Option<String>,
    created_by: Option<uuid::Uuid>,
    updated_by: Option<uuid::Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<OrderRow> for Order {
    fn from(r: OrderRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            order_number: r.order_number,
            customer_id: r.customer_id.map(uuid_to_string),
            organization_id: uuid_to_string(r.organization_id),
            status: r.status,
            order_type: r.order_type,
            subtotal: r.subtotal,
            tax_amount: r.tax_amount,
            discount_amount: r.discount_amount,
            total_amount: r.total_amount,
            payment_method: r.payment_method,
            payment_status: r.payment_status,
            paid_amount: r.paid_amount,
            notes: r.notes,
            table_number: r.table_number,
            customer_name: r.customer_name,
            created_by: r.created_by.map(uuid_to_string),
            updated_by: r.updated_by.map(uuid_to_string),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

/// Internal row type matching the `order_items` table.
#[derive(sqlx::FromRow)]
struct OrderItemRow {
    id: uuid::Uuid,
    order_id: uuid::Uuid,
    product_id: uuid::Uuid,
    item_name: String,
    quantity: i32,
    unit_price: i64,
    discount_amount: i64,
    total_price: i64,
    notes: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<OrderItemRow> for OrderItem {
    fn from(r: OrderItemRow) -> Self {
        Self {
            id: uuid_to_string(r.id),
            order_id: uuid_to_string(r.order_id),
            product_id: uuid_to_string(r.product_id),
            item_name: r.item_name,
            quantity: r.quantity,
            unit_price: r.unit_price,
            discount_amount: r.discount_amount,
            total_price: r.total_price,
            notes: r.notes,
            created_at: r.created_at,
        }
    }
}

impl PgClient {
    /// Finds an order by ID.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn find_order(&self, id: &str) -> Result<Option<Order>, PgError> {
        let uid = parse_uuid(id)?;
        let row = sqlx::query_as::<_, OrderRow>("SELECT * FROM orders WHERE id = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Order::from))
    }

    /// Lists orders with pagination, search, sorting, and status filters.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn list_orders(
        &self,
        org_id: Option<&str>,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
        status_filter: Option<&[&str]>,
        status_exclude: Option<&[&str]>,
    ) -> Result<Vec<Order>, PgError> {
        let sort_col = super::rows::safe_sort_column(
            sort_by,
            &[
                "order_number",
                "status",
                "total_amount",
                "payment_status",
                "created_at",
            ],
            "created_at",
        );
        let dir = super::rows::sort_dir(sort_desc);
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));
        let sf: Option<Vec<String>> =
            status_filter.map(|f| f.iter().map(|s| (*s).to_string()).collect());
        let se: Option<Vec<String>> =
            status_exclude.map(|f| f.iter().map(|s| (*s).to_string()).collect());

        let sql = format!(
            "SELECT * FROM orders \
             WHERE ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR order_number ILIKE $2 OR customer_name ILIKE $2) \
             AND ($3::text[] IS NULL OR status = ANY($3)) \
             AND ($4::text[] IS NULL OR status != ALL($4)) \
             ORDER BY {sort_col} {dir} LIMIT $5 OFFSET $6"
        );

        let rows = sqlx::query_as::<_, OrderRow>(&sql)
            .bind(org_uuid)
            .bind(search_pattern.as_deref())
            .bind(sf.as_deref())
            .bind(se.as_deref())
            .bind(limit as i64)
            .bind(start as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Order::from).collect())
    }

    /// Counts orders with optional filters.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn count_orders(
        &self,
        org_id: Option<&str>,
        search: Option<&str>,
        status_filter: Option<&[&str]>,
        status_exclude: Option<&[&str]>,
    ) -> Result<u64, PgError> {
        let org_uuid = org_id.map(parse_uuid).transpose()?;
        let search_pattern = search.map(|s| format!("%{s}%"));
        let sf: Option<Vec<String>> =
            status_filter.map(|f| f.iter().map(|s| (*s).to_string()).collect());
        let se: Option<Vec<String>> =
            status_exclude.map(|f| f.iter().map(|s| (*s).to_string()).collect());

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM orders \
             WHERE ($1::uuid IS NULL OR organization_id = $1) \
             AND ($2::text IS NULL OR order_number ILIKE $2 OR customer_name ILIKE $2) \
             AND ($3::text[] IS NULL OR status = ANY($3)) \
             AND ($4::text[] IS NULL OR status != ALL($4))",
        )
        .bind(org_uuid)
        .bind(search_pattern.as_deref())
        .bind(sf.as_deref())
        .bind(se.as_deref())
        .fetch_one(&self.pool)
        .await?;

        #[allow(clippy::cast_sign_loss)]
        Ok(row.0 as u64)
    }

    /// Creates a new order.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_order(
        &self,
        order_number: &str,
        organization_id: &str,
        order_type: &str,
        subtotal: i64,
        tax_amount: i64,
        total_amount: i64,
        payment_method: Option<&str>,
        notes: Option<&str>,
        table_number: Option<&str>,
        customer_name: Option<&str>,
        created_by: &str,
    ) -> Result<Order, PgError> {
        let org_uuid = parse_uuid(organization_id)?;
        let creator_uuid = parse_uuid(created_by)?;

        let row = sqlx::query_as::<_, OrderRow>(
            "INSERT INTO orders \
             (order_number, organization_id, type, status, subtotal, tax_amount, \
              discount_amount, total_amount, payment_method, payment_status, paid_amount, \
              notes, table_number, customer_name, created_by, updated_by) \
             VALUES ($1,$2,$3,'pending',$4,$5,0,$6,$7,'unpaid',0,$8,$9,$10,$11,$11) \
             RETURNING *",
        )
        .bind(order_number)
        .bind(org_uuid)
        .bind(order_type)
        .bind(subtotal)
        .bind(tax_amount)
        .bind(total_amount)
        .bind(payment_method)
        .bind(notes)
        .bind(table_number)
        .bind(customer_name)
        .bind(creator_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(Order::from(row))
    }

    /// Updates an order with optional fields.
    ///
    /// # Errors
    /// Returns [`PgError`] if the update fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_order(
        &self,
        id: &str,
        status: Option<&str>,
        payment_method: Option<&str>,
        payment_status: Option<&str>,
        paid_amount: Option<i64>,
        notes: Option<&str>,
        table_number: Option<&str>,
        customer_name: Option<&str>,
        updated_by: &str,
    ) -> Result<Order, PgError> {
        let uid = parse_uuid(id)?;
        let updater_uuid = parse_uuid(updated_by)?;

        let row = sqlx::query_as::<_, OrderRow>(
            "UPDATE orders SET \
             status = COALESCE($2, status), \
             payment_method = COALESCE($3, payment_method), \
             payment_status = COALESCE($4, payment_status), \
             paid_amount = COALESCE($5, paid_amount), \
             notes = COALESCE($6, notes), \
             table_number = COALESCE($7, table_number), \
             customer_name = COALESCE($8, customer_name), \
             updated_by = $9 \
             WHERE id = $1 RETURNING *",
        )
        .bind(uid)
        .bind(status)
        .bind(payment_method)
        .bind(payment_status)
        .bind(paid_amount)
        .bind(notes)
        .bind(table_number)
        .bind(customer_name)
        .bind(updater_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(Order::from(row))
    }

    /// Creates a single order item.
    ///
    /// # Errors
    /// Returns [`PgError`] if the insert fails.
    pub async fn create_order_item(
        &self,
        order_id: &str,
        product_id: &str,
        item_name: &str,
        quantity: i32,
        unit_price: i64,
        total_price: i64,
    ) -> Result<OrderItem, PgError> {
        let order_uuid = parse_uuid(order_id)?;
        let product_uuid = parse_uuid(product_id)?;

        let row = sqlx::query_as::<_, OrderItemRow>(
            "INSERT INTO order_items \
             (order_id, product_id, item_name, quantity, unit_price, discount_amount, total_price) \
             VALUES ($1,$2,$3,$4,$5,0,$6) RETURNING *",
        )
        .bind(order_uuid)
        .bind(product_uuid)
        .bind(item_name)
        .bind(quantity)
        .bind(unit_price)
        .bind(total_price)
        .fetch_one(&self.pool)
        .await?;

        Ok(OrderItem::from(row))
    }

    /// Batch-inserts order items.
    ///
    /// # Errors
    /// Returns [`PgError`] if any insert fails.
    pub async fn create_order_items_batch(
        &self,
        order_id: &str,
        items: &[(String, String, i32, i64, i64)],
    ) -> Result<(), PgError> {
        if items.is_empty() {
            return Ok(());
        }

        let order_uuid = parse_uuid(order_id)?;

        let mut qb = sqlx::QueryBuilder::new(
            "INSERT INTO order_items (order_id, product_id, item_name, quantity, unit_price, discount_amount, total_price) ",
        );

        qb.push_values(items, |mut b, (product_id, item_name, qty, up, tp)| {
            let product_uuid = uuid::Uuid::parse_str(product_id).unwrap_or_default();
            b.push_bind(order_uuid)
                .push_bind(product_uuid)
                .push_bind(item_name)
                .push_bind(qty)
                .push_bind(up)
                .push_bind(0_i64)
                .push_bind(tp);
        });

        qb.build().execute(&self.pool).await?;
        Ok(())
    }

    /// Lists order items for a given order.
    ///
    /// # Errors
    /// Returns [`PgError`] if the query fails.
    pub async fn list_order_items(&self, order_id: &str) -> Result<Vec<OrderItem>, PgError> {
        let uid = parse_uuid(order_id)?;
        let rows = sqlx::query_as::<_, OrderItemRow>(
            "SELECT * FROM order_items WHERE order_id = $1 ORDER BY created_at",
        )
        .bind(uid)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(OrderItem::from).collect())
    }
}
