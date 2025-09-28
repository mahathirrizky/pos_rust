use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Payments::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Payments::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Payments::OrderId).integer().not_null())
                .col(ColumnDef::new(Payments::PaymentMethod).string().not_null())
                .col(ColumnDef::new(Payments::Amount).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(Payments::PaymentDate).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Payments::Status).string().not_null())
                .col(ColumnDef::new(Payments::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Payments::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Payments::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    OrderId,
    PaymentMethod,
    Amount,
    PaymentDate,
    Status,
    CreatedAt,
    UpdatedAt,
}
