//! SeaORM entity for the `sessions` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Session entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// User who owns this session.
    pub user_id: Uuid,
    /// Organization context for this session.
    pub organization_id: Uuid,
    /// Hashed session token.
    pub token_hash: String,
    /// Optional user agent string.
    pub user_agent: Option<String>,
    /// Optional IP address.
    pub ip_address: Option<String>,
    /// When the session expires.
    pub expires_at: DateTimeWithTimeZone,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
}

/// Relations for the sessions entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// A session belongs to a user.
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
    /// A session belongs to an organization.
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id"
    )]
    Organization,
    /// A session has many refresh tokens.
    #[sea_orm(has_many = "super::refresh_tokens::Entity")]
    RefreshTokens,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::refresh_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshTokens.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
