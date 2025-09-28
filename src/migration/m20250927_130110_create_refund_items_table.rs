use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(RefundItems::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(RefundItems::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(RefundItems::RefundId).integer().not_null())
                .col(ColumnDef::new(RefundItems::OrderItemId).integer().not_null())
                .col(ColumnDef::new(RefundItems::ProductId).integer().not_null())
                .col(ColumnDef::new(RefundItems::Quantity).integer().not_null())
                .col(ColumnDef::new(RefundItems::Amount).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(RefundItems::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(RefundItems::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(RefundItems::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum RefundItems {
    Table,
    Id,
    RefundId,
    OrderItemId,
    ProductId,
    Quantity,
    Amount,
    CreatedAt,
    UpdatedAt,
}
