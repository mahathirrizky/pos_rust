use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRefund {
    pub order_id: i32,
    pub reason: String,
    pub items: Vec<CreateRefundItem>,
}

#[derive(Deserialize)]
pub struct CreateRefundItem {
    pub order_item_id: i32,
    pub quantity: i32,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "refunds")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: i32,
    pub employee_id: i32,
    pub store_id: i32,
    pub reason: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub total_amount: Decimal,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::orders::Entity",
        from = "Column::OrderId",
        to = "super::orders::Column::Id"
    )]
    Order,
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
    #[sea_orm(has_many = "super::refund_items::Entity")]
    RefundItem,
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
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

impl Related<super::refund_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefundItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
