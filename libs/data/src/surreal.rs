//! SurrealDB (Jupiter) client abstraction.
//!
//! Always use the authenticated client from this module.
//! Never embed raw SurrealDB credentials in application code.

use surrealdb_types::SurrealValue;
use surrealdb_types_derive::SurrealValue as SurrealValueMacro;
use thiserror::Error;

/// Errors produced by the SurrealDB client layer.
#[derive(Debug, Error)]
pub enum SurrealError {
    /// Connection to SurrealDB failed.
    #[error("SurrealDB connection failed: {0}")]
    Connection(String),
    /// A query execution error.
    #[error("SurrealDB query error: {0}")]
    Query(String),
}

/// A client for interacting with the Jupiter SurrealDB instance.
#[derive(Clone)]
pub struct SurrealClient {
    db: surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
}

impl SurrealClient {
    /// Initialize a new SurrealDB connection to a specific endpoint.
    ///
    /// # Errors
    /// Returns a [`SurrealError`] if the connection fails or `USE` fails.
    pub async fn connect(url: &str) -> Result<Self, SurrealError> {
        let clean_url = url
            .trim_start_matches("http://")
            .trim_start_matches("ws://")
            .trim_start_matches("wss://");
        tracing::info!(url = %clean_url, "🧵 SurrealClient connecting with WebSocket engine");

        // Add a timeout to avoid hanging indefinitely
        let db = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>(clean_url),
        )
        .await
        .map_err(|_| SurrealError::Connection("Connection timeout at engine init".to_string()))?
        .map_err(|e| SurrealError::Connection(e.to_string()))?;

        tracing::info!("🧵 SurrealClient engine initialized");

