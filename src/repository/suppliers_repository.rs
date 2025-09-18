use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue};
use crate::entities::suppliers;

pub struct SupplierRepository;

impl SupplierRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<suppliers::Model>, DbErr> {
        suppliers::Entity::find().all(db).await
    }

    pub async fn create(db: &DatabaseConnection, new_supplier: suppliers::CreateSupplier) -> Result<suppliers::Model, DbErr> {
        let supplier = suppliers::ActiveModel {
            name: ActiveValue::Set(new_supplier.name),
            contact_person: ActiveValue::Set(new_supplier.contact_person),
            email: ActiveValue::Set(new_supplier.email),
            phone: ActiveValue::Set(new_supplier.phone),
            address: ActiveValue::Set(new_supplier.address),
            ..Default::default()
        };
        supplier.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<suppliers::Model>, DbErr> {
        suppliers::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: suppliers::UpdateSupplier) -> Result<Option<suppliers::Model>, DbErr> {
        let supplier: Option<suppliers::Model> = suppliers::Entity::find_by_id(id).one(db).await?;
        if let Some(supplier) = supplier {
            let mut active_model: suppliers::ActiveModel = supplier.into();
            if let Some(name) = update_data.name {
                active_model.name = ActiveValue::Set(name);
            }
            if let Some(contact_person) = update_data.contact_person {
                active_model.contact_person = ActiveValue::Set(Some(contact_person));
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
        let res = suppliers::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}