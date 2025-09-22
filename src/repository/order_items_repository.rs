use sea_orm::{DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter, JoinType, QuerySelect, RelationTrait, ConnectionTrait};
use crate::entities::{order_items, orders};

pub struct OrderItemRepository;

impl OrderItemRepository {
    pub async fn get_all<C: ConnectionTrait>(db: &C) -> Result<Vec<order_items::Model>, DbErr> {
        order_items::Entity::find().all(db).await
    }

    pub async fn get_all_by_order_id<C: ConnectionTrait>(db: &C, order_id: i32) -> Result<Vec<order_items::Model>, DbErr> {
        order_items::Entity::find()
            .filter(order_items::Column::OrderId.eq(order_id))
            .all(db)
            .await
    }

    pub async fn get_all_by_store<C: ConnectionTrait>(db: &C, store_id: i32) -> Result<Vec<order_items::Model>, DbErr> {
        order_items::Entity::find()
            .join(JoinType::InnerJoin, order_items::Relation::Order.def())
            .filter(orders::Column::StoreId.eq(store_id))
            .all(db)
            .await
    }

    pub async fn get_all_by_employee<C: ConnectionTrait>(db: &C, employee_id: i32) -> Result<Vec<order_items::Model>, DbErr> {
        order_items::Entity::find()
            .join(JoinType::InnerJoin, order_items::Relation::Order.def())
            .filter(orders::Column::EmployeeId.eq(employee_id))
            .all(db)
            .await
    }

    

    pub async fn find_by_id<C: ConnectionTrait>(db: &C, id: i32) -> Result<Option<order_items::Model>, DbErr> {
        order_items::Entity::find_by_id(id).one(db).await
    }

    pub async fn update<C: ConnectionTrait>(db: &C, id: i32, update_data: order_items::UpdateOrderItem) -> Result<Option<order_items::Model>, DbErr> {
        let order_item: Option<order_items::Model> = order_items::Entity::find_by_id(id).one(db).await?;
        if let Some(order_item) = order_item {
            let mut active_model: order_items::ActiveModel = order_item.into();
            if let Some(order_id) = update_data.order_id {
                active_model.order_id = ActiveValue::Set(order_id);
            }
            if let Some(product_id) = update_data.product_id {
                active_model.product_id = ActiveValue::Set(product_id);
            }
            if let Some(quantity) = update_data.quantity {
                active_model.quantity = ActiveValue::Set(quantity);
            }
            if let Some(unit_price) = update_data.unit_price {
                active_model.unit_price = ActiveValue::Set(unit_price);
            }
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete<C: ConnectionTrait>(db: &C, id: i32) -> Result<u64, DbErr> {
        let res = order_items::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
