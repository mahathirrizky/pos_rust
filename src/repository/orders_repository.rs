use sea_orm::{DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter, ConnectionTrait, prelude::Decimal};
use crate::entities::{orders, order_items};
use chrono::{Utc, DateTime};

pub struct OrderRepository;

impl OrderRepository {
    pub async fn get_all<C>(db: &C) -> Result<Vec<orders::Model>, DbErr> where C: ConnectionTrait {
        orders::Entity::find().all(db).await
    }

    pub async fn get_all_by_store<C>(db: &C, store_id: i32) -> Result<Vec<orders::Model>, DbErr> where C: ConnectionTrait {
        orders::Entity::find()
            .filter(orders::Column::StoreId.eq(store_id))
            .all(db)
            .await
    }

    

    pub async fn get_all_by_employee_and_store<C>(db: &C, employee_id: i32, store_id: i32) -> Result<Vec<orders::Model>, DbErr> where C: ConnectionTrait {
        orders::Entity::find()
            .filter(orders::Column::EmployeeId.eq(employee_id))
            .filter(orders::Column::StoreId.eq(store_id))
            .all(db)
            .await
    }

    pub async fn create<C>(db: &C, customer_id: i32, employee_id: i32, store_id: i32, total_amount: Decimal, status: String, items: Vec<order_items::ActiveModel>) -> Result<orders::Model, DbErr> where C: ConnectionTrait {
        let now: DateTime<Utc> = Utc::now();
        let order = orders::ActiveModel {
            customer_id: ActiveValue::Set(customer_id),
            employee_id: ActiveValue::Set(employee_id),
            store_id: ActiveValue::Set(store_id),
            order_date: ActiveValue::Set(now),
            total_amount: ActiveValue::Set(total_amount),
            status: ActiveValue::Set(status),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            ..Default::default()
        };
        let inserted_order = order.insert(db).await?;

        for mut item in items {
            item.order_id = ActiveValue::Set(inserted_order.id);
            item.insert(db).await?;
        }

        Ok(inserted_order)
    }

    pub async fn find_by_id<C>(db: &C, id: i32) -> Result<Option<orders::Model>, DbErr> where C: ConnectionTrait {
        orders::Entity::find_by_id(id).one(db).await
    }

    pub async fn update<C>(db: &C, id: i32, update_data: orders::UpdateOrder) -> Result<Option<orders::Model>, DbErr> where C: ConnectionTrait {
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
            active_model.updated_at = ActiveValue::Set(Utc::now());
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete<C>(db: &C, id: i32) -> Result<u64, DbErr> where C: ConnectionTrait {
        let res = orders::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
