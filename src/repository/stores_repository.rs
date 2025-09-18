use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue};
use crate::entities::stores;

pub struct StoreRepository;

impl StoreRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<stores::Model>, DbErr> {
        stores::Entity::find().all(db).await
    }

    pub async fn create(db: &DatabaseConnection, new_store: stores::CreateStore) -> Result<stores::Model, DbErr> {
        let store = stores::ActiveModel {
            name: ActiveValue::Set(new_store.name),
            address: ActiveValue::Set(new_store.address),
            phone: ActiveValue::Set(new_store.phone),
            ..Default::default()
        };
        store.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<stores::Model>, DbErr> {
        stores::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: stores::UpdateStore) -> Result<Option<stores::Model>, DbErr> {
        let store: Option<stores::Model> = stores::Entity::find_by_id(id).one(db).await?;
        if let Some(store) = store {
            let mut active_model: stores::ActiveModel = store.into();
            if let Some(name) = update_data.name {
                active_model.name = ActiveValue::Set(name);
            }
            if let Some(address) = update_data.address {
                active_model.address = ActiveValue::Set(Some(address));
            }
            if let Some(phone) = update_data.phone {
                active_model.phone = ActiveValue::Set(Some(phone));
            }
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
        let res = stores::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
