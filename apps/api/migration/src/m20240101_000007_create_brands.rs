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
                    .table(Brands::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Brands::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Brands::Name).string().not_null())
                    .col(ColumnDef::new(Brands::Slug).string().not_null())
                    .col(ColumnDef::new(Brands::Description).text())
                    .col(ColumnDef::new(Brands::Logo).string())
                    .col(ColumnDef::new(Brands::Website).string())
                    .col(ColumnDef::new(Brands::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Brands::CreatedBy).uuid())
                    .col(
                        ColumnDef::new(Brands::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Brands::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_brands_organization_id")
                            .from(Brands::Table, Brands::OrganizationId)
                            .to(Organizations::Table, Organizations::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_brands_created_by")
                            .from(Brands::Table, Brands::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_brands_slug_organization_id")
                    .table(Brands::Table)
                    .col(Brands::Slug)
                    .col(Brands::OrganizationId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Brands::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Brands {
    Table,
    Id,
    Name,
    Slug,
    Description,
    Logo,
    Website,
    OrganizationId,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
