use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "purchase_order_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub purchase_order_id: i32,
    pub product_id: i32,
    pub quantity_ordered: i32,
    pub quantity_received: i32,
    pub unit_price: Decimal,
    pub created_at: ChronoDateTimeWithTimeZone,
    pub updated_at: ChronoDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::purchase_orders::Entity",
        from = "Column::PurchaseOrderId",
        to = "super::purchase_orders::Column::Id"
    )]
    PurchaseOrder,
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id"
    )]
    Product,
}

impl Related<super::purchase_orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PurchaseOrder.def()
    }
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
