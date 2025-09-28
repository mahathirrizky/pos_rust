use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Inventory::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Inventory::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Inventory::ProductId).integer().not_null())
                .col(ColumnDef::new(Inventory::StoreId).integer().not_null())
                .col(ColumnDef::new(Inventory::Quantity).integer().not_null())
                .col(ColumnDef::new(Inventory::LastRestocked).timestamp_with_time_zone())
                .col(ColumnDef::new(Inventory::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Inventory::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Inventory::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Inventory {
    Table,
    Id,
    ProductId,
    StoreId,
    Quantity,
    LastRestocked,
    CreatedAt,
    UpdatedAt,
}
