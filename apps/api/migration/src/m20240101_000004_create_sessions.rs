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
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Sessions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Sessions::UserId).uuid().not_null())
                    .col(ColumnDef::new(Sessions::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Sessions::TokenHash).string().not_null())
                    .col(ColumnDef::new(Sessions::UserAgent).string())
                    .col(ColumnDef::new(Sessions::IpAddress).string())
                    .col(
                        ColumnDef::new(Sessions::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Sessions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sessions_user_id")
                            .from(Sessions::Table, Sessions::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sessions_organization_id")
                            .from(Sessions::Table, Sessions::OrganizationId)
                            .to(Organizations::Table, Organizations::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sessions {
    Table,
    Id,
    UserId,
    OrganizationId,
    TokenHash,
    UserAgent,
    IpAddress,
    ExpiresAt,
    CreatedAt,
}
