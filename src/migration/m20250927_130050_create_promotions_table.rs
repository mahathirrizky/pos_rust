use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Promotions::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Promotions::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Promotions::Name).string().not_null())
                .col(ColumnDef::new(Promotions::Description).string())
                .col(ColumnDef::new(Promotions::PromotionType).string().not_null())
                .col(ColumnDef::new(Promotions::Value).decimal_len(10, 2).not_null())
                .col(ColumnDef::new(Promotions::StartDate).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Promotions::EndDate).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Promotions::IsActive).boolean().not_null())
                .col(ColumnDef::new(Promotions::ProductId).integer())
                .col(ColumnDef::new(Promotions::CreatedAt).timestamp_with_time_zone().not_null())
                .col(ColumnDef::new(Promotions::UpdatedAt).timestamp_with_time_zone().not_null())
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Promotions::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Promotions {
    Table,
    Id,
    Name,
    Description,
    PromotionType,
    Value,
    StartDate,
    EndDate,
    IsActive,
    ProductId,
    CreatedAt,
    UpdatedAt,
}
