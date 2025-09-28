use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Refunds::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Refunds::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Refunds::OrderId).integer().not_null())
                .col(ColumnDef::new(Refunds::EmployeeId).integer().not_null())
                .col(ColumnDef::new(Refunds::StoreId).integer().not_null())
                .col(ColumnDef::new(Refunds::Reason).string().not_null())
                .col(ColumnDef::new(Refunds::TotalAmount).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(Refunds::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Refunds::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Refunds::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Refunds {
    Table,
    Id,
    OrderId,
    EmployeeId,
    StoreId,
    Reason,
    TotalAmount,
    CreatedAt,
    UpdatedAt,
}
