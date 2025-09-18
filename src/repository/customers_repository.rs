use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue};
use crate::entities::customers;

pub struct CustomerRepository;

impl CustomerRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<customers::Model>, DbErr> {
        customers::Entity::find().all(db).await
    }

    pub async fn create(db: &DatabaseConnection, new_customer: customers::CreateCustomer) -> Result<customers::Model, DbErr> {
        let customer = customers::ActiveModel {
            first_name: ActiveValue::Set(new_customer.first_name),
            last_name: ActiveValue::Set(new_customer.last_name),
            email: ActiveValue::Set(new_customer.email),
            phone: ActiveValue::Set(new_customer.phone),
            address: ActiveValue::Set(new_customer.address),
            ..Default::default()
        };
        customer.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<customers::Model>, DbErr> {
        customers::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: customers::UpdateCustomer) -> Result<Option<customers::Model>, DbErr> {
        let customer: Option<customers::Model> = customers::Entity::find_by_id(id).one(db).await?;
        if let Some(customer) = customer {
            let mut active_model: customers::ActiveModel = customer.into();
            if let Some(first_name) = update_data.first_name {
                active_model.first_name = ActiveValue::Set(first_name);
            }
            if let Some(last_name) = update_data.last_name {
                active_model.last_name = ActiveValue::Set(last_name);
            }
            if let Some(email) = update_data.email {
                active_model.email = ActiveValue::Set(Some(email));
            }
            if let Some(phone) = update_data.phone {
                active_model.phone = ActiveValue::Set(Some(phone));
            }
            if let Some(address) = update_data.address {
                active_model.address = ActiveValue::Set(Some(address));
            }
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
        let res = customers::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
