use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_organizations::Organizations;
use crate::m20240101_000003_create_users::Users;
use crate::m20240101_000006_create_categories::Categories;
use crate::m20240101_000007_create_brands::Brands;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Products::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Products::Name).string().not_null())
                    .col(ColumnDef::new(Products::Description).text())
                    .col(ColumnDef::new(Products::Sku).string().not_null())
                    .col(ColumnDef::new(Products::Barcode).string())
                    .col(ColumnDef::new(Products::BasePrice).big_integer().not_null())
                    .col(ColumnDef::new(Products::CostPrice).big_integer())
                    .col(ColumnDef::new(Products::CategoryId).uuid())
                    .col(ColumnDef::new(Products::BrandId).uuid())
                    .col(ColumnDef::new(Products::ImageUrl).string())
                    .col(ColumnDef::new(Products::Weight).double())
                    .col(ColumnDef::new(Products::Dimensions).json_binary())
                    .col(
                        ColumnDef::new(Products::TrackInventory)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Products::MinStockLevel).integer())
                    .col(
                        ColumnDef::new(Products::IsFeatured)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Products::Tags).json_binary())
                    .col(ColumnDef::new(Products::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Products::CreatedBy).uuid())
                    .col(ColumnDef::new(Products::DeletedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Products::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Products::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_category_id")
                            .from(Products::Table, Products::CategoryId)
                            .to(Categories::Table, Categories::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_brand_id")
                            .from(Products::Table, Products::BrandId)
                            .to(Brands::Table, Brands::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_organization_id")
                            .from(Products::Table, Products::OrganizationId)
                            .to(Organizations::Table, Organizations::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_created_by")
                            .from(Products::Table, Products::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_products_sku_organization_id")
                    .table(Products::Table)
                    .col(Products::Sku)
                    .col(Products::OrganizationId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Products {
    Table,
    Id,
    Name,
    Description,
    Sku,
    Barcode,
    BasePrice,
    CostPrice,
    CategoryId,
    BrandId,
    ImageUrl,
    Weight,
    Dimensions,
    TrackInventory,
    MinStockLevel,
    IsFeatured,
    Tags,
    OrganizationId,
    CreatedBy,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
