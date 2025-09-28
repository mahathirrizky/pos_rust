use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Employees::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Employees::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Employees::FirstName).string().not_null())
                .col(ColumnDef::new(Employees::LastName).string().not_null())
                .col(ColumnDef::new(Employees::Email).string().not_null().unique_key())
                .col(ColumnDef::new(Employees::Phone).string())
                .col(ColumnDef::new(Employees::StoreId).integer())
                .col(ColumnDef::new(Employees::Role).string().not_null())
                .col(ColumnDef::new(Employees::PasswordHash).string().not_null())
                .col(ColumnDef::new(Employees::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Employees::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Employees::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Employees {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Phone,
    StoreId,
    Role,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
}
