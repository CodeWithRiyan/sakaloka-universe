//! SeaORM database migrations for the Sakaloka API.

pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_organizations;
mod m20240101_000002_create_roles;
mod m20240101_000003_create_users;
mod m20240101_000004_create_sessions;
mod m20240101_000005_create_refresh_tokens;
mod m20240101_000006_create_categories;
mod m20240101_000007_create_brands;
mod m20240101_000008_create_products;
mod m20240101_000009_create_orders;
mod m20240101_000010_create_order_items;
mod m20240101_000011_create_inventory_items;
mod m20240101_000012_create_stock_movements;

/// Runs all database migrations in order.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_organizations::Migration),
            Box::new(m20240101_000002_create_roles::Migration),
            Box::new(m20240101_000003_create_users::Migration),
            Box::new(m20240101_000004_create_sessions::Migration),
            Box::new(m20240101_000005_create_refresh_tokens::Migration),
            Box::new(m20240101_000006_create_categories::Migration),
            Box::new(m20240101_000007_create_brands::Migration),
            Box::new(m20240101_000008_create_products::Migration),
            Box::new(m20240101_000009_create_orders::Migration),
            Box::new(m20240101_000010_create_order_items::Migration),
            Box::new(m20240101_000011_create_inventory_items::Migration),
            Box::new(m20240101_000012_create_stock_movements::Migration),
        ]
    }
}
