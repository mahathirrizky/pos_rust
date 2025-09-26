use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomer {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "customers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(unique, nullable)]
    pub email: Option<String>,
    #[sea_orm(nullable)]
    pub phone: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub address: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}