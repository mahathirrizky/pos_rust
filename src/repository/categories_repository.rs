use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue};
use crate::entities::categories;
use chrono::{Utc, DateTime};

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<categories::Model>, DbErr> {
        categories::Entity::find().all(db).await
    }

    pub async fn create(db: &DatabaseConnection, new_category: categories::CreateCategory) -> Result<categories::Model, DbErr> {
        let now: DateTime<Utc> = Utc::now();
        let category = categories::ActiveModel {
            name: ActiveValue::Set(new_category.name),
            description: ActiveValue::Set(new_category.description),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            ..Default::default()
        };
        category.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<categories::Model>, DbErr> {
        categories::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: categories::UpdateCategory) -> Result<Option<categories::Model>, DbErr> {
        let category: Option<categories::Model> = categories::Entity::find_by_id(id).one(db).await?;
        if let Some(category) = category {
            let mut active_model: categories::ActiveModel = category.into();
            if let Some(name) = update_data.name {
                active_model.name = ActiveValue::Set(name);
            }
            if let Some(description) = update_data.description {
                active_model.description = ActiveValue::Set(Some(description));
            }
            active_model.updated_at = ActiveValue::Set(Utc::now());
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
        let res = categories::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}