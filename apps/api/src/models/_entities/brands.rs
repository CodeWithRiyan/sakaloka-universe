//! SeaORM entity for the `brands` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Brand entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "brands")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Brand name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional logo URL.
    pub logo: Option<String>,
    /// Optional website URL.
    pub website: Option<String>,
    /// Organization this brand belongs to.
    pub organization_id: Uuid,
    /// User who created this brand.
    pub created_by: Option<Uuid>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
    /// Timestamp when the record was last updated.
    pub updated_at: DateTimeWithTimeZone,
}

/// Relations for the brands entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// A brand belongs to an organization.
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id"
    )]
    Organization,
    /// A brand has many products.
    #[sea_orm(has_many = "super::products::Entity")]
    Products,
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Products.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
