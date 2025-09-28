use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Suppliers::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Suppliers::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Suppliers::Name).string().not_null())
                .col(ColumnDef::new(Suppliers::ContactPerson).string())
                .col(ColumnDef::new(Suppliers::Email).string().unique_key())
                .col(ColumnDef::new(Suppliers::Phone).string())
                .col(ColumnDef::new(Suppliers::Address).text())
                .col(ColumnDef::new(Suppliers::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Suppliers::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Suppliers::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Suppliers {
    Table,
    Id,
    Name,
    ContactPerson,
    Email,
    Phone,
    Address,
    CreatedAt,
    UpdatedAt,
}