        tracing::info!("🧵 SurrealClient switching to sakaloka/universe");
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            db.use_ns("sakaloka").use_db("universe"),
        )
        .await
        .map_err(|_| SurrealError::Connection("Timeout switching namespace/db".to_string()))?
        .map_err(|e| SurrealError::Connection(e.to_string()))?;

        tracing::info!("🧵 SurrealClient namespace/db set");

        Ok(Self { db })
    }

    /// Authenticate the global client connection using a Service JWT.
    ///
    /// The token must be signed correctly and have the proper audience (`sakaloka:jupiter`).
    pub async fn authenticate(&self, token: &str) -> Result<(), SurrealError> {
        self.db
            .authenticate(token)
            .await
            .map(|_| ())
            .map_err(|e| SurrealError::Connection(format!("Token auth failed: {}", e)))
    }

    /// Sign in using root/user credentials.
    pub async fn signin(&self, user: &str, pass: &str) -> Result<(), SurrealError> {
        self.db
            .signin(surrealdb::opt::auth::Root {
                username: user.to_string(),
                password: pass.to_string(),
            })
            .await
            .map(|_| ())
            .map_err(|e| SurrealError::Connection(format!("Signin failed: {}", e)))
    }

    /// Executes raw SurrealQL (used by the migration runner).
    ///
    /// # Errors
    /// Returns a [`SurrealError`] if the query fails.
    pub async fn execute_raw(&self, sql: &str) -> Result<(), SurrealError> {
        self.db
            .query(sql)
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;
        Ok(())
    }

    /// Finds a user by their username.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<sakaloka_core::models::user::User>, SurrealError> {
        let mut result = self
            .db
            .query("SELECT * FROM user WHERE username = $username LIMIT 1")
            .bind(("username", username.to_string()))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let user: Option<sakaloka_core::models::user::User> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(user)
    }

    /// Finds the user associated with a given session.
    pub async fn find_user_by_session(
        &self,
        session_id: &sakaloka_secure::newtypes::SessionId,
    ) -> Result<Option<sakaloka_core::models::user::User>, SurrealError> {
        let sid = surrealdb_types::RecordId::new(
            "session",
            surrealdb_types::RecordIdKey::String(
                session_id.to_record_id_string().replace("session:", ""),
            ),
        );

        let mut result = self
            .db
            .query("SELECT user_id.* AS user FROM session WHERE id = $sid LIMIT 1")
            .bind(("sid", sid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        #[derive(serde::Deserialize, SurrealValue)]
        struct SessionUser {
            user: Vec<sakaloka_core::models::user::User>,
        }

        let session: Option<SessionUser> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(session.and_then(|s| s.user.into_iter().next()))
    }

    /// Creates a new session and its initial refresh token.
    pub async fn create_session(
        &self,
        user_id: &sakaloka_secure::newtypes::UserId,
        session_id: &sakaloka_secure::newtypes::SessionId,
        token_hash: &str,
    ) -> Result<(), SurrealError> {
        let uid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(user_id.as_str().replace("user:", "")),
        );
        let sid = surrealdb_types::RecordId::new(
            "session",
            surrealdb_types::RecordIdKey::String(
                session_id.to_record_id_string().replace("session:", ""),
            ),
        );

        self.db.query("
            BEGIN TRANSACTION;
            LET $expires = time::now() + 7d;
            INSERT INTO session (id, user_id, expires_at) VALUES ($sid, $uid, $expires);
            INSERT INTO refresh_token (session_id, token_hash, expires_at) VALUES ($sid, $token_hash, $expires);
            COMMIT TRANSACTION;
        ")
            .bind(("sid", sid))
            .bind(("uid", uid))
            .bind(("token_hash", token_hash.to_string()))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    // --- Product CRUD ---

    /// Finds a product by its record ID.
    pub async fn find_product(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::product::Product>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(id.replace("product:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let product: Option<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(product)
    }

    /// Fetch multiple products by their IDs in a single query.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_products_by_ids(
        &self,
        ids: &[String],
    ) -> Result<Vec<sakaloka_core::models::product::Product>, SurrealError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let tids: Vec<surrealdb_types::RecordId> = ids
            .iter()
            .map(|id| {
                surrealdb_types::RecordId::new(
                    "product",
                    surrealdb_types::RecordIdKey::String(id.replace("product:", "")),
                )
            })
            .collect();

        let mut result = self
            .db
            .query("SELECT * FROM product WHERE id IN $ids")
            .bind(("ids", tids))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let products: Vec<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(products)
    }

    /// Lists products with pagination, search, and sorting.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_products(
        &self,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<sakaloka_core::models::product::Product>, SurrealError> {
        let sort_col = match sort_by {
            "name" => "name",
            "sku" => "sku",
            "base_price" => "base_price",
            "is_featured" => "is_featured",
            _ => "created_at",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let query = if search.is_some() {
            format!(
                "SELECT * FROM product WHERE deleted_at = NONE AND \
                 (string::lowercase(name) CONTAINS string::lowercase($search) \
                 OR string::lowercase(sku) CONTAINS string::lowercase($search) \
                 OR string::lowercase(barcode ?? '') CONTAINS string::lowercase($search)) \
                 ORDER BY {sort_col} {dir} LIMIT $limit START $start"
            )
        } else {
            format!(
                "SELECT * FROM product WHERE deleted_at = NONE \
                 ORDER BY {sort_col} {dir} LIMIT $limit START $start"
            )
        };

        let mut q = self.db.query(&query);
        q = q.bind(("limit", limit)).bind(("start", start));
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let products: Vec<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(products)
    }

    /// Counts products, with optional search filter.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_products(&self, search: Option<&str>) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let query = if search.is_some() {
            "SELECT count() AS count FROM product WHERE deleted_at = NONE AND \
             (string::lowercase(name) CONTAINS string::lowercase($search) \
             OR string::lowercase(sku) CONTAINS string::lowercase($search)) \
             GROUP ALL"
        } else {
            "SELECT count() AS count FROM product WHERE deleted_at = NONE GROUP ALL"
        };

        let mut q = self.db.query(query);
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Creates a new product with full field set.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
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
    ) -> Result<sakaloka_core::models::product::Product, SurrealError> {
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );
        let creator_tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(created_by.replace("user:", "")),
        );
        let cat_tid = category_id.map(|c| {
            surrealdb_types::RecordId::new(
                "category",
                surrealdb_types::RecordIdKey::String(c.replace("category:", "")),
            )
        });
        let brand_tid = brand_id.map(|b| {
            surrealdb_types::RecordId::new(
                "brand",
                surrealdb_types::RecordIdKey::String(b.replace("brand:", "")),
            )
        });

        let mut result = self
            .db
            .query(
                "CREATE product SET \
                 name = $name, \
                 sku = $sku, \
                 base_price = $base_price, \
                 description = $description, \
                 barcode = $barcode, \
                 cost_price = $cost_price, \
                 category_id = $category_id, \
                 brand_id = $brand_id, \
                 image_url = $image_url, \
                 weight = $weight, \
                 dimensions = $dimensions, \
                 track_inventory = $track_inventory, \
                 min_stock_level = $min_stock_level, \
                 is_featured = $is_featured, \
                 tags = $tags, \
                 organization_id = $organization_id, \
                 created_by = $created_by",
            )
            .bind(("name", name.to_string()))
            .bind(("sku", sku.to_string()))
            .bind(("base_price", base_price))
            .bind(("description", description.map(String::from)))
            .bind(("barcode", barcode.map(String::from)))
            .bind(("cost_price", cost_price))
            .bind(("category_id", cat_tid))
            .bind(("brand_id", brand_tid))
            .bind(("image_url", image_url.map(String::from)))
            .bind(("weight", weight))
            .bind(("dimensions", dimensions.cloned()))
            .bind(("track_inventory", track_inventory))
            .bind(("min_stock_level", min_stock_level.unwrap_or(0)))
            .bind(("is_featured", is_featured))
            .bind(("tags", tags.map(|t| t.to_vec())))
            .bind(("organization_id", org_tid))
            .bind(("created_by", creator_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let product: Option<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        product.ok_or_else(|| SurrealError::Query("Failed to create product".into()))
    }

    /// Updates an existing product by merging a JSON patch.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails or the product is not found.
    pub async fn update_product(
        &self,
        id: &str,
        updates: &serde_json::Value,
    ) -> Result<sakaloka_core::models::product::Product, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(id.replace("product:", "")),
        );

        let mut result = self
            .db
            .query("UPDATE $id MERGE $updates")
            .bind(("id", tid))
            .bind(("updates", updates.clone()))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let product: Option<sakaloka_core::models::product::Product> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        product.ok_or_else(|| SurrealError::Query("Product not found or update failed".into()))
    }

    /// Soft-deletes a product by setting `deleted_at` to the current timestamp.
    ///
    /// The product remains in the database but is excluded from list and count
    /// queries that filter on `deleted_at = NONE`.
    pub async fn delete_product(&self, id: &str) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(id.replace("product:", "")),
        );

        self.db
            .query("UPDATE $id SET deleted_at = time::now()")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    // =====================================================================
    // User CRUD (additions)
    // =====================================================================

    /// Finds a user by their email address.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<sakaloka_core::models::user::User>, SurrealError> {
        let mut result = self
            .db
            .query("SELECT * FROM user WHERE email = $email LIMIT 1")
            .bind(("email", email.to_string()))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let user: Option<sakaloka_core::models::user::User> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(user)
    }

    /// Finds a user by their record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_user_by_id(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::user::User>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(id.replace("user:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let user: Option<sakaloka_core::models::user::User> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(user)
    }

    /// Lists users with pagination, optional search, and sorting.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_users(
        &self,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<sakaloka_core::models::user::User>, SurrealError> {
        let sort_col = match sort_by {
            "email" => "email",
            "full_name" => "full_name",
            "username" => "username",
            "is_active" => "is_active",
            "last_login_at" => "last_login_at",
            _ => "created_at",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let query = if search.is_some() {
            format!(
                "SELECT * FROM user WHERE string::lowercase(email) CONTAINS string::lowercase($search) \
                 OR string::lowercase(full_name ?? '') CONTAINS string::lowercase($search) \
                 OR string::lowercase(username ?? '') CONTAINS string::lowercase($search) \
                 ORDER BY {sort_col} {dir} LIMIT $limit START $start"
            )
        } else {
            format!("SELECT * FROM user ORDER BY {sort_col} {dir} LIMIT $limit START $start")
        };

        let mut q = self.db.query(&query);
        q = q.bind(("limit", limit)).bind(("start", start));
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let users: Vec<sakaloka_core::models::user::User> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(users)
    }

    /// Counts users, optionally filtered by a search term.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_users(&self, search: Option<&str>) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let query = if search.is_some() {
            "SELECT count() AS count FROM user WHERE \
             string::lowercase(email) CONTAINS string::lowercase($search) \
             OR string::lowercase(full_name ?? '') CONTAINS string::lowercase($search) \
             OR string::lowercase(username ?? '') CONTAINS string::lowercase($search) \
             GROUP ALL"
        } else {
            "SELECT count() AS count FROM user GROUP ALL"
        };

        let mut q = self.db.query(query);
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Creates a new user.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
    pub async fn create_user(
        &self,
        email: &str,
        full_name: &str,
        password_hash: &str,
        organization_id: &str,
        role_id: &str,
    ) -> Result<sakaloka_core::models::user::User, SurrealError> {
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );
        let role_tid = surrealdb_types::RecordId::new(
            "role",
            surrealdb_types::RecordIdKey::String(role_id.replace("role:", "")),
        );

        let mut result = self
            .db
            .query(
                "CREATE user SET \
                 email = $email, \
                 full_name = $full_name, \
                 password_hash = $password_hash, \
                 organization_id = $organization_id, \
                 role_id = $role_id, \
                 is_active = true",
            )
            .bind(("email", email.to_string()))
            .bind(("full_name", full_name.to_string()))
            .bind(("password_hash", password_hash.to_string()))
            .bind(("organization_id", org_tid))
            .bind(("role_id", role_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let user: Option<sakaloka_core::models::user::User> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        user.ok_or_else(|| SurrealError::Query("Failed to create user".into()))
    }

    /// Updates an existing user with the provided optional fields.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails or the user is not found.
    pub async fn update_user(
        &self,
        id: &str,
        email: Option<&str>,
        full_name: Option<&str>,
        password_hash: Option<&str>,
        role_id: Option<&str>,
        is_active: Option<bool>,
    ) -> Result<sakaloka_core::models::user::User, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(id.replace("user:", "")),
        );

        let role_tid = role_id.map(|r| {
            surrealdb_types::RecordId::new(
                "role",
                surrealdb_types::RecordIdKey::String(r.replace("role:", "")),
            )
        });

        let mut result = self
            .db
            .query(
                "UPDATE $id MERGE { \
                 email: IF $email != NONE THEN $email ELSE email END, \
                 full_name: IF $full_name != NONE THEN $full_name ELSE full_name END, \
                 password_hash: IF $password_hash != NONE THEN $password_hash ELSE password_hash END, \
                 role_id: IF $role_id != NONE THEN $role_id ELSE role_id END, \
                 is_active: IF $is_active != NONE THEN $is_active ELSE is_active END \
                 }",
            )
            .bind(("id", tid))
            .bind(("email", email.map(String::from)))
            .bind(("full_name", full_name.map(String::from)))
            .bind(("password_hash", password_hash.map(String::from)))
            .bind(("role_id", role_tid))
            .bind(("is_active", is_active))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let user: Option<sakaloka_core::models::user::User> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        user.ok_or_else(|| SurrealError::Query("User not found or update failed".into()))
    }

    /// Switches a user's active organization.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails.
    pub async fn update_user_organization(
        &self,
        user_id: &str,
        organization_id: &str,
    ) -> Result<(), SurrealError> {
        let user_tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(user_id.replace("user:", "")),
        );
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );

        self.db
            .query("UPDATE $id SET organization_id = $org_id")
            .bind(("id", user_tid))
            .bind(("org_id", org_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    /// Updates a user's last login timestamp to now.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails.
    pub async fn update_last_login(&self, id: &str) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(id.replace("user:", "")),
        );

        self.db
            .query("UPDATE $id SET last_login_at = time::now()")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    /// Deletes a user record by ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the deletion fails.
    pub async fn delete_user(&self, id: &str) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(id.replace("user:", "")),
        );
        self.db
            .query("DELETE $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;
        Ok(())
    }

    // =====================================================================
    // Organization CRUD
    // =====================================================================

    /// Finds an organization by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_organization(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::organization::Organization>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(id.replace("organization:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let org: Option<sakaloka_core::models::organization::Organization> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(org)
    }

    /// Lists organizations with pagination, optional search, and sorting.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_organizations(
        &self,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<sakaloka_core::models::organization::Organization>, SurrealError> {
        let sort_col = match sort_by {
            "name" => "name",
            "type" => "type",
            "is_active" => "is_active",
            _ => "created_at",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let query = if search.is_some() {
            format!(
                "SELECT * FROM organization \
                 WHERE string::lowercase(name) CONTAINS string::lowercase($search) \
                 ORDER BY {sort_col} {dir} LIMIT $limit START $start"
            )
        } else {
            format!(
                "SELECT * FROM organization ORDER BY {sort_col} {dir} LIMIT $limit START $start"
            )
        };

        let mut q = self.db.query(&query);
        q = q.bind(("limit", limit)).bind(("start", start));
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let orgs: Vec<sakaloka_core::models::organization::Organization> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(orgs)
    }

    /// Counts organizations, optionally filtered by a search term.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_organizations(&self, search: Option<&str>) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let query = if search.is_some() {
            "SELECT count() AS count FROM organization \
             WHERE string::lowercase(name) CONTAINS string::lowercase($search) \
             GROUP ALL"
        } else {
            "SELECT count() AS count FROM organization GROUP ALL"
        };

        let mut q = self.db.query(query);
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Creates a new organization with minimal fields (for registration flow).
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
    pub async fn create_organization(
        &self,
        name: &str,
        org_type: &str,
        owner_id: Option<&str>,
    ) -> Result<sakaloka_core::models::organization::Organization, SurrealError> {
        let owner_tid = owner_id.map(|o| {
            surrealdb_types::RecordId::new(
                "user",
                surrealdb_types::RecordIdKey::String(o.replace("user:", "")),
            )
        });

        let mut result = self
            .db
            .query(
                "CREATE organization SET \
                 name = $name, \
                 type = $org_type, \
                 owner_id = $owner_id, \
                 is_active = true",
            )
            .bind(("name", name.to_string()))
            .bind(("org_type", org_type.to_string()))
            .bind(("owner_id", owner_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let org: Option<sakaloka_core::models::organization::Organization> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        org.ok_or_else(|| SurrealError::Query("Failed to create organization".into()))
    }

    /// Updates an organization using a MERGE of arbitrary JSON fields.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails or the organization is not found.
    pub async fn update_organization(
        &self,
        id: &str,
        updates: &serde_json::Value,
    ) -> Result<sakaloka_core::models::organization::Organization, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(id.replace("organization:", "")),
        );

        let mut result = self
            .db
            .query("UPDATE $id MERGE $updates")
            .bind(("id", tid))
            .bind(("updates", updates.clone()))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let org: Option<sakaloka_core::models::organization::Organization> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        org.ok_or_else(|| SurrealError::Query("Organization not found or update failed".into()))
    }

    /// Updates the owner of an organization.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails.
    pub async fn update_organization_owner(
        &self,
        id: &str,
        owner_id: &str,
    ) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(id.replace("organization:", "")),
        );
        let owner_tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(owner_id.replace("user:", "")),
        );

        self.db
            .query("UPDATE $id SET owner_id = $owner_id")
            .bind(("id", tid))
            .bind(("owner_id", owner_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    /// Deletes an organization record by ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the deletion fails.
    pub async fn delete_organization(&self, id: &str) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(id.replace("organization:", "")),
        );
        self.db
            .query("DELETE $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;
        Ok(())
    }

    // =====================================================================
    // Role CRUD
    // =====================================================================

    /// Finds a role by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_role(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::role::Role>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "role",
            surrealdb_types::RecordIdKey::String(id.replace("role:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let role: Option<sakaloka_core::models::role::Role> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(role)
    }

    /// Lists roles with pagination, optional search, and sorting.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_roles(
        &self,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<sakaloka_core::models::role::Role>, SurrealError> {
        let sort_col = match sort_by {
            "name" => "name",
            "is_system_role" => "is_system_role",
            "is_active" => "is_active",
            _ => "created_at",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let query = if search.is_some() {
            format!(
                "SELECT * FROM role \
                 WHERE string::lowercase(name) CONTAINS string::lowercase($search) \
                 ORDER BY {sort_col} {dir} LIMIT $limit START $start"
            )
        } else {
            format!("SELECT * FROM role ORDER BY {sort_col} {dir} LIMIT $limit START $start")
        };

        let mut q = self.db.query(&query);
        q = q.bind(("limit", limit)).bind(("start", start));
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let roles: Vec<sakaloka_core::models::role::Role> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(roles)
    }

    /// Counts roles, optionally filtered by a search term.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_roles(&self, search: Option<&str>) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let query = if search.is_some() {
            "SELECT count() AS count FROM role \
             WHERE string::lowercase(name) CONTAINS string::lowercase($search) \
             GROUP ALL"
        } else {
            "SELECT count() AS count FROM role GROUP ALL"
        };

        let mut q = self.db.query(query);
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Creates a new role.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
    pub async fn create_role(
        &self,
        name: &str,
        organization_id: &str,
        permissions: &serde_json::Value,
        is_system_role: bool,
    ) -> Result<sakaloka_core::models::role::Role, SurrealError> {
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );

        let mut result = self
            .db
            .query(
                "CREATE role SET \
                 name = $name, \
                 organization_id = $organization_id, \
                 permissions = $permissions, \
                 is_system_role = $is_system_role, \
                 is_active = true",
            )
            .bind(("name", name.to_string()))
            .bind(("organization_id", org_tid))
            .bind(("permissions", permissions.clone()))
            .bind(("is_system_role", is_system_role))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let role: Option<sakaloka_core::models::role::Role> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        role.ok_or_else(|| SurrealError::Query("Failed to create role".into()))
    }

    /// Updates an existing role with the provided optional fields.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails or the role is not found.
    pub async fn update_role(
        &self,
        id: &str,
        name: Option<&str>,
        permissions: Option<&serde_json::Value>,
        is_active: Option<bool>,
    ) -> Result<sakaloka_core::models::role::Role, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "role",
            surrealdb_types::RecordIdKey::String(id.replace("role:", "")),
        );

        let mut result = self
            .db
            .query(
                "UPDATE $id MERGE { \
                 name: IF $name != NONE THEN $name ELSE name END, \
                 permissions: IF $permissions != NONE THEN $permissions ELSE permissions END, \
                 is_active: IF $is_active != NONE THEN $is_active ELSE is_active END \
                 }",
            )
            .bind(("id", tid))
            .bind(("name", name.map(String::from)))
            .bind(("permissions", permissions.cloned()))
            .bind(("is_active", is_active))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let role: Option<sakaloka_core::models::role::Role> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        role.ok_or_else(|| SurrealError::Query("Role not found or update failed".into()))
    }

    /// Deletes a role by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn delete_role(&self, id: &str) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "role",
            surrealdb_types::RecordIdKey::String(id.replace("role:", "")),
        );

        self.db
            .query("DELETE $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    /// Finds a role by its name within a specific organization.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_role_by_name_and_org(
        &self,
        name: &str,
        organization_id: &str,
    ) -> Result<Option<sakaloka_core::models::role::Role>, SurrealError> {
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );

        let mut result = self
            .db
            .query(
                "SELECT * FROM role \
                 WHERE name = $name AND organization_id = $organization_id \
                 LIMIT 1",
            )
            .bind(("name", name.to_string()))
            .bind(("organization_id", org_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let role: Option<sakaloka_core::models::role::Role> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(role)
    }

    /// Counts users that have a specific role assigned.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_users_with_role(&self, role_id: &str) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let role_tid = surrealdb_types::RecordId::new(
            "role",
            surrealdb_types::RecordIdKey::String(role_id.replace("role:", "")),
        );

        let mut result = self
            .db
            .query(
                "SELECT count() AS count FROM user \
                 WHERE role_id = $role_id \
                 GROUP ALL",
            )
            .bind(("role_id", role_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    // =====================================================================
    // Category CRUD
    // =====================================================================

    /// Finds a category by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_category(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::category::Category>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "category",
            surrealdb_types::RecordIdKey::String(id.replace("category:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let cat: Option<sakaloka_core::models::category::Category> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(cat)
    }

    /// Fetch multiple categories by their IDs in a single query.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_categories_by_ids(
        &self,
        ids: &[String],
    ) -> Result<Vec<sakaloka_core::models::category::Category>, SurrealError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let tids: Vec<surrealdb_types::RecordId> = ids
            .iter()
            .map(|id| {
                surrealdb_types::RecordId::new(
                    "category",
                    surrealdb_types::RecordIdKey::String(id.replace("category:", "")),
                )
            })
            .collect();

        let mut result = self
            .db
            .query("SELECT * FROM category WHERE id IN $ids")
            .bind(("ids", tids))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let cats: Vec<sakaloka_core::models::category::Category> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(cats)
    }

    /// Lists categories with pagination, optional search, and sorting.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_categories(
        &self,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<sakaloka_core::models::category::Category>, SurrealError> {
        let sort_col = match sort_by {
            "name" => "name",
            "slug" => "slug",
            "sort_order" => "sort_order",
            "is_active" => "is_active",
            _ => "created_at",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let query = if search.is_some() {
            format!(
                "SELECT * FROM category \
                 WHERE string::lowercase(name) CONTAINS string::lowercase($search) \
                 ORDER BY {sort_col} {dir} LIMIT $limit START $start"
            )
        } else {
            format!("SELECT * FROM category ORDER BY {sort_col} {dir} LIMIT $limit START $start")
        };

        let mut q = self.db.query(&query);
        q = q.bind(("limit", limit)).bind(("start", start));
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let categories: Vec<sakaloka_core::models::category::Category> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(categories)
    }

    /// Counts categories, optionally filtered by a search term.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_categories(&self, search: Option<&str>) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let query = if search.is_some() {
            "SELECT count() AS count FROM category \
             WHERE string::lowercase(name) CONTAINS string::lowercase($search) \
             GROUP ALL"
        } else {
            "SELECT count() AS count FROM category GROUP ALL"
        };

        let mut q = self.db.query(query);
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Creates a new category.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_category(
        &self,
        name: &str,
        slug: &str,
        organization_id: &str,
        description: Option<&str>,
        parent_id: Option<&str>,
        image_url: Option<&str>,
        created_by: Option<&str>,
    ) -> Result<sakaloka_core::models::category::Category, SurrealError> {
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );
        let parent_tid = parent_id.map(|p| {
            surrealdb_types::RecordId::new(
                "category",
                surrealdb_types::RecordIdKey::String(p.replace("category:", "")),
            )
        });
        let creator_tid = created_by.map(|c| {
            surrealdb_types::RecordId::new(
                "user",
                surrealdb_types::RecordIdKey::String(c.replace("user:", "")),
            )
        });

        let mut result = self
            .db
            .query(
                "CREATE category SET \
                 name = $name, \
                 slug = $slug, \
                 organization_id = $organization_id, \
                 description = $description, \
                 parent_id = $parent_id, \
                 image_url = $image_url, \
                 created_by = $created_by, \
                 sort_order = 0, \
                 is_active = true",
            )
            .bind(("name", name.to_string()))
            .bind(("slug", slug.to_string()))
            .bind(("organization_id", org_tid))
            .bind(("description", description.map(String::from)))
            .bind(("parent_id", parent_tid))
            .bind(("image_url", image_url.map(String::from)))
            .bind(("created_by", creator_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let cat: Option<sakaloka_core::models::category::Category> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        cat.ok_or_else(|| SurrealError::Query("Failed to create category".into()))
    }

    /// Updates an existing category with the provided optional fields.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails or the category is not found.
    pub async fn update_category(
        &self,
        id: &str,
        name: Option<&str>,
        slug: Option<&str>,
        description: Option<&str>,
        parent_id: Option<&str>,
        image_url: Option<&str>,
    ) -> Result<sakaloka_core::models::category::Category, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "category",
            surrealdb_types::RecordIdKey::String(id.replace("category:", "")),
        );

        let parent_tid = parent_id.map(|p| {
            surrealdb_types::RecordId::new(
                "category",
                surrealdb_types::RecordIdKey::String(p.replace("category:", "")),
            )
        });

        let mut result = self
            .db
            .query(
                "UPDATE $id MERGE { \
                 name: IF $name != NONE THEN $name ELSE name END, \
                 slug: IF $slug != NONE THEN $slug ELSE slug END, \
                 description: IF $description != NONE THEN $description ELSE description END, \
                 parent_id: IF $parent_id != NONE THEN $parent_id ELSE parent_id END, \
                 image_url: IF $image_url != NONE THEN $image_url ELSE image_url END \
                 }",
            )
            .bind(("id", tid))
            .bind(("name", name.map(String::from)))
            .bind(("slug", slug.map(String::from)))
            .bind(("description", description.map(String::from)))
            .bind(("parent_id", parent_tid))
            .bind(("image_url", image_url.map(String::from)))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let cat: Option<sakaloka_core::models::category::Category> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        cat.ok_or_else(|| SurrealError::Query("Category not found or update failed".into()))
    }

    /// Deletes a category by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn delete_category(&self, id: &str) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "category",
            surrealdb_types::RecordIdKey::String(id.replace("category:", "")),
        );

        self.db
            .query("DELETE $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    /// Counts child categories under a given parent.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_child_categories(&self, parent_id: &str) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let parent_tid = surrealdb_types::RecordId::new(
            "category",
            surrealdb_types::RecordIdKey::String(parent_id.replace("category:", "")),
        );

        let mut result = self
            .db
            .query(
                "SELECT count() AS count FROM category \
                 WHERE parent_id = $parent_id \
                 GROUP ALL",
            )
            .bind(("parent_id", parent_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    // =====================================================================
    // Brand CRUD
    // =====================================================================

    /// Finds a brand by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_brand(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::brand::Brand>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "brand",
            surrealdb_types::RecordIdKey::String(id.replace("brand:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let brand: Option<sakaloka_core::models::brand::Brand> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(brand)
    }

    /// Fetch multiple brands by their IDs in a single query.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_brands_by_ids(
        &self,
        ids: &[String],
    ) -> Result<Vec<sakaloka_core::models::brand::Brand>, SurrealError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let tids: Vec<surrealdb_types::RecordId> = ids
            .iter()
            .map(|id| {
                surrealdb_types::RecordId::new(
                    "brand",
                    surrealdb_types::RecordIdKey::String(id.replace("brand:", "")),
                )
            })
            .collect();

        let mut result = self
            .db
            .query("SELECT * FROM brand WHERE id IN $ids")
            .bind(("ids", tids))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let brands: Vec<sakaloka_core::models::brand::Brand> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(brands)
    }

    /// Lists brands with pagination, optional search, and sorting.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_brands(
        &self,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<sakaloka_core::models::brand::Brand>, SurrealError> {
        let sort_col = match sort_by {
            "name" => "name",
            "slug" => "slug",
            "is_active" => "is_active",
            _ => "created_at",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let query = if search.is_some() {
            format!(
                "SELECT * FROM brand \
                 WHERE string::lowercase(name) CONTAINS string::lowercase($search) \
                 ORDER BY {sort_col} {dir} LIMIT $limit START $start"
            )
        } else {
            format!("SELECT * FROM brand ORDER BY {sort_col} {dir} LIMIT $limit START $start")
        };

        let mut q = self.db.query(&query);
        q = q.bind(("limit", limit)).bind(("start", start));
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let brands: Vec<sakaloka_core::models::brand::Brand> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(brands)
    }

    /// Counts brands, optionally filtered by a search term.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_brands(&self, search: Option<&str>) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let query = if search.is_some() {
            "SELECT count() AS count FROM brand \
             WHERE string::lowercase(name) CONTAINS string::lowercase($search) \
             GROUP ALL"
        } else {
            "SELECT count() AS count FROM brand GROUP ALL"
        };

        let mut q = self.db.query(query);
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Creates a new brand.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_brand(
        &self,
        name: &str,
        slug: &str,
        organization_id: &str,
        description: Option<&str>,
        logo: Option<&str>,
        website: Option<&str>,
        created_by: Option<&str>,
    ) -> Result<sakaloka_core::models::brand::Brand, SurrealError> {
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );
        let creator_tid = created_by.map(|c| {
            surrealdb_types::RecordId::new(
                "user",
                surrealdb_types::RecordIdKey::String(c.replace("user:", "")),
            )
        });

        let mut result = self
            .db
            .query(
                "CREATE brand SET \
                 name = $name, \
                 slug = $slug, \
                 organization_id = $organization_id, \
                 description = $description, \
                 logo = $logo, \
                 website = $website, \
                 created_by = $created_by, \
                 is_active = true",
            )
            .bind(("name", name.to_string()))
            .bind(("slug", slug.to_string()))
            .bind(("organization_id", org_tid))
            .bind(("description", description.map(String::from)))
            .bind(("logo", logo.map(String::from)))
            .bind(("website", website.map(String::from)))
            .bind(("created_by", creator_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let brand: Option<sakaloka_core::models::brand::Brand> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        brand.ok_or_else(|| SurrealError::Query("Failed to create brand".into()))
    }

    /// Updates an existing brand with the provided optional fields.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails or the brand is not found.
    pub async fn update_brand(
        &self,
        id: &str,
        name: Option<&str>,
        slug: Option<&str>,
        description: Option<&str>,
        logo: Option<&str>,
        website: Option<&str>,
    ) -> Result<sakaloka_core::models::brand::Brand, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "brand",
            surrealdb_types::RecordIdKey::String(id.replace("brand:", "")),
        );

        let mut result = self
            .db
            .query(
                "UPDATE $id MERGE { \
                 name: IF $name != NONE THEN $name ELSE name END, \
                 slug: IF $slug != NONE THEN $slug ELSE slug END, \
                 description: IF $description != NONE THEN $description ELSE description END, \
                 logo: IF $logo != NONE THEN $logo ELSE logo END, \
                 website: IF $website != NONE THEN $website ELSE website END \
                 }",
            )
            .bind(("id", tid))
            .bind(("name", name.map(String::from)))
            .bind(("slug", slug.map(String::from)))
            .bind(("description", description.map(String::from)))
            .bind(("logo", logo.map(String::from)))
            .bind(("website", website.map(String::from)))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let brand: Option<sakaloka_core::models::brand::Brand> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        brand.ok_or_else(|| SurrealError::Query("Brand not found or update failed".into()))
    }

    /// Deletes a brand by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn delete_brand(&self, id: &str) -> Result<(), SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "brand",
            surrealdb_types::RecordIdKey::String(id.replace("brand:", "")),
        );

        self.db
            .query("DELETE $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?
            .check()
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(())
    }

    // =====================================================================
    // Order CRUD
    // =====================================================================

    /// Finds an order by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_order(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::order::Order>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "order",
            surrealdb_types::RecordIdKey::String(id.replace("order:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let order: Option<sakaloka_core::models::order::Order> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(order)
    }

    /// Lists orders with pagination, optional search, sorting, and status filters.
    ///
    /// `status_filter` includes only the listed statuses; `status_exclude` excludes them.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn list_orders(
        &self,
        limit: u64,
        start: u64,
        search: Option<&str>,
        sort_by: &str,
        sort_desc: bool,
        status_filter: Option<&[&str]>,
        status_exclude: Option<&[&str]>,
    ) -> Result<Vec<sakaloka_core::models::order::Order>, SurrealError> {
        let sort_col = match sort_by {
            "order_number" => "order_number",
            "status" => "status",
            "total_amount" => "total_amount",
            "payment_status" => "payment_status",
            _ => "created_at",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let mut conditions: Vec<String> = Vec::new();

        if search.is_some() {
            conditions.push(
                "(string::lowercase(order_number) CONTAINS string::lowercase($search) \
                 OR string::lowercase(customer_name ?? '') CONTAINS string::lowercase($search))"
                    .to_string(),
            );
        }
        if status_filter.is_some() {
            conditions.push("status IN $status_filter".to_string());
        }
        if status_exclude.is_some() {
            conditions.push("status NOT IN $status_exclude".to_string());
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let query = format!(
            "SELECT * FROM order {where_clause} ORDER BY {sort_col} {dir} LIMIT $limit START $start"
        );

        let mut q = self.db.query(&query);
        q = q.bind(("limit", limit)).bind(("start", start));
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }
        if let Some(sf) = status_filter {
            let vals: Vec<String> = sf.iter().map(|s| (*s).to_string()).collect();
            q = q.bind(("status_filter", vals));
        }
        if let Some(se) = status_exclude {
            let vals: Vec<String> = se.iter().map(|s| (*s).to_string()).collect();
            q = q.bind(("status_exclude", vals));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let orders: Vec<sakaloka_core::models::order::Order> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(orders)
    }

    /// Counts orders, with optional search and status filters.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_orders(
        &self,
        search: Option<&str>,
        status_filter: Option<&[&str]>,
        status_exclude: Option<&[&str]>,
    ) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let mut conditions: Vec<String> = Vec::new();

        if search.is_some() {
            conditions.push(
                "(string::lowercase(order_number) CONTAINS string::lowercase($search) \
                 OR string::lowercase(customer_name ?? '') CONTAINS string::lowercase($search))"
                    .to_string(),
            );
        }
        if status_filter.is_some() {
            conditions.push("status IN $status_filter".to_string());
        }
        if status_exclude.is_some() {
            conditions.push("status NOT IN $status_exclude".to_string());
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let query = format!("SELECT count() AS count FROM order {where_clause} GROUP ALL");

        let mut q = self.db.query(&query);
        if let Some(s) = search {
            q = q.bind(("search", s.to_string()));
        }
        if let Some(sf) = status_filter {
            let vals: Vec<String> = sf.iter().map(|s| (*s).to_string()).collect();
            q = q.bind(("status_filter", vals));
        }
        if let Some(se) = status_exclude {
            let vals: Vec<String> = se.iter().map(|s| (*s).to_string()).collect();
            q = q.bind(("status_exclude", vals));
        }

        let mut result = q.await.map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Creates a new order.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
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
    ) -> Result<sakaloka_core::models::order::Order, SurrealError> {
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );
        let creator_tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(created_by.replace("user:", "")),
        );

        let mut result = self
            .db
            .query(
                "CREATE order SET \
                 order_number = $order_number, \
                 organization_id = $organization_id, \
                 type = $order_type, \
                 status = 'pending', \
                 subtotal = $subtotal, \
                 tax_amount = $tax_amount, \
                 discount_amount = 0, \
                 total_amount = $total_amount, \
                 payment_method = $payment_method, \
                 payment_status = 'unpaid', \
                 paid_amount = 0, \
                 notes = $notes, \
                 table_number = $table_number, \
                 customer_name = $customer_name, \
                 created_by = $created_by, \
                 updated_by = $created_by",
            )
            .bind(("order_number", order_number.to_string()))
            .bind(("organization_id", org_tid))
            .bind(("order_type", order_type.to_string()))
            .bind(("subtotal", subtotal))
            .bind(("tax_amount", tax_amount))
            .bind(("total_amount", total_amount))
            .bind(("payment_method", payment_method.map(String::from)))
            .bind(("notes", notes.map(String::from)))
            .bind(("table_number", table_number.map(String::from)))
            .bind(("customer_name", customer_name.map(String::from)))
            .bind(("created_by", creator_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let order: Option<sakaloka_core::models::order::Order> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        order.ok_or_else(|| SurrealError::Query("Failed to create order".into()))
    }

    /// Updates an existing order with the provided optional fields.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails or the order is not found.
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
    ) -> Result<sakaloka_core::models::order::Order, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "order",
            surrealdb_types::RecordIdKey::String(id.replace("order:", "")),
        );
        let updater_tid = surrealdb_types::RecordId::new(
            "user",
            surrealdb_types::RecordIdKey::String(updated_by.replace("user:", "")),
        );

        let mut result = self
            .db
            .query(
                "UPDATE $id MERGE { \
                 status: IF $status != NONE THEN $status ELSE status END, \
                 payment_method: IF $payment_method != NONE THEN $payment_method ELSE payment_method END, \
                 payment_status: IF $payment_status != NONE THEN $payment_status ELSE payment_status END, \
                 paid_amount: IF $paid_amount != NONE THEN $paid_amount ELSE paid_amount END, \
                 notes: IF $notes != NONE THEN $notes ELSE notes END, \
                 table_number: IF $table_number != NONE THEN $table_number ELSE table_number END, \
                 customer_name: IF $customer_name != NONE THEN $customer_name ELSE customer_name END, \
                 updated_by: $updated_by \
                 }",
            )
            .bind(("id", tid))
            .bind(("status", status.map(String::from)))
            .bind(("payment_method", payment_method.map(String::from)))
            .bind(("payment_status", payment_status.map(String::from)))
            .bind(("paid_amount", paid_amount))
            .bind(("notes", notes.map(String::from)))
            .bind(("table_number", table_number.map(String::from)))
            .bind(("customer_name", customer_name.map(String::from)))
            .bind(("updated_by", updater_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let order: Option<sakaloka_core::models::order::Order> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        order.ok_or_else(|| SurrealError::Query("Order not found or update failed".into()))
    }

    // =====================================================================
    // OrderItem CRUD
    // =====================================================================

    /// Creates a new order item.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
    pub async fn create_order_item(
        &self,
        order_id: &str,
        product_id: &str,
        item_name: &str,
        quantity: i32,
        unit_price: i64,
        total_price: i64,
    ) -> Result<sakaloka_core::models::order::OrderItem, SurrealError> {
        let order_tid = surrealdb_types::RecordId::new(
            "order",
            surrealdb_types::RecordIdKey::String(order_id.replace("order:", "")),
        );
        let product_tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(product_id.replace("product:", "")),
        );

        let mut result = self
            .db
            .query(
                "CREATE order_item SET \
                 order_id = $order_id, \
                 product_id = $product_id, \
                 item_name = $item_name, \
                 quantity = $quantity, \
                 unit_price = $unit_price, \
                 discount_amount = 0, \
                 total_price = $total_price",
            )
            .bind(("order_id", order_tid))
            .bind(("product_id", product_tid))
            .bind(("item_name", item_name.to_string()))
            .bind(("quantity", quantity))
            .bind(("unit_price", unit_price))
            .bind(("total_price", total_price))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let item: Option<sakaloka_core::models::order::OrderItem> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        item.ok_or_else(|| SurrealError::Query("Failed to create order item".into()))
    }

    /// Lists all order items belonging to a specific order.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_order_items(
        &self,
        order_id: &str,
    ) -> Result<Vec<sakaloka_core::models::order::OrderItem>, SurrealError> {
        let order_tid = surrealdb_types::RecordId::new(
            "order",
            surrealdb_types::RecordIdKey::String(order_id.replace("order:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM order_item WHERE order_id = $order_id ORDER BY created_at ASC")
            .bind(("order_id", order_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let items: Vec<sakaloka_core::models::order::OrderItem> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(items)
    }

    // =====================================================================
    // InventoryItem CRUD
    // =====================================================================

    /// Finds an inventory item by its record ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_inventory_item(
        &self,
        id: &str,
    ) -> Result<Option<sakaloka_core::models::inventory::InventoryItem>, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "inventory_item",
            surrealdb_types::RecordIdKey::String(id.replace("inventory_item:", "")),
        );

        let mut result = self
            .db
            .query("SELECT * FROM $id")
            .bind(("id", tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let item: Option<sakaloka_core::models::inventory::InventoryItem> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(item)
    }

    /// Finds an inventory item by product ID and organization ID.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn find_inventory_by_product_org(
        &self,
        product_id: &str,
        organization_id: &str,
    ) -> Result<Option<sakaloka_core::models::inventory::InventoryItem>, SurrealError> {
        let product_tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(product_id.replace("product:", "")),
        );
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );

        let mut result = self
            .db
            .query(
                "SELECT * FROM inventory_item \
                 WHERE product_id = $product_id AND organization_id = $organization_id \
                 LIMIT 1",
            )
            .bind(("product_id", product_tid))
            .bind(("organization_id", org_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let item: Option<sakaloka_core::models::inventory::InventoryItem> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(item)
    }

    /// Lists inventory items with pagination and sorting.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_inventory(
        &self,
        limit: u64,
        start: u64,
        sort_by: &str,
        sort_desc: bool,
    ) -> Result<Vec<sakaloka_core::models::inventory::InventoryItem>, SurrealError> {
        let sort_col = match sort_by {
            "quantity_on_hand" => "quantity_on_hand",
            "quantity_available" => "quantity_available",
            "sku" => "sku",
            _ => "created_at",
        };
        let dir = if sort_desc { "DESC" } else { "ASC" };

        let query = format!(
            "SELECT * FROM inventory_item ORDER BY {sort_col} {dir} LIMIT $limit START $start"
        );

        let mut result = self
            .db
            .query(&query)
            .bind(("limit", limit))
            .bind(("start", start))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let items: Vec<sakaloka_core::models::inventory::InventoryItem> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(items)
    }

    /// Counts all inventory items.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_inventory(&self) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let mut result = self
            .db
            .query("SELECT count() AS count FROM inventory_item GROUP ALL")
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Lists inventory items where available stock is at or below the minimum
    /// level, with pagination.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_low_stock(
        &self,
        limit: u64,
        start: u64,
    ) -> Result<Vec<sakaloka_core::models::inventory::InventoryItem>, SurrealError> {
        let mut result = self
            .db
            .query(
                "SELECT * FROM inventory_item \
                 WHERE quantity_available <= min_stock_level \
                 ORDER BY quantity_available ASC \
                 LIMIT $limit START $start",
            )
            .bind(("limit", limit))
            .bind(("start", start))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let items: Vec<sakaloka_core::models::inventory::InventoryItem> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(items)
    }

    /// Counts inventory items where available stock is at or below the minimum
    /// level.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_low_stock(&self) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let mut result = self
            .db
            .query(
                "SELECT count() AS count FROM inventory_item \
                 WHERE quantity_available <= min_stock_level \
                 GROUP ALL",
            )
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }

    /// Creates a new inventory item for a product within an organization.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
    pub async fn create_inventory_item(
        &self,
        product_id: &str,
        organization_id: &str,
    ) -> Result<sakaloka_core::models::inventory::InventoryItem, SurrealError> {
        let product_tid = surrealdb_types::RecordId::new(
            "product",
            surrealdb_types::RecordIdKey::String(product_id.replace("product:", "")),
        );
        let org_tid = surrealdb_types::RecordId::new(
            "organization",
            surrealdb_types::RecordIdKey::String(organization_id.replace("organization:", "")),
        );

        let mut result = self
            .db
            .query(
                "CREATE inventory_item SET \
                 product_id = $product_id, \
                 organization_id = $organization_id, \
                 quantity_on_hand = 0, \
                 quantity_reserved = 0, \
                 quantity_available = 0, \
                 min_stock_level = 0",
            )
            .bind(("product_id", product_tid))
            .bind(("organization_id", org_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let item: Option<sakaloka_core::models::inventory::InventoryItem> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        item.ok_or_else(|| SurrealError::Query("Failed to create inventory item".into()))
    }

    /// Updates the stock quantities for an inventory item.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the update fails or the item is not found.
    pub async fn update_inventory_stock(
        &self,
        id: &str,
        quantity_on_hand: i32,
        quantity_available: i32,
    ) -> Result<sakaloka_core::models::inventory::InventoryItem, SurrealError> {
        let tid = surrealdb_types::RecordId::new(
            "inventory_item",
            surrealdb_types::RecordIdKey::String(id.replace("inventory_item:", "")),
        );

        let mut result = self
            .db
            .query(
                "UPDATE $id SET \
                 quantity_on_hand = $quantity_on_hand, \
                 quantity_available = $quantity_available",
            )
            .bind(("id", tid))
            .bind(("quantity_on_hand", quantity_on_hand))
            .bind(("quantity_available", quantity_available))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let item: Option<sakaloka_core::models::inventory::InventoryItem> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        item.ok_or_else(|| SurrealError::Query("Inventory item not found or update failed".into()))
    }

    // =====================================================================
    // StockMovement CRUD
    // =====================================================================

    /// Creates a new stock movement record.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the insert fails.
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
    ) -> Result<sakaloka_core::models::inventory::StockMovement, SurrealError> {
        let inv_tid = surrealdb_types::RecordId::new(
            "inventory_item",
            surrealdb_types::RecordIdKey::String(inventory_item_id.replace("inventory_item:", "")),
        );
        let creator_tid = created_by.map(|c| {
            surrealdb_types::RecordId::new(
                "user",
                surrealdb_types::RecordIdKey::String(c.replace("user:", "")),
            )
        });

        let mut result = self
            .db
            .query(
                "CREATE stock_movement SET \
                 inventory_item_id = $inventory_item_id, \
                 movement_type = $movement_type, \
                 quantity = $quantity, \
                 reference_type = $reference_type, \
                 reference_id = $reference_id, \
                 notes = $notes, \
                 created_by = $created_by",
            )
            .bind(("inventory_item_id", inv_tid))
            .bind(("movement_type", movement_type.to_string()))
            .bind(("quantity", quantity))
            .bind(("reference_type", reference_type.map(String::from)))
            .bind(("reference_id", reference_id.map(String::from)))
            .bind(("notes", notes.map(String::from)))
            .bind(("created_by", creator_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let movement: Option<sakaloka_core::models::inventory::StockMovement> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        movement.ok_or_else(|| SurrealError::Query("Failed to create stock movement".into()))
    }

    /// Lists stock movements for a specific inventory item with pagination.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn list_stock_movements(
        &self,
        inventory_item_id: &str,
        limit: u64,
        start: u64,
    ) -> Result<Vec<sakaloka_core::models::inventory::StockMovement>, SurrealError> {
        let inv_tid = surrealdb_types::RecordId::new(
            "inventory_item",
            surrealdb_types::RecordIdKey::String(inventory_item_id.replace("inventory_item:", "")),
        );

        let mut result = self
            .db
            .query(
                "SELECT * FROM stock_movement \
                 WHERE inventory_item_id = $inventory_item_id \
                 ORDER BY created_at DESC \
                 LIMIT $limit START $start",
            )
            .bind(("inventory_item_id", inv_tid))
            .bind(("limit", limit))
            .bind(("start", start))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let movements: Vec<sakaloka_core::models::inventory::StockMovement> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(movements)
    }

    /// Counts stock movements for a specific inventory item.
    ///
    /// # Errors
    /// Returns [`SurrealError::Query`] if the query fails.
    pub async fn count_stock_movements(
        &self,
        inventory_item_id: &str,
    ) -> Result<u64, SurrealError> {
        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct CountResult {
            count: u64,
        }

        let inv_tid = surrealdb_types::RecordId::new(
            "inventory_item",
            surrealdb_types::RecordIdKey::String(inventory_item_id.replace("inventory_item:", "")),
        );

        let mut result = self
            .db
            .query(
                "SELECT count() AS count FROM stock_movement \
                 WHERE inventory_item_id = $inventory_item_id \
                 GROUP ALL",
            )
            .bind(("inventory_item_id", inv_tid))
            .await
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        let row: Option<CountResult> = result
            .take(0)
            .map_err(|e| SurrealError::Query(e.to_string()))?;

        Ok(row.map(|r| r.count).unwrap_or(0))
    }
}

impl sakaloka_secure::tokens::rotation::JtiStore for SurrealClient {
    async fn is_jti_blocked(&self, jti: &str) -> Result<bool, sakaloka_secure::error::SecureError> {
        let mut result = self
            .db
            .query("SELECT * FROM jti_blocklist WHERE jti = $jti LIMIT 1")
            .bind(("jti", jti.to_string()))
            .await
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e))
            })?;

        let rows: Vec<serde_json::Value> = result.take(0).map_err(|e| {
            sakaloka_secure::error::SecureError::JwtDecode(format!("Query error: {}", e))
        })?;

        Ok(!rows.is_empty())
    }

    async fn block_jti(
        &self,
        jti: &str,
        exp: u64,
    ) -> Result<(), sakaloka_secure::error::SecureError> {
        self.db
            .query("INSERT INTO jti_blocklist (jti, exp) VALUES ($jti, $exp)")
            .bind(("jti", jti.to_string()))
            .bind(("exp", exp))
            .await
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e))
            })?
            .check()
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("Execution error: {}", e))
            })?;
        Ok(())
    }
}
impl sakaloka_secure::tokens::rotation::RefreshStore for SurrealClient {
    async fn find_token(
        &self,
        hash: &str,
    ) -> Result<
        Option<sakaloka_secure::tokens::rotation::RefreshTokenRecord>,
        sakaloka_secure::error::SecureError,
    > {
        let mut result = self
            .db
            .query("SELECT session_id, expires_at, rotated_at FROM refresh_token WHERE token_hash = $hash LIMIT 1")
            .bind(("hash", hash.to_string()))
            .await
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e)))?;

        #[derive(serde::Deserialize, SurrealValueMacro)]
        struct TokenRow {
            session_id: surrealdb_types::RecordId,
            expires_at: surrealdb_types::Datetime,
            rotated_at: Option<surrealdb_types::Datetime>,
        }

        let token: Option<TokenRow> = result.take(0).map_err(|e| {
            sakaloka_secure::error::SecureError::JwtDecode(format!("Query error: {}", e))
        })?;

        match token {
            None => Ok(None),
            Some(t) => {
                let expires_at: chrono::DateTime<chrono::Utc> = t.expires_at.into();
                let rotated_at: Option<chrono::DateTime<chrono::Utc>> =
                    t.rotated_at.map(|r| r.into());
                let sid_raw = match &t.session_id.key {
                    surrealdb_types::RecordIdKey::String(s) => s.clone(),
                    surrealdb_types::RecordIdKey::Uuid(u) => u.to_string(),
                    _ => format!("{:?}", t.session_id.key),
                };
                let session_id = sakaloka_secure::newtypes::SessionId::new_with_raw(&sid_raw)
                    .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(e.to_string()))?;
                Ok(Some(
                    sakaloka_secure::tokens::rotation::RefreshTokenRecord {
                        session_id,
                        expires_at: expires_at.timestamp() as u64,
                        rotated_at: rotated_at.map(|r| r.timestamp() as u64),
                    },
                ))
            }
        }
    }

    async fn rotate_token(
        &self,
        old_hash: &str,
        new_hash: &str,
        session_id: &sakaloka_secure::newtypes::SessionId,
        expires_at: u64,
    ) -> Result<(), sakaloka_secure::error::SecureError> {
        let sid = surrealdb_types::RecordIdKey::String(session_id.to_record_id_string());
        let session_thing = surrealdb_types::RecordId::new("session", sid);
        let expiry = chrono::DateTime::from_timestamp(expires_at as i64, 0).unwrap_or_default();

        self.db.query("
            BEGIN TRANSACTION;
            UPDATE refresh_token SET rotated_at = time::now() WHERE token_hash = $old_hash;
            INSERT INTO refresh_token (session_id, token_hash, expires_at) VALUES ($session_id, $new_hash, $expires_at);
            COMMIT TRANSACTION;
        ")
            .bind(("old_hash", old_hash.to_string()))
            .bind(("session_id", session_thing))
            .bind(("new_hash", new_hash.to_string()))
            .bind(("expires_at", expiry))
            .await
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e)))?
            .check()
            .map_err(|e| sakaloka_secure::error::SecureError::JwtDecode(format!("Execution error: {}", e)))?;

        Ok(())
    }

    async fn terminate_session(
        &self,
        session_id: &sakaloka_secure::newtypes::SessionId,
    ) -> Result<(), sakaloka_secure::error::SecureError> {
        let sid = surrealdb_types::RecordIdKey::String(session_id.to_record_id_string());
        let session_thing = surrealdb_types::RecordId::new("session", sid);

        self.db
            .query("DELETE $session_id")
            .bind(("session_id", session_thing))
            .await
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("DB error: {}", e))
            })?
            .check()
            .map_err(|e| {
                sakaloka_secure::error::SecureError::JwtDecode(format!("Execution error: {}", e))
            })?;

        Ok(())
    }
}
