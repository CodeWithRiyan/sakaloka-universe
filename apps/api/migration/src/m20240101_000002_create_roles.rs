use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_organizations::Organizations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Roles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .col(ColumnDef::new(Roles::OrganizationId).uuid().not_null())
                    .col(
                        ColumnDef::new(Roles::IsSystemRole)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Roles::Permissions)
                            .json_binary()
                            .not_null()
                            .default("{}"),
                    )
                    .col(ColumnDef::new(Roles::CreatedBy).uuid())
                    .col(
                        ColumnDef::new(Roles::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Roles::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Roles::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_roles_organization_id")
                            .from(Roles::Table, Roles::OrganizationId)
                            .to(Organizations::Table, Organizations::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Roles {
    Table,
    Id,
    Name,
    OrganizationId,
    IsSystemRole,
    Permissions,
    CreatedBy,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
