use sea_orm::{DbErr, EntityTrait, ActiveModelTrait, ActiveValue, ConnectionTrait, QueryFilter, ColumnTrait};
use crate::entities::roles::{self, CreateRole, UpdateRole};
use crate::entities::role_permissions;

pub struct RoleRepository;

impl RoleRepository {
    pub async fn get_all<C: ConnectionTrait>(db: &C) -> Result<Vec<roles::Model>, DbErr> {
        roles::Entity::find().all(db).await
    }

    pub async fn find_by_id<C: ConnectionTrait>(db: &C, id: i32) -> Result<Option<roles::Model>, DbErr> {
        roles::Entity::find_by_id(id).one(db).await
    }

    pub async fn create<C: ConnectionTrait>(db: &C, new_role: CreateRole) -> Result<roles::Model, DbErr> {
        let role = roles::ActiveModel {
            name: ActiveValue::Set(new_role.name),
            description: ActiveValue::Set(new_role.description),
            ..Default::default()
        };
        role.insert(db).await
    }

    pub async fn update<C: ConnectionTrait>(db: &C, id: i32, update_data: UpdateRole) -> Result<Option<roles::Model>, DbErr> {
        let role: Option<roles::Model> = roles::Entity::find_by_id(id).one(db).await?;
        if let Some(role) = role {
            let mut active_model: roles::ActiveModel = role.into();
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

    pub async fn find_by_name<C: ConnectionTrait>(db: &C, name: String) -> Result<Option<roles::Model>, DbErr> {
        roles::Entity::find().filter(roles::Column::Name.eq(name)).one(db).await
    }

    pub async fn delete<C: ConnectionTrait>(db: &C, id: i32) -> Result<u64, DbErr> {
        let res = roles::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }

    pub async fn assign_permission<C: ConnectionTrait>(db: &C, role_id: i32, permission_id: i32) -> Result<role_permissions::Model, DbErr> {
        let role_permission = role_permissions::ActiveModel {
            role_id: ActiveValue::Set(role_id),
            permission_id: ActiveValue::Set(permission_id),
        };
        role_permission.insert(db).await
    }

    pub async fn remove_permission<C: ConnectionTrait>(db: &C, role_id: i32, permission_id: i32) -> Result<u64, DbErr> {
        let res = role_permissions::Entity::delete_many()
            .filter(role_permissions::Column::RoleId.eq(role_id))
            .filter(role_permissions::Column::PermissionId.eq(permission_id))
            .exec(db)
            .await?;
        Ok(res.rows_affected)
    }
}
