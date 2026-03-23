//! SeaORM entity for the `products` table.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Product entity model.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "products")]
pub struct Model {
    /// Primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// Product name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Stock keeping unit.
    #[sea_orm(unique)]
    pub sku: String,
    /// Optional barcode.
    pub barcode: Option<String>,
    /// Base selling price in smallest currency unit.
    pub base_price: i64,
    /// Optional cost price in smallest currency unit.
    pub cost_price: Option<i64>,
    /// Optional category ID.
    pub category_id: Option<Uuid>,
    /// Optional brand ID.
    pub brand_id: Option<Uuid>,
    /// Optional image URL.
    pub image_url: Option<String>,
    /// Optional weight in kilograms.
    pub weight: Option<f64>,
    /// Optional JSON dimensions.
    pub dimensions: Option<Json>,
    /// Whether inventory tracking is enabled.
    pub track_inventory: bool,
    /// Optional minimum stock level threshold.
    pub min_stock_level: Option<i32>,
    /// Whether the product is featured.
    pub is_featured: bool,
    /// Optional JSON tags.
    pub tags: Option<Json>,
    /// Organization this product belongs to.
    pub organization_id: Uuid,
    /// User who created this product.
    pub created_by: Option<Uuid>,
    /// Soft delete timestamp.
    pub deleted_at: Option<DateTimeWithTimeZone>,
    /// Timestamp when the record was created.
    pub created_at: DateTimeWithTimeZone,
    /// Timestamp when the record was last updated.
    pub updated_at: DateTimeWithTimeZone,
}

/// Relations for the products entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// A product belongs to an organization.
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrganizationId",
        to = "super::organizations::Column::Id"
    )]
    Organization,
    /// A product optionally belongs to a category.
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id"
    )]
    Category,
    /// A product optionally belongs to a brand.
    #[sea_orm(
        belongs_to = "super::brands::Entity",
        from = "Column::BrandId",
        to = "super::brands::Column::Id"
    )]
    Brand,
    /// A product has many order items.
    #[sea_orm(has_many = "super::order_items::Entity")]
    OrderItems,
    /// A product has many inventory items.
    #[sea_orm(has_many = "super::inventory_items::Entity")]
    InventoryItems,
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::brands::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Brand.def()
    }
}

impl Related<super::order_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItems.def()
    }
}

impl Related<super::inventory_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InventoryItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
