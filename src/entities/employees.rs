use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateEmployee {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub store_id: i32,
    pub role: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct UpdateEmployee {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub store_id: Option<i32>,
    pub role: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "employees")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(nullable)]
    pub phone: Option<String>,
    pub store_id: i32,
    pub role: String,
    pub password_hash: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::stores::Entity",
        from = "Column::StoreId",
        to = "super::stores::Column::Id"
    )]
    Store,
}

impl Related<super::stores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
