use sea_orm_migration::prelude::*;
use sea_orm::{ConnectionTrait, Statement, DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO stores (name, address, phone, created_at, updated_at) VALUES ('Main Store', '123 Main St', '555-1234', NOW(), NOW());")).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "DELETE FROM stores WHERE name = 'Main Store';")).await?;
        Ok(())
    }
}
