use sea_orm::{DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ConnectionTrait, ColumnTrait, QueryFilter};
use crate::entities::promotions;
use chrono::{Utc, DateTime};

pub struct PromotionRepository;

impl PromotionRepository {
    pub async fn create<C: ConnectionTrait>(db: &C, new_promotion: promotions::CreatePromotion) -> Result<promotions::Model, DbErr> {
        let now: DateTime<Utc> = Utc::now();
        let promo = promotions::ActiveModel {
            name: ActiveValue::Set(new_promotion.name),
            description: ActiveValue::Set(new_promotion.description),
            promotion_type: ActiveValue::Set(new_promotion.promotion_type),
            value: ActiveValue::Set(new_promotion.value),
            start_date: ActiveValue::Set(new_promotion.start_date),
            end_date: ActiveValue::Set(new_promotion.end_date),
            is_active: ActiveValue::Set(new_promotion.is_active.unwrap_or(true) as i8),
            product_id: ActiveValue::Set(new_promotion.product_id),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            ..Default::default()
        };
        promo.insert(db).await
    }

    pub async fn get_all<C: ConnectionTrait>(db: &C) -> Result<Vec<promotions::Model>, DbErr> {
        promotions::Entity::find().all(db).await
    }

    pub async fn find_by_id<C: ConnectionTrait>(db: &C, id: i32) -> Result<Option<promotions::Model>, DbErr> {
        promotions::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_by_ids<C: ConnectionTrait>(db: &C, ids: Vec<i32>) -> Result<Vec<promotions::Model>, DbErr> {
        promotions::Entity::find().filter(promotions::Column::Id.is_in(ids)).all(db).await
    }

    pub async fn update<C: ConnectionTrait>(db: &C, id: i32, update_data: promotions::UpdatePromotion) -> Result<promotions::Model, DbErr> {
        let mut promo: promotions::ActiveModel = Self::find_by_id(db, id).await?
            .ok_or(DbErr::Custom("Promotion not found".to_owned()))?
            .into();

        if let Some(name) = update_data.name {
            promo.name = ActiveValue::Set(name);
        }
        if let Some(description) = update_data.description {
            promo.description = ActiveValue::Set(Some(description));
        }
        if let Some(promotion_type) = update_data.promotion_type {
            promo.promotion_type = ActiveValue::Set(promotion_type);
        }
        if let Some(value) = update_data.value {
            promo.value = ActiveValue::Set(value);
        }
        if let Some(start_date) = update_data.start_date {
            promo.start_date = ActiveValue::Set(start_date);
        }
        if let Some(end_date) = update_data.end_date {
            promo.end_date = ActiveValue::Set(end_date);
        }
        if let Some(is_active) = update_data.is_active {
            promo.is_active = ActiveValue::Set(is_active as i8);
        }
        if let Some(product_id) = update_data.product_id {
            promo.product_id = ActiveValue::Set(Some(product_id));
        }
        
        promo.updated_at = ActiveValue::Set(chrono::Utc::now().into());

        promo.update(db).await
    }

    pub async fn delete<C: ConnectionTrait>(db: &C, id: i32) -> Result<u64, DbErr> {
        let result = promotions::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected)
    }
}
