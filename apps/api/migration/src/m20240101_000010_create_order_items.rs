use sea_orm_migration::prelude::*;

use crate::m20240101_000008_create_products::Products;
use crate::m20240101_000009_create_orders::Orders;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItems::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderItems::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderItems::ProductId).uuid().not_null())
                    .col(ColumnDef::new(OrderItems::ItemName).string().not_null())
                    .col(ColumnDef::new(OrderItems::Quantity).integer().not_null())
                    .col(
                        ColumnDef::new(OrderItems::UnitPrice)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderItems::TotalPrice)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_items_order_id")
                            .from(OrderItems::Table, OrderItems::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_items_product_id")
                            .from(OrderItems::Table, OrderItems::ProductId)
                            .to(Products::Table, Products::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum OrderItems {
    Table,
    Id,
    OrderId,
    ProductId,
    ItemName,
    Quantity,
    UnitPrice,
    TotalPrice,
    CreatedAt,
}
