use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Customers::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Customers::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Customers::FirstName).string().not_null())
                .col(ColumnDef::new(Customers::LastName).string().not_null())
                .col(ColumnDef::new(Customers::Email).string().unique_key())
                .col(ColumnDef::new(Customers::Phone).string())
                .col(ColumnDef::new(Customers::Address).text())
                .col(ColumnDef::new(Customers::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Customers::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Customers::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Customers {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Phone,
    Address,
    CreatedAt,
    UpdatedAt,
}
