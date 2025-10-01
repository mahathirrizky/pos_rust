use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue, ConnectionTrait};
use crate::entities::{permissions, role_permissions};

pub struct PermissionsRepository;

impl PermissionsRepository {
    pub async fn get_all<C: ConnectionTrait>(db: &C) -> Result<Vec<permissions::Model>, DbErr> {
        permissions::Entity::find().all(db).await
    }

    pub async fn create<C: ConnectionTrait>(db: &C, new_permission: permissions::CreatePermission) -> Result<permissions::Model, DbErr> {
        let permission = permissions::ActiveModel {
            name: ActiveValue::Set(new_permission.name),
            description: ActiveValue::Set(new_permission.description),
            ..Default::default()
        };
        permission.insert(db).await
    }

    pub async fn update<C: ConnectionTrait>(db: &C, id: i32, update_data: permissions::UpdatePermission) -> Result<Option<permissions::Model>, DbErr> {
        let permission: Option<permissions::Model> = permissions::Entity::find_by_id(id).one(db).await?;
        if let Some(permission) = permission {
            let mut active_model: permissions::ActiveModel = permission.into();
            if let Some(name) = update_data.name {
                active_model.name = ActiveValue::Set(name);
            }
            if let Some(description) = update_data.description {
                active_model.description = ActiveValue::Set(Some(description));
            }
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete<C: ConnectionTrait>(db: &C, id: i32) -> Result<u64, DbErr> {
        let res = permissions::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }

    pub async fn find_permissions_for_role(db: &DatabaseConnection, role_id: i32) -> Result<Vec<permissions::Model>, DbErr> {
        let permissions = permissions::Entity::find()
            .inner_join(role_permissions::Entity)
            .filter(role_permissions::Column::RoleId.eq(role_id))
            .all(db)
            .await?;

        Ok(permissions)
    }
}
