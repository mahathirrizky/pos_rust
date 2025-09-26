use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStore {
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateStore {
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "stores")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub address: Option<String>,
    #[sea_orm(nullable)]
    pub phone: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::employees::Entity")]
    Employee,
    #[sea_orm(has_many = "super::orders::Entity")]
    Order,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
}

impl Related<super::employees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
