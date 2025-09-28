use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Stores::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Stores::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Stores::Name).string().not_null())
                .col(ColumnDef::new(Stores::Address).text())
                .col(ColumnDef::new(Stores::Phone).string())
                .col(ColumnDef::new(Stores::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Stores::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Stores::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Stores {
    Table,
    Id,
    Name,
    Address,
    Phone,
    CreatedAt,
    UpdatedAt,
}
