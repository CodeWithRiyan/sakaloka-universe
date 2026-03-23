//! SeaORM entity for the `roles` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Role entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Role name.
    pub name: String,
    /// Organization this role belongs to.
    pub organization_id: Uuid,
    /// Whether this is a system-defined role.
    pub is_system_role: bool,
    /// JSON permissions for this role.
    pub permissions: Json,
    /// User who created this role.
    pub created_by: Option<Uuid>,
    /// Whether the role is active.
    pub is_active: bool,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
    /// Timestamp when the record was last updated.
    pub updated_at: DateTimeWithTimeZone,
}

/// Relations for the roles entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// A role belongs to an organization.
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id"
    )]
    Organization,
    /// A role has many users.
    #[sea_orm(has_many = "super::users::Entity")]
    Users,
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
