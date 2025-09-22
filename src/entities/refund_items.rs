use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "refund_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub refund_id: i32,
    pub order_item_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::refunds::Entity",
        from = "Column::RefundId",
        to = "super::refunds::Column::Id"
    )]
    Refund,
    #[sea_orm(
        belongs_to = "super::order_items::Entity",
        from = "Column::OrderItemId",
        to = "super::order_items::Column::Id"
    )]
    OrderItem,
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id"
    )]
    Product,
}

impl Related<super::refunds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Refund.def()
    }
}

impl Related<super::order_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItem.def()
    }
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
