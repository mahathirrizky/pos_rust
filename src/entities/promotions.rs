use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePromotion {
    pub name: String,
    pub description: Option<String>,
    pub promotion_type: String,
    pub value: Decimal,
    pub start_date: DateTimeUtc,
    pub end_date: DateTimeUtc,
    pub is_active: Option<bool>,
    pub product_id: Option<i32>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct UpdatePromotion {
    pub name: Option<String>,
    pub description: Option<String>,
    pub promotion_type: Option<String>,
    pub value: Option<Decimal>,
    pub start_date: Option<DateTimeUtc>,
    pub end_date: Option<DateTimeUtc>,
    pub is_active: Option<bool>,
    pub product_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "promotions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub promotion_type: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub value: Decimal,
    pub start_date: DateTimeUtc,
    pub end_date: DateTimeUtc,
    pub is_active: bool,
    pub product_id: Option<i32>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id"
    )]
    Product,
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
