use sea_orm_migration::prelude::*;
use sea_orm::EntityName;
use crate::entities::employees::{Column as EmployeesColumn, Entity as EmployeesEntity};
use crate::entities::roles::{Column as RolesColumn, Entity as RolesEntity};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(EmployeesEntity.table_ref())
                    .add_column_if_not_exists(
                        ColumnDef::new(EmployeesColumn::RoleId)
                            .integer()
                            .null(), // Allow null temporarily for data migration
                    )
                    .to_owned(),
            )
            .await?;

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-employees-role_id")
                .from(EmployeesEntity.table_ref(), EmployeesColumn::RoleId)
                .to(RolesEntity.table_ref(), RolesColumn::Id)
                .on_delete(ForeignKeyAction::Restrict)
                .on_update(ForeignKeyAction::Cascade)
                .to_owned(),
        ).await?;

        manager
            .alter_table(
                Table::alter()
                    .table(EmployeesEntity.table_ref())
                    .modify_column(ColumnDef::new(EmployeesColumn::RoleId).integer().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter().table(EmployeesEntity.table_ref()).drop_column(EmployeesColumn::RoleId).to_owned())
            .await?;
        Ok(())
    }
}
