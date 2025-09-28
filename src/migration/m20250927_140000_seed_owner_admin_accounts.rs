use sea_orm_migration::prelude::*;
use sea_orm::{ConnectionTrait, Statement, DbBackend};
use crate::auth::auth_service::hash_password;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Seed Owner and Admin users
        let owner_role_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, "SELECT id FROM roles WHERE name = 'Owner'")).await?.unwrap().try_get("", "id")?;
        let admin_role_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, "SELECT id FROM roles WHERE name = 'Admin'")).await?.unwrap().try_get("", "id")?;
        let store_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, "SELECT id FROM stores WHERE name = 'Main Store'")).await?.unwrap().try_get("", "id")?;

        let password = "password123";
        let password_hash = hash_password(password).unwrap();

        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, &format!("INSERT INTO employees (first_name, last_name, email, role, password_hash, role_id, store_id, created_at, updated_at) VALUES ('Owner', 'User', 'owner@example.com', 'Owner', '{}', {}, {}, NOW(), NOW());", password_hash, owner_role_id, store_id))).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, &format!("INSERT INTO employees (first_name, last_name, email, role, password_hash, role_id, store_id, created_at, updated_at) VALUES ('Admin', 'User', 'admin@example.com', 'Admin', '{}', {}, {}, NOW(), NOW());", password_hash, admin_role_id, store_id))).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Delete seeded employees
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "DELETE FROM employees WHERE email = 'owner@example.com' OR email = 'admin@example.com';")).await?;
        Ok(())
    }
}
