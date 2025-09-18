use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter};
use crate::entities::inventory;

pub struct InventoryRepository;

impl InventoryRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<inventory::Model>, DbErr> {
        inventory::Entity::find().all(db).await
    }

    pub async fn get_all_by_store(db: &DatabaseConnection, store_id: i32) -> Result<Vec<inventory::Model>, DbErr> {
        inventory::Entity::find()
            .filter(inventory::Column::StoreId.eq(store_id))
            .all(db).await
    }

    pub async fn create(db: &DatabaseConnection, new_inventory: inventory::CreateInventory) -> Result<inventory::Model, DbErr> {
        let inventory = inventory::ActiveModel {
            product_id: ActiveValue::Set(new_inventory.product_id),
            store_id: ActiveValue::Set(new_inventory.store_id),
            quantity: ActiveValue::Set(new_inventory.quantity),
            last_restocked: ActiveValue::Set(new_inventory.last_restocked),
            ..Default::default()
        };
        inventory.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<inventory::Model>, DbErr> {
        inventory::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: inventory::UpdateInventory) -> Result<Option<inventory::Model>, DbErr> {
        let inventory: Option<inventory::Model> = inventory::Entity::find_by_id(id).one(db).await?;
        if let Some(inventory) = inventory {
            let mut active_model: inventory::ActiveModel = inventory.into();
            if let Some(product_id) = update_data.product_id {
                active_model.product_id = ActiveValue::Set(product_id);
            }
            if let Some(store_id) = update_data.store_id {
                active_model.store_id = ActiveValue::Set(store_id);
            }
            if let Some(quantity) = update_data.quantity {
                active_model.quantity = ActiveValue::Set(quantity);
            }
            if let Some(last_restocked) = update_data.last_restocked {
                active_model.last_restocked = ActiveValue::Set(Some(last_restocked));
            }
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
        let res = inventory::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
