use sea_orm_migration::prelude::*;
use sea_orm::EntityName;
use crate::entities::permissions::{Column as PermissionsColumn, Entity as PermissionsEntity};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PermissionsEntity.table_ref())
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermissionsColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PermissionsColumn::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(PermissionsColumn::Description).string())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PermissionsEntity.table_ref()).to_owned())
            .await?;
        Ok(())
    }
}
