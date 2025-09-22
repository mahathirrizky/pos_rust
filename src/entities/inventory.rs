use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateInventory {
    pub product_id: i32,
    pub store_id: i32,
    pub quantity: i32,
    pub last_restocked: Option<DateTimeUtc>,
}

#[derive(Deserialize)]
pub struct UpdateInventory {
    pub product_id: Option<i32>,
    pub store_id: Option<i32>,
    pub quantity: Option<i32>,
    pub last_restocked: Option<DateTimeUtc>,
}

#[derive(Debug, Serialize)]
pub struct InventoryReportItem {
    pub inventory_id: i32,
    pub product_id: i32,
    pub product_name: String,
    pub store_id: i32,
    pub store_name: String,
    pub quantity: i32,
    pub last_restocked: Option<DateTimeUtc>,
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, Serialize)]
pub struct InventoryReport {
    pub items: Vec<InventoryReportItem>,
    pub total_items: u64,
}

#[derive(Deserialize)]
pub struct InventoryReportQueryParams {
    pub store_id: Option<i32>,
    pub product_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "inventory")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub product_id: i32,
    pub store_id: i32,
    pub quantity: i32,
    #[sea_orm(nullable)]
    pub last_restocked: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id"
    )]
    Product,
    #[sea_orm(
        belongs_to = "super::stores::Entity",
        from = "Column::StoreId",
        to = "super::stores::Column::Id"
    )]
    Store,
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::stores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
