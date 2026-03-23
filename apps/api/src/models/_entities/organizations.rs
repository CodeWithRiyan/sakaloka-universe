//! SeaORM entity for the `organizations` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Organization entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "organizations")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Organization name.
    pub name: String,
    /// Organization type.
    pub r#type: String,
    /// Optional organization code.
    pub code: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional parent organization ID for hierarchy.
    pub parent_id: Option<Uuid>,
    /// Optional contact email.
    pub email: Option<String>,
    /// Optional contact phone.
    pub phone: Option<String>,
    /// Optional website URL.
    pub website: Option<String>,
    /// Optional physical address.
    pub address: Option<String>,
    /// Optional city.
    pub city: Option<String>,
    /// Optional state or province.
    pub state: Option<String>,
    /// Optional country.
    pub country: Option<String>,
    /// Optional postal code.
    pub postal_code: Option<String>,
    /// Optional tax identification number.
    pub tax_number: Option<String>,
    /// Optional business registration number.
    pub registration_number: Option<String>,
    /// Optional logo URL.
    pub logo: Option<String>,
    /// Optional JSON settings.
    pub settings: Option<Json>,
    /// Whether the organization is active.
    pub is_active: bool,
    /// Optional owner user ID.
    pub owner_id: Option<Uuid>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
    /// Timestamp when the record was last updated.
    pub updated_at: DateTimeWithTimeZone,
}

/// Relations for the organizations entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Self-referential relation to parent organization.
    #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
    ParentOrganization,
    /// An organization has many users.
    #[sea_orm(has_many = "super::users::Entity")]
    Users,
    /// An organization has many roles.
    #[sea_orm(has_many = "super::roles::Entity")]
    Roles,
    /// An organization has many categories.
    #[sea_orm(has_many = "super::categories::Entity")]
    Categories,
    /// An organization has many brands.
    #[sea_orm(has_many = "super::brands::Entity")]
    Brands,
    /// An organization has many products.
    #[sea_orm(has_many = "super::products::Entity")]
    Products,
    /// An organization has many orders.
    #[sea_orm(has_many = "super::orders::Entity")]
    Orders,
    /// An organization has many inventory items.
    #[sea_orm(has_many = "super::inventory_items::Entity")]
    InventoryItems,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::brands::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Brands.def()
    }
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Products.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::inventory_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InventoryItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
