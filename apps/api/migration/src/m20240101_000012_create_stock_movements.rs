use sea_orm_migration::prelude::*;

use crate::m20240101_000003_create_users::Users;
use crate::m20240101_000011_create_inventory_items::InventoryItems;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockMovements::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockMovements::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::InventoryItemId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::MovementType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockMovements::Reason).string().not_null())
                    .col(ColumnDef::new(StockMovements::Reference).string())
                    .col(ColumnDef::new(StockMovements::Notes).text())
                    .col(
                        ColumnDef::new(StockMovements::TotalBefore)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::QuantityChange)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::TotalAfter)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockMovements::CreatedBy).uuid())
                    .col(
                        ColumnDef::new(StockMovements::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_movements_inventory_item_id")
                            .from(StockMovements::Table, StockMovements::InventoryItemId)
                            .to(InventoryItems::Table, InventoryItems::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_movements_created_by")
                            .from(StockMovements::Table, StockMovements::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockMovements::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StockMovements {
    Table,
    Id,
    InventoryItemId,
    MovementType,
    Quantity,
    Reason,
    Reference,
    Notes,
    TotalBefore,
    QuantityChange,
    TotalAfter,
    CreatedBy,
    CreatedAt,
}
