use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_organizations::Organizations;
use crate::m20240101_000003_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Orders::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Orders::OrderNumber)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Orders::CustomerId).uuid())
                    .col(ColumnDef::new(Orders::OrganizationId).uuid().not_null())
                    .col(
                        ColumnDef::new(Orders::Status)
                            .string()
                            .not_null()
                            .default("draft"),
                    )
                    .col(
                        ColumnDef::new(Orders::Type)
                            .string()
                            .not_null()
                            .default("dine_in"),
                    )
                    .col(
                        ColumnDef::new(Orders::Subtotal)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Orders::TaxAmount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Orders::DiscountAmount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Orders::TotalAmount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Orders::PaymentMethod).string())
                    .col(
                        ColumnDef::new(Orders::PaymentStatus)
                            .string()
                            .not_null()
                            .default("unpaid"),
                    )
                    .col(
                        ColumnDef::new(Orders::PaidAmount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Orders::Notes).text())
                    .col(ColumnDef::new(Orders::TableNumber).string())
                    .col(ColumnDef::new(Orders::CustomerName).string())
                    .col(ColumnDef::new(Orders::CreatedBy).uuid())
                    .col(ColumnDef::new(Orders::UpdatedBy).uuid())
                    .col(
                        ColumnDef::new(Orders::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Orders::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_customer_id")
                            .from(Orders::Table, Orders::CustomerId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_organization_id")
                            .from(Orders::Table, Orders::OrganizationId)
                            .to(Organizations::Table, Organizations::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_created_by")
                            .from(Orders::Table, Orders::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_updated_by")
                            .from(Orders::Table, Orders::UpdatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Orders {
    Table,
    Id,
    OrderNumber,
    CustomerId,
    OrganizationId,
    Status,
    #[sea_orm(iden = "type")]
    Type,
    Subtotal,
    TaxAmount,
    DiscountAmount,
    TotalAmount,
    PaymentMethod,
    PaymentStatus,
    PaidAmount,
    Notes,
    TableNumber,
    CustomerName,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
}
