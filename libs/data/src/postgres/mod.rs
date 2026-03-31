//! PostgreSQL database client for Sakaloka-Universe.
//!
//! Wraps [`sqlx::PgPool`] and exposes typed query methods that return
//! domain models from [`sakaloka_core::models`]. Uses runtime SQL queries
//! (no compile-time checking) for CI friendliness.

mod brands;
mod categories;
mod inventory;
mod orders;
mod organizations;
mod products;
mod roles;
mod rows;
mod sessions;
mod users;

pub use rows::PgError;

/// PostgreSQL client backed by a connection pool.
#[derive(Debug, Clone)]
pub struct PgClient {
    /// The underlying connection pool.
    pub pool: sqlx::PgPool,
}

impl PgClient {
    /// Creates a new [`PgClient`] from a `DATABASE_URL`.
    ///
    /// Configures a connection pool with sensible defaults for VPS workloads:
    /// `max_connections = 20`, `min_connections = 5`, `acquire_timeout = 5s`.
    ///
    /// # Errors
    ///
    /// Returns [`PgError`] if the pool cannot be created.
    pub async fn connect(database_url: &str) -> Result<Self, PgError> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .idle_timeout(std::time::Duration::from_secs(600))
            .connect(database_url)
            .await
            .map_err(|e| PgError::Connection(e.to_string()))?;
        Ok(Self { pool })
    }

    /// Runs the embedded SQL migrations.
    ///
    /// # Errors
    ///
    /// Returns [`PgError`] if any migration fails.
    pub async fn run_migrations(&self) -> Result<(), PgError> {
        sqlx::migrate!("src/postgres/../../migrations")
            .run(&self.pool)
            .await
            .map_err(|e| PgError::Migration(e.to_string()))
    }
}
