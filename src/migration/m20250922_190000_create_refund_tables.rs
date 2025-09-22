use sea_orm_migration::{prelude::*, schema::*};

use super::m20250918_024324_create_table::{Employees, OrderItems, Orders, Products, Stores};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Refunds Table
        manager
            .create_table(
                Table::create()
                    .table(Refunds::Table)
                    .if_not_exists()
                    .col(pk_auto(Refunds::Id))
                    .col(integer(Refunds::OrderId).not_null())
                    .col(integer_null(Refunds::EmployeeId))
                    .col(integer(Refunds::StoreId).not_null())
                    .col(text(Refunds::Reason).not_null())
                    .col(decimal_len(Refunds::TotalAmount, 10, 2).not_null())
                    .col(timestamp_with_time_zone(Refunds::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Refunds::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refund-order_id")
                            .from(Refunds::Table, Refunds::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refund-employee_id")
                            .from(Refunds::Table, Refunds::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refund-store_id")
                            .from(Refunds::Table, Refunds::StoreId)
                            .to(Stores::Table, Stores::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // RefundItems Table
        manager
            .create_table(
                Table::create()
                    .table(RefundItems::Table)
                    .if_not_exists()
                    .col(pk_auto(RefundItems::Id))
                    .col(integer(RefundItems::RefundId).not_null())
                    .col(integer(RefundItems::OrderItemId).not_null())
                    .col(integer(RefundItems::ProductId).not_null())
                    .col(integer(RefundItems::Quantity).not_null())
                    .col(decimal_len(RefundItems::Amount, 10, 2).not_null())
                    .col(timestamp_with_time_zone(RefundItems::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(RefundItems::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refund_item-refund_id")
                            .from(RefundItems::Table, RefundItems::RefundId)
                            .to(Refunds::Table, Refunds::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refund_item-order_item_id")
                            .from(RefundItems::Table, RefundItems::OrderItemId)
                            .to(OrderItems::Table, OrderItems::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refund_item-product_id")
                            .from(RefundItems::Table, RefundItems::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefundItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Refunds::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Refunds {
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

#[derive(DeriveIden)]
pub enum RefundItems {
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
