use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait, ActiveValue, QueryFilter, ColumnTrait};
use crate::entities::employees;

pub struct EmployeeRepository;

impl EmployeeRepository {
    pub async fn get_all(db: &DatabaseConnection, role: Option<String>, store_id: Option<i32>, roles_to_exclude: Option<Vec<String>>) -> Result<Vec<employees::Model>, DbErr> {
        dbg!(&role, &store_id, &roles_to_exclude);
        let mut query = employees::Entity::find();
        if let Some(role) = role {
            query = query.filter(employees::Column::Role.eq(role));
        }
        if let Some(store_id) = store_id {
            query = query.filter(employees::Column::StoreId.eq(store_id));
        }
        if let Some(roles_to_exclude) = roles_to_exclude {
            query = query.filter(employees::Column::Role.is_not_in(roles_to_exclude));
        }
        query.all(db).await
    }

    pub async fn get_all_by_store(db: &DatabaseConnection, store_id: i32) -> Result<Vec<employees::Model>, DbErr> {
        employees::Entity::find().filter(employees::Column::StoreId.eq(store_id)).all(db).await
    }

    pub async fn create(db: &DatabaseConnection, new_employee: employees::CreateEmployee, role: String) -> Result<employees::Model, DbErr> {
        let employee = employees::ActiveModel {
            first_name: ActiveValue::Set(new_employee.first_name),
            last_name: ActiveValue::Set(new_employee.last_name),
            email: ActiveValue::Set(new_employee.email),
            phone: ActiveValue::Set(new_employee.phone),
            store_id: ActiveValue::Set(new_employee.store_id),
            role: ActiveValue::Set(role),
            password_hash: ActiveValue::Set(new_employee.password_hash),
            ..Default::default()
        };
        employee.insert(db).await
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<employees::Model>, DbErr> {
        employees::Entity::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DatabaseConnection, id: i32, update_data: employees::UpdateEmployee) -> Result<Option<employees::Model>, DbErr> {
        let employee: Option<employees::Model> = employees::Entity::find_by_id(id).one(db).await?;
        if let Some(employee) = employee {
            let mut active_model: employees::ActiveModel = employee.into();
            if let Some(first_name) = update_data.first_name {
                active_model.first_name = ActiveValue::Set(first_name);
            }
            if let Some(last_name) = update_data.last_name {
                active_model.last_name = ActiveValue::Set(last_name);
            }
            if let Some(email) = update_data.email {
                active_model.email = ActiveValue::Set(email);
            }
            if let Some(phone) = update_data.phone {
                active_model.phone = ActiveValue::Set(Some(phone));
            }
            if let Some(store_id) = update_data.store_id {
                active_model.store_id = ActiveValue::Set(Some(store_id));
            }
            if let Some(role) = update_data.role {
                active_model.role = ActiveValue::Set(role);
            }
            if let Some(password_hash) = update_data.password_hash {
                active_model.password_hash = ActiveValue::Set(password_hash);
            }
            Ok(Some(active_model.update(db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
        let res = employees::Entity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }

    pub async fn find_by_email(db: &DatabaseConnection, email: String) -> Result<Option<employees::Model>, DbErr> {
        employees::Entity::find().filter(employees::Column::Email.eq(email)).one(db).await
    }
}
