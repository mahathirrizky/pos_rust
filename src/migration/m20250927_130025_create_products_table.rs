use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Products::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Products::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Products::Name).string().not_null())
                .col(ColumnDef::new(Products::Description).text())
                .col(ColumnDef::new(Products::Price).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(Products::Sku).string().not_null().unique_key())
                .col(ColumnDef::new(Products::CategoryId).integer().not_null())
                .col(ColumnDef::new(Products::SupplierId).integer().not_null())
                .col(ColumnDef::new(Products::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Products::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Products::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    Name,
    Description,
    Price,
    Sku,
    CategoryId,
    SupplierId,
    CreatedAt,
    UpdatedAt,
}
