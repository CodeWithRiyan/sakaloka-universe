//! SeaORM entity for the `refresh_tokens` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Refresh token entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "refresh_tokens")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Session this refresh token belongs to.
    pub session_id: Uuid,
    /// Hashed token value.
    pub token_hash: String,
    /// When the token expires.
    pub expires_at: DateTimeWithTimeZone,
    /// When the token was rotated (replaced by a new one).
    pub rotated_at: Option<DateTimeWithTimeZone>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
}

/// Relations for the refresh tokens entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// A refresh token belongs to a session.
    #[sea_orm(
        belongs_to = "super::sessions::Entity",
        from = "Column::SessionId",
        to = "super::sessions::Column::Id"
    )]
    Session,
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
