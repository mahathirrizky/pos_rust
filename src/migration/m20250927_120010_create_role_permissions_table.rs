use sea_orm_migration::prelude::*;
use sea_orm::EntityName;
use crate::entities::role_permissions::{Column as RolePermissionsColumn, Entity as RolePermissionsEntity};
use crate::entities::roles::{Column as RolesColumn, Entity as RolesEntity};
use crate::entities::permissions::{Column as PermissionsColumn, Entity as PermissionsEntity};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermissionsEntity.table_ref())
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermissionsColumn::RoleId).integer().not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermissionsColumn::PermissionId).integer().not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(RolePermissionsColumn::RoleId)
                            .col(RolePermissionsColumn::PermissionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role_permissions-role_id")
                            .from(RolePermissionsEntity.table_ref(), RolePermissionsColumn::RoleId)
                            .to(RolesEntity.table_ref(), RolesColumn::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role_permissions-permission_id")
                            .from(RolePermissionsEntity.table_ref(), RolePermissionsColumn::PermissionId)
                            .to(PermissionsEntity.table_ref(), PermissionsColumn::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermissionsEntity.table_ref()).to_owned())
            .await?;
        Ok(())
    }
}
