use sea_orm::{DbConn, DbErr, Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, DeleteResult};
use crate::entities::password_reset_tokens;

pub struct PasswordResetTokenRepository;

impl PasswordResetTokenRepository {
    pub async fn create(db: &DbConn, employee_id: i32, token: String, expires_at: chrono::NaiveDateTime) -> Result<password_reset_tokens::Model, DbErr> {
        let new_token = password_reset_tokens::ActiveModel {
            employee_id: Set(employee_id),
            token: Set(token),
            expires_at: Set(expires_at),
            ..Default::default()
        };

        new_token.insert(db).await
    }

    pub async fn find_by_token(db: &DbConn, token: String) -> Result<Option<password_reset_tokens::Model>, DbErr> {
        password_reset_tokens::Entity::find()
            .filter(password_reset_tokens::Column::Token.eq(token))
            .one(db)
            .await
    }

    pub async fn delete(db: &DbConn, token_id: i32) -> Result<DeleteResult, DbErr> {
        password_reset_tokens::Entity::delete_by_id(token_id).exec(db).await
    }
}
