use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePayment {
    pub order_id: i32,
    pub payment_method: String,
    pub amount: Decimal,
    pub payment_date: DateTimeUtc,
    pub status: String,
}

#[derive(Deserialize)]
pub struct UpdatePayment {
    pub order_id: Option<i32>,
    pub payment_method: Option<String>,
    pub amount: Option<Decimal>,
    pub payment_date: Option<DateTimeUtc>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: i32,
    pub payment_method: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    pub payment_date: DateTimeUtc,
    pub status: String,
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
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
