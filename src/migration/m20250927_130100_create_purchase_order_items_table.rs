use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(PurchaseOrderItems::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(PurchaseOrderItems::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(PurchaseOrderItems::PurchaseOrderId).integer().not_null())
                .col(ColumnDef::new(PurchaseOrderItems::ProductId).integer().not_null())
                .col(ColumnDef::new(PurchaseOrderItems::QuantityOrdered).integer().not_null())
                .col(ColumnDef::new(PurchaseOrderItems::QuantityReceived).integer().not_null())
                .col(ColumnDef::new(PurchaseOrderItems::UnitPrice).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(PurchaseOrderItems::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(PurchaseOrderItems::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(PurchaseOrderItems::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum PurchaseOrderItems {
    Table,
    Id,
    PurchaseOrderId,
    ProductId,
    QuantityOrdered,
    QuantityReceived,
    UnitPrice,
    CreatedAt,
    UpdatedAt,
}
