use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_organizations::Organizations;
use crate::m20240101_000008_create_products::Products;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(InventoryItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(InventoryItems::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(InventoryItems::ProductId).uuid().not_null())
                    .col(ColumnDef::new(InventoryItems::LocationId).uuid())
                    .col(
                        ColumnDef::new(InventoryItems::QuantityOnHand)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(InventoryItems::QuantityReserved)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(InventoryItems::QuantityAvailable)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(InventoryItems::MinStockLevel).integer())
                    .col(ColumnDef::new(InventoryItems::MaxStockLevel).integer())
                    .col(ColumnDef::new(InventoryItems::ReorderPoint).integer())
                    .col(ColumnDef::new(InventoryItems::ReorderQuantity).integer())
                    .col(ColumnDef::new(InventoryItems::AverageCost).big_integer())
                    .col(ColumnDef::new(InventoryItems::LastCost).big_integer())
                    .col(
                        ColumnDef::new(InventoryItems::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(InventoryItems::LastMovementAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(InventoryItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(InventoryItems::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_items_product_id")
                            .from(InventoryItems::Table, InventoryItems::ProductId)
                            .to(Products::Table, Products::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_inventory_items_organization_id")
                            .from(InventoryItems::Table, InventoryItems::OrganizationId)
                            .to(Organizations::Table, Organizations::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_inventory_items_product_organization")
                    .table(InventoryItems::Table)
                    .col(InventoryItems::ProductId)
                    .col(InventoryItems::OrganizationId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InventoryItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum InventoryItems {
    Table,
    Id,
    ProductId,
    LocationId,
    QuantityOnHand,
    QuantityReserved,
    QuantityAvailable,
    MinStockLevel,
    MaxStockLevel,
    ReorderPoint,
    ReorderQuantity,
    AverageCost,
    LastCost,
    OrganizationId,
    LastMovementAt,
    CreatedAt,
    UpdatedAt,
}
