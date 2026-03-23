//! SeaORM entity for the `users` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// User entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// User email address.
    #[sea_orm(unique)]
    pub email: String,
    /// User full name.
    pub full_name: String,
    /// Hashed password.
    pub password_hash: String,
    /// Organization this user belongs to.
    pub organization_id: Uuid,
    /// Role assigned to this user.
    pub role_id: Uuid,
    /// Whether the user is active.
    pub is_active: bool,
    /// Timestamp of last login.
    pub last_login_at: Option<DateTimeWithTimeZone>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
    /// Timestamp when the record was last updated.
    pub updated_at: DateTimeWithTimeZone,
}

/// Relations for the users entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// A user belongs to an organization.
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id"
    )]
    Organization,
    /// A user belongs to a role.
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RoleId",
        to = "super::roles::Column::Id"
    )]
    Role,
    /// A user has many sessions.
    #[sea_orm(has_many = "super::sessions::Entity")]
    Sessions,
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
