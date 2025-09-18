use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter, JoinType, QuerySelect, RelationTrait};
use crate::entities::{order_items, orders};

pub struct OrderItemRepository;

impl OrderItemRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<order_items::Model>, DbErr> {
        order_items::Entity::find().all(db).await
    }

    pub async fn get_all_by_store(db: &DatabaseConnection, store_id: i32) -> Result<Vec<order_items::Model>, DbErr> {
        order_items::Entity::find()
            .join(JoinType::InnerJoin, order_items::Relation::Order.def())
            .filter(orders::Column::StoreId.eq(store_id))
            .all(db)
            .await
    }

    pub async fn get_all_by_employee(db: &DatabaseConnection, employee_id: i32) -> Result<Vec<order_items::Model>, DbErr> {
        order_items::Entity::find()
            .join(JoinType::InnerJoin, order_items::Relation::Order.def())
            .filter(orders::Column::EmployeeId.eq(employee_id))
            .all(db)
            .await
    }

    pub async fn create(db: &DatabaseConnection, new_order_item: order_items::CreateOrderItem) -> Result<order_items::Model, DbErr> {
        let order_item = order_items::ActiveModel {
            order_id: ActiveValue::Set(new_order_item.order_id),
            product_id: ActiveValue::Set(new_order_item.product_id),
            quantity: ActiveValue::Set(new_order_item.quantity),
            unit_price: ActiveValue::Set(new_order_item.unit_price),
            ..Default::default()
        };
        order_item.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<order_items::Model>, DbErr> {
        order_items::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: order_items::UpdateOrderItem) -> Result<Option<order_items::Model>, DbErr> {
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

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
        let res = order_items::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
