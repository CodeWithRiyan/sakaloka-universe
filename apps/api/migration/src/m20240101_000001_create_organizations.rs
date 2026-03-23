use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organizations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organizations::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Organizations::Name).string().not_null())
                    .col(
                        ColumnDef::new(Organizations::Type)
                            .string()
                            .not_null()
                            .default("company"),
                    )
                    .col(ColumnDef::new(Organizations::Code).string().unique_key())
                    .col(ColumnDef::new(Organizations::Description).text())
                    .col(ColumnDef::new(Organizations::ParentId).uuid())
                    .col(ColumnDef::new(Organizations::Email).string())
                    .col(ColumnDef::new(Organizations::Phone).string())
                    .col(ColumnDef::new(Organizations::Website).string())
                    .col(ColumnDef::new(Organizations::Address).text())
                    .col(ColumnDef::new(Organizations::City).string())
                    .col(ColumnDef::new(Organizations::State).string())
                    .col(ColumnDef::new(Organizations::Country).string())
                    .col(ColumnDef::new(Organizations::PostalCode).string())
                    .col(ColumnDef::new(Organizations::TaxNumber).string())
                    .col(ColumnDef::new(Organizations::RegistrationNumber).string())
                    .col(ColumnDef::new(Organizations::Logo).string())
                    .col(ColumnDef::new(Organizations::Settings).json_binary())
                    .col(
                        ColumnDef::new(Organizations::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Organizations::OwnerId).uuid())
                    .col(
                        ColumnDef::new(Organizations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Organizations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organizations_parent_id")
                            .from(Organizations::Table, Organizations::ParentId)
                            .to(Organizations::Table, Organizations::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organizations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Organizations {
    Table,
    Id,
    Name,
    #[sea_orm(iden = "type")]
    Type,
    Code,
    Description,
    ParentId,
    Email,
    Phone,
    Website,
    Address,
    City,
    State,
    Country,
    PostalCode,
    TaxNumber,
    RegistrationNumber,
    Logo,
    Settings,
    IsActive,
    OwnerId,
    CreatedAt,
    UpdatedAt,
}
