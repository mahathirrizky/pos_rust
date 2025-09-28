use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(OrderItems::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(OrderItems::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(OrderItems::OrderId).integer().not_null())
                .col(ColumnDef::new(OrderItems::ProductId).integer().not_null())
                .col(ColumnDef::new(OrderItems::Quantity).integer().not_null())
                .col(ColumnDef::new(OrderItems::UnitPrice).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(OrderItems::DiscountAmount).decimal_len(10, 2).not_null().default(0.00))
                .col(ColumnDef::new(OrderItems::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(OrderItems::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(OrderItems::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum OrderItems {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
    UnitPrice,
    DiscountAmount,
    CreatedAt,
    UpdatedAt,
}
