use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue};
use crate::entities::products;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<products::Model>, DbErr> {
        products::Entity::find().all(db).await
    }

    pub async fn create(db: &DatabaseConnection, new_product: products::CreateProduct) -> Result<products::Model, DbErr> {
        let product = products::ActiveModel {
            name: ActiveValue::Set(new_product.name),
            description: ActiveValue::Set(new_product.description),
            price: ActiveValue::Set(new_product.price),
            sku: ActiveValue::Set(new_product.sku),
            category_id: ActiveValue::Set(new_product.category_id),
            supplier_id: ActiveValue::Set(new_product.supplier_id),
            ..Default::default()
        };
        product.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<products::Model>, DbErr> {
        products::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: products::UpdateProduct) -> Result<Option<products::Model>, DbErr> {
        let product: Option<products::Model> = products::Entity::find_by_id(id).one(db).await?;
        if let Some(product) = product {
            let mut active_model: products::ActiveModel = product.into();
            if let Some(name) = update_data.name {
                active_model.name = ActiveValue::Set(name);
            }
            if let Some(description) = update_data.description {
                active_model.description = ActiveValue::Set(Some(description));
            }
            if let Some(price) = update_data.price {
                active_model.price = ActiveValue::Set(price);
            }
            if let Some(sku) = update_data.sku {
                active_model.sku = ActiveValue::Set(sku);
            }
            if let Some(category_id) = update_data.category_id {
                active_model.category_id = ActiveValue::Set(category_id);
            }
            if let Some(supplier_id) = update_data.supplier_id {
                active_model.supplier_id = ActiveValue::Set(supplier_id);
            }
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
        let res = products::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}
