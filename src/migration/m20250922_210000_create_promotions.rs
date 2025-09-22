use sea_orm_migration::{prelude::*, schema::*};

use super::m20250918_024324_create_table::{Products};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Promotions Table
        manager
            .create_table(
                Table::create()
                    .table(Promotions::Table)
                    .if_not_exists()
                    .col(pk_auto(Promotions::Id))
                    .col(string(Promotions::Name).not_null())
                    .col(text_null(Promotions::Description))
                    .col(string(Promotions::PromotionType).not_null()) // e.g., "PERCENTAGE", "FIXED_AMOUNT"
                    .col(decimal_len(Promotions::Value, 10, 2).not_null())
                    .col(timestamp_with_time_zone(Promotions::StartDate).not_null())
                    .col(timestamp_with_time_zone(Promotions::EndDate).not_null())
                    .col(boolean(Promotions::IsActive).not_null().default(true))
                    .col(integer_null(Promotions::ProductId))
                    .col(timestamp_with_time_zone(Promotions::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Promotions::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-promotion-product_id")
                            .from(Promotions::Table, Promotions::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add discount_amount column to order_items table
        manager
            .alter_table(
                Table::alter()
                    .table(OrderItems::Table)
                    .add_column(
                        decimal_len(OrderItems::DiscountAmount, 10, 2)
                            .not_null()
                            .default(0.00),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the column from order_items
        manager
            .alter_table(
                Table::alter()
                    .table(OrderItems::Table)
                    .drop_column(Alias::new("discount_amount"))
                    .to_owned(),
            )
            .await?;

        // Drop the promotions table
        manager
            .drop_table(Table::drop().table(Promotions::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Promotions {
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

#[derive(DeriveIden)]
enum OrderItems {
    Table,
    DiscountAmount,
}
