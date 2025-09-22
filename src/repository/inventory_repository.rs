use sea_orm::{DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter, ConnectionTrait};
use crate::entities::inventory;

pub struct InventoryRepository;

impl InventoryRepository {
    pub async fn get_all<C: ConnectionTrait>(db: &C) -> Result<Vec<inventory::Model>, DbErr> {
        inventory::Entity::find().all(db).await
    }

    pub async fn get_all_by_store<C: ConnectionTrait>(db: &C, store_id: i32) -> Result<Vec<inventory::Model>, DbErr> {
        inventory::Entity::find()
            .filter(inventory::Column::StoreId.eq(store_id))
            .all(db).await
    }

    pub async fn find_by_product_and_store<C: ConnectionTrait>(db: &C, product_id: i32, store_id: i32) -> Result<Option<inventory::Model>, DbErr> {
        inventory::Entity::find()
            .filter(inventory::Column::ProductId.eq(product_id))
            .filter(inventory::Column::StoreId.eq(store_id))
            .one(db)
            .await
    }

    pub async fn increase_quantity<C: ConnectionTrait>(db: &C, product_id: i32, store_id: i32, quantity_to_add: i32) -> Result<inventory::Model, DbErr> {
        let inventory_item = Self::find_by_product_and_store(db, product_id, store_id)
            .await?
            .ok_or_else(|| DbErr::Custom("Inventory item not found".to_string()))?;

        let mut active_model: inventory::ActiveModel = inventory_item.into();
        active_model.quantity = ActiveValue::Set(active_model.quantity.as_ref() + quantity_to_add);
        active_model.update(db).await
    }

    pub async fn create<C: ConnectionTrait>(db: &C, new_inventory: inventory::CreateInventory) -> Result<inventory::Model, DbErr> {
        let inventory = inventory::ActiveModel {
            product_id: ActiveValue::Set(new_inventory.product_id),
            store_id: ActiveValue::Set(new_inventory.store_id),
            quantity: ActiveValue::Set(new_inventory.quantity),
            last_restocked: ActiveValue::Set(new_inventory.last_restocked),
            ..Default::default()
        };
        inventory.insert(db).await
    }

    pub async fn find_by_id<C: ConnectionTrait>(db: &C, id: i32) -> Result<Option<inventory::Model>, DbErr> {
        inventory::Entity::find_by_id(id).one(db).await
    }

    pub async fn update<C: ConnectionTrait>(db: &C, id: i32, update_data: inventory::UpdateInventory) -> Result<Option<inventory::Model>, DbErr> {
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

    pub async fn delete<C: ConnectionTrait>(db: &C, id: i32) -> Result<u64, DbErr> {
        let res = inventory::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
