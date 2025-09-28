use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(PurchaseOrders::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(PurchaseOrders::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(PurchaseOrders::SupplierId).integer().not_null())
                .col(ColumnDef::new(PurchaseOrders::StoreId).integer().not_null())
                .col(ColumnDef::new(PurchaseOrders::EmployeeId).integer())
                .col(ColumnDef::new(PurchaseOrders::OrderDate).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(PurchaseOrders::ExpectedDeliveryDate).timestamp_with_time_zone())
                .col(ColumnDef::new(PurchaseOrders::Status).string().not_null())
                .col(ColumnDef::new(PurchaseOrders::TotalAmount).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(PurchaseOrders::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(PurchaseOrders::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(PurchaseOrders::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum PurchaseOrders {
    Table,
    Id,
    SupplierId,
    StoreId,
    EmployeeId,
    OrderDate,
    ExpectedDeliveryDate,
    Status,
    TotalAmount,
    CreatedAt,
    UpdatedAt,
}
