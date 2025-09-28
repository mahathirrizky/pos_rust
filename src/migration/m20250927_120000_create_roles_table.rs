use sea_orm_migration::prelude::*;
use sea_orm::EntityName;
use crate::entities::roles::{Column as RolesColumn, Entity as RolesEntity};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolesEntity.table_ref())
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolesColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RolesColumn::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(RolesColumn::Description).string())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolesEntity.table_ref()).to_owned())
            .await?;
        Ok(())
    }
}
