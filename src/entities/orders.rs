use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateOrder {
    pub customer_id: i32,
    pub employee_id: i32,
    pub store_id: i32,
    pub order_date: DateTimeUtc,
    pub total_amount: Decimal,
    pub status: String,
}

#[derive(Deserialize)]
pub struct UpdateOrder {
    pub customer_id: Option<i32>,
    pub employee_id: Option<i32>,
    pub store_id: Option<i32>,
    pub order_date: Option<DateTimeUtc>,
    pub total_amount: Option<Decimal>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub customer_id: i32,
    pub employee_id: i32,
    pub store_id: i32,
    pub order_date: DateTimeUtc,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub total_amount: Decimal,
    pub status: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customers::Entity",
        from = "Column::CustomerId",
        to = "super::customers::Column::Id"
    )]
    Customer,
    #[sea_orm(
        belongs_to = "super::employees::Entity",
        from = "Column::EmployeeId",
        to = "super::employees::Column::Id"
    )]
    Employee,
    #[sea_orm(
        belongs_to = "super::stores::Entity",
        from = "Column::StoreId",
        to = "super::stores::Column::Id"
    )]
    Store,
    #[sea_orm(has_many = "super::order_items::Entity")]
    OrderItem,
    #[sea_orm(has_many = "super::payments::Entity")]
    Payment,
}

impl Related<super::customers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::employees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl Related<super::stores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<super::order_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItem.def()
    }
}

impl Related<super::payments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
