use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// This is the new payload from the client for an order item
#[derive(Deserialize)]
pub struct CreateOrderItemPayload {
    pub product_id: i32,
    pub quantity: i32,
    pub promotion_id: Option<i32>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct UpdateOrderItem {
    pub order_id: Option<i32>,
    pub product_id: Option<i32>,
    pub quantity: Option<i32>,
    pub unit_price: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "order_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub unit_price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))", default_value = "0.00")]
    pub discount_amount: Decimal,
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
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id"
    )]
    Product,
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
