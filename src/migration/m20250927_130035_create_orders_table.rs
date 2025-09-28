use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Orders::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Orders::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Orders::CustomerId).integer().not_null())
                .col(ColumnDef::new(Orders::EmployeeId).integer().not_null())
                .col(ColumnDef::new(Orders::StoreId).integer().not_null())
                .col(ColumnDef::new(Orders::OrderDate).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Orders::TotalAmount).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(Orders::Status).string().not_null())
                .col(ColumnDef::new(Orders::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Orders::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Orders::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
    CustomerId,
    EmployeeId,
    StoreId,
    OrderDate,
    TotalAmount,
    Status,
    CreatedAt,
    UpdatedAt,
}
