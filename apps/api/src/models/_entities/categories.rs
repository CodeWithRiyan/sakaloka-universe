//! SeaORM entity for the `categories` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Category entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Category name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional parent category ID for hierarchy.
    pub parent_id: Option<Uuid>,
    /// Optional image URL.
    pub image_url: Option<String>,
    /// Organization this category belongs to.
    pub organization_id: Uuid,
    /// User who created this category.
    pub created_by: Option<Uuid>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
    /// Timestamp when the record was last updated.
    pub updated_at: DateTimeWithTimeZone,
}

/// Relations for the categories entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// A category belongs to an organization.
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id"
    )]
    Organization,
    /// Self-referential relation to parent category.
    #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
    ParentCategory,
    /// A category has many products.
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
