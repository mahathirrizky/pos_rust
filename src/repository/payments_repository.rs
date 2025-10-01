use sea_orm::{DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter, JoinType, QuerySelect, RelationTrait, ConnectionTrait, DatabaseTransaction};
use crate::entities::{payments, orders};
use chrono::{Utc, DateTime};

pub struct PaymentRepository;

impl PaymentRepository {
    pub async fn get_all<C: ConnectionTrait>(db: &C) -> Result<Vec<payments::Model>, DbErr> {
        payments::Entity::find().all(db).await
    }

    pub async fn get_all_by_store<C: ConnectionTrait>(db: &C, store_id: i32) -> Result<Vec<payments::Model>, DbErr> {
        payments::Entity::find()
            .join(JoinType::InnerJoin, payments::Relation::Orders.def())
            .filter(orders::Column::StoreId.eq(store_id))
            .all(db)
            .await
    }

    pub async fn get_all_by_employee<C: ConnectionTrait>(db: &C, employee_id: i32) -> Result<Vec<payments::Model>, DbErr> {
        payments::Entity::find()
            .join(JoinType::InnerJoin, payments::Relation::Orders.def())
            .filter(orders::Column::EmployeeId.eq(employee_id))
            .all(db)
            .await
    }

    pub async fn create<C: ConnectionTrait>(db: &C, new_payment: payments::CreatePayment) -> Result<payments::Model, DbErr> {
        let now: DateTime<Utc> = Utc::now();
        let payment = payments::ActiveModel {
            order_id: ActiveValue::Set(new_payment.order_id),
            payment_method: ActiveValue::Set(new_payment.payment_method),
            amount: ActiveValue::Set(new_payment.amount),
            payment_date: ActiveValue::Set(new_payment.payment_date),
            status: ActiveValue::Set(new_payment.status),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            ..Default::default()
        };
        payment.insert(db).await
    }

    pub async fn create_in_txn(db: &DatabaseTransaction, new_payment: payments::CreatePayment) -> Result<payments::Model, DbErr> {
        let now: DateTime<Utc> = Utc::now();
        let payment = payments::ActiveModel {
            order_id: ActiveValue::Set(new_payment.order_id),
            payment_method: ActiveValue::Set(new_payment.payment_method),
            amount: ActiveValue::Set(new_payment.amount),
            payment_date: ActiveValue::Set(new_payment.payment_date),
            status: ActiveValue::Set(new_payment.status),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            ..Default::default()
        };
        payment.insert(db).await
    }

    pub async fn find_by_id<C: ConnectionTrait>(db: &C, id: i32) -> Result<Option<payments::Model>, DbErr> {
        payments::Entity::find_by_id(id).one(db).await
    }

    pub async fn update<C: ConnectionTrait>(db: &C, id: i32, update_data: payments::UpdatePayment) -> Result<Option<payments::Model>, DbErr> {
        let payment: Option<payments::Model> = payments::Entity::find_by_id(id).one(db).await?;
        if let Some(payment) = payment {
            let mut active_model: payments::ActiveModel = payment.into();
            if let Some(order_id) = update_data.order_id {
                active_model.order_id = ActiveValue::Set(order_id);
            }
            if let Some(payment_method) = update_data.payment_method {
                active_model.payment_method = ActiveValue::Set(payment_method);
            }
            if let Some(amount) = update_data.amount {
                active_model.amount = ActiveValue::Set(amount);
            }
            if let Some(payment_date) = update_data.payment_date {
                active_model.payment_date = ActiveValue::Set(payment_date);
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

    pub async fn delete<C: ConnectionTrait>(db: &C, id: i32) -> Result<u64, DbErr> {
        let res = payments::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
