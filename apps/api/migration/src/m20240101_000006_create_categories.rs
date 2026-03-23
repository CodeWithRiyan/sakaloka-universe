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
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Categories::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(ColumnDef::new(Categories::Slug).string().not_null())
                    .col(ColumnDef::new(Categories::Description).text())
                    .col(ColumnDef::new(Categories::ParentId).uuid())
                    .col(ColumnDef::new(Categories::ImageUrl).string())
                    .col(ColumnDef::new(Categories::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Categories::CreatedBy).uuid())
                    .col(
                        ColumnDef::new(Categories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Categories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_categories_parent_id")
                            .from(Categories::Table, Categories::ParentId)
                            .to(Categories::Table, Categories::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_categories_organization_id")
                            .from(Categories::Table, Categories::OrganizationId)
                            .to(Organizations::Table, Organizations::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_categories_created_by")
                            .from(Categories::Table, Categories::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_categories_slug_organization_id")
                    .table(Categories::Table)
                    .col(Categories::Slug)
                    .col(Categories::OrganizationId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Categories {
    Table,
    Id,
    Name,
    Slug,
    Description,
    ParentId,
    ImageUrl,
    OrganizationId,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
