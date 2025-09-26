use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "purchase_orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub supplier_id: i32,
    pub store_id: i32,
    pub employee_id: Option<i32>,
    pub order_date: ChronoDateTimeWithTimeZone,
    pub expected_delivery_date: Option<ChronoDateTimeWithTimeZone>,
    pub status: String,
    pub total_amount: Decimal,
    pub created_at: ChronoDateTimeWithTimeZone,
    pub updated_at: ChronoDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::purchase_order_items::Entity")]
    PurchaseOrderItem,
    #[sea_orm(
        belongs_to = "super::suppliers::Entity",
        from = "Column::SupplierId",
        to = "super::suppliers::Column::Id"
    )]
    Supplier,
    #[sea_orm(
        belongs_to = "super::stores::Entity",
        from = "Column::StoreId",
        to = "super::stores::Column::Id"
    )]
    Store,
    #[sea_orm(
        belongs_to = "super::employees::Entity",
        from = "Column::EmployeeId",
        to = "super::employees::Column::Id"
    )]
    Employee,
}

impl Related<super::purchase_order_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PurchaseOrderItem.def()
    }
}

impl Related<super::suppliers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Supplier.def()
    }
}

impl Related<super::stores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<super::employees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
