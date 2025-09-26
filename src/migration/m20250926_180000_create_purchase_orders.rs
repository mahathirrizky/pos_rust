use sea_orm_migration::{prelude::*, schema::*};

use super::m20250918_024324_create_table::{Employees, Products, Stores, Suppliers};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // PurchaseOrders Table
        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrders::Table)
                    .if_not_exists()
                    .col(pk_auto(PurchaseOrders::Id))
                    .col(integer(PurchaseOrders::SupplierId).not_null())
                    .col(integer(PurchaseOrders::StoreId).not_null())
                    .col(integer_null(PurchaseOrders::EmployeeId))
                    .col(timestamp_with_time_zone(PurchaseOrders::OrderDate).not_null())
                    .col(timestamp_with_time_zone_null(PurchaseOrders::ExpectedDeliveryDate))
                    .col(string(PurchaseOrders::Status).not_null().default("draft"))
                    .col(decimal_len(PurchaseOrders::TotalAmount, 10, 2).not_null().default(0.00))
                    .col(timestamp_with_time_zone(PurchaseOrders::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(PurchaseOrders::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-po-supplier_id")
                            .from(PurchaseOrders::Table, PurchaseOrders::SupplierId)
                            .to(Suppliers::Table, Suppliers::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-po-store_id")
                            .from(PurchaseOrders::Table, PurchaseOrders::StoreId)
                            .to(Stores::Table, Stores::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-po-employee_id")
                            .from(PurchaseOrders::Table, PurchaseOrders::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // PurchaseOrderItems Table
        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrderItems::Table)
                    .if_not_exists()
                    .col(pk_auto(PurchaseOrderItems::Id))
                    .col(integer(PurchaseOrderItems::PurchaseOrderId).not_null())
                    .col(integer(PurchaseOrderItems::ProductId).not_null())
                    .col(integer(PurchaseOrderItems::QuantityOrdered).not_null())
                    .col(integer(PurchaseOrderItems::QuantityReceived).not_null().default(0))
                    .col(decimal_len(PurchaseOrderItems::UnitPrice, 10, 2).not_null())
                    .col(timestamp_with_time_zone(PurchaseOrderItems::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(PurchaseOrderItems::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-po_item-po_id")
                            .from(PurchaseOrderItems::Table, PurchaseOrderItems::PurchaseOrderId)
                            .to(PurchaseOrders::Table, PurchaseOrders::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-po_item-product_id")
                            .from(PurchaseOrderItems::Table, PurchaseOrderItems::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PurchaseOrderItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PurchaseOrders::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PurchaseOrders {
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

#[derive(DeriveIden)]
pub enum PurchaseOrderItems {
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
