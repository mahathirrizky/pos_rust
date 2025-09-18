use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter};
use crate::entities::orders;

pub struct OrderRepository;

impl OrderRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<orders::Model>, DbErr> {
        orders::Entity::find().all(db).await
    }

    pub async fn get_all_by_store(db: &DatabaseConnection, store_id: i32) -> Result<Vec<orders::Model>, DbErr> {
        orders::Entity::find()
            .filter(orders::Column::StoreId.eq(store_id))
            .all(db)
            .await
    }

    pub async fn get_all_by_employee(db: &DatabaseConnection, employee_id: i32) -> Result<Vec<orders::Model>, DbErr> {
        orders::Entity::find()
            .filter(orders::Column::EmployeeId.eq(employee_id))
            .all(db)
            .await
    }

    pub async fn create(db: &DatabaseConnection, new_order: orders::CreateOrder) -> Result<orders::Model, DbErr> {
        let order = orders::ActiveModel {
            customer_id: ActiveValue::Set(new_order.customer_id),
            employee_id: ActiveValue::Set(new_order.employee_id),
            store_id: ActiveValue::Set(new_order.store_id),
            order_date: ActiveValue::Set(new_order.order_date),
            total_amount: ActiveValue::Set(new_order.total_amount),
            status: ActiveValue::Set(new_order.status),
            ..Default::default()
        };
        order.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<orders::Model>, DbErr> {
        orders::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: orders::UpdateOrder) -> Result<Option<orders::Model>, DbErr> {
        let order: Option<orders::Model> = orders::Entity::find_by_id(id).one(db).await?;
        if let Some(order) = order {
            let mut active_model: orders::ActiveModel = order.into();
            if let Some(customer_id) = update_data.customer_id {
                active_model.customer_id = ActiveValue::Set(customer_id);
            }
            if let Some(employee_id) = update_data.employee_id {
                active_model.employee_id = ActiveValue::Set(employee_id);
            }
            if let Some(store_id) = update_data.store_id {
                active_model.store_id = ActiveValue::Set(store_id);
            }
            if let Some(order_date) = update_data.order_date {
                active_model.order_date = ActiveValue::Set(order_date);
            }
            if let Some(total_amount) = update_data.total_amount {
                active_model.total_amount = ActiveValue::Set(total_amount);
            }
            if let Some(status) = update_data.status {
                active_model.status = ActiveValue::Set(status);
            }
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
        let res = orders::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
