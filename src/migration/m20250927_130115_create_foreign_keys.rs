#![allow(dead_code)]
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create foreign keys
        manager.create_foreign_key(ForeignKey::create().name("fk-employees-store_id").from(Employees::Table, Employees::StoreId).to(Stores::Table, Stores::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-inventory-product_id").from(Inventory::Table, Inventory::ProductId).to(Products::Table, Products::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-inventory-store_id").from(Inventory::Table, Inventory::StoreId).to(Stores::Table, Stores::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-order_items-order_id").from(OrderItems::Table, OrderItems::OrderId).to(Orders::Table, Orders::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-order_items-product_id").from(OrderItems::Table, OrderItems::ProductId).to(Products::Table, Products::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-orders-customer_id").from(Orders::Table, Orders::CustomerId).to(Customers::Table, Customers::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-orders-employee_id").from(Orders::Table, Orders::EmployeeId).to(Employees::Table, Employees::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-orders-store_id").from(Orders::Table, Orders::StoreId).to(Stores::Table, Stores::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-payments-order_id").from(Payments::Table, Payments::OrderId).to(Orders::Table, Orders::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-products-category_id").from(Products::Table, Products::CategoryId).to(Categories::Table, Categories::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-products-supplier_id").from(Products::Table, Products::SupplierId).to(Suppliers::Table, Suppliers::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-promotions-product_id").from(Promotions::Table, Promotions::ProductId).to(Products::Table, Products::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-purchase_order_items-purchase_order_id").from(PurchaseOrderItems::Table, PurchaseOrderItems::PurchaseOrderId).to(PurchaseOrders::Table, PurchaseOrders::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-purchase_order_items-product_id").from(PurchaseOrderItems::Table, PurchaseOrderItems::ProductId).to(Products::Table, Products::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-purchase_orders-supplier_id").from(PurchaseOrders::Table, PurchaseOrders::SupplierId).to(Suppliers::Table, Suppliers::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-purchase_orders-store_id").from(PurchaseOrders::Table, PurchaseOrders::StoreId).to(Stores::Table, Stores::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-purchase_orders-employee_id").from(PurchaseOrders::Table, PurchaseOrders::EmployeeId).to(Employees::Table, Employees::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-refund_items-refund_id").from(RefundItems::Table, RefundItems::RefundId).to(Refunds::Table, Refunds::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-refund_items-order_item_id").from(RefundItems::Table, RefundItems::OrderItemId).to(OrderItems::Table, OrderItems::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-refund_items-product_id").from(RefundItems::Table, RefundItems::ProductId).to(Products::Table, Products::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-refunds-order_id").from(Refunds::Table, Refunds::OrderId).to(Orders::Table, Orders::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-refunds-employee_id").from(Refunds::Table, Refunds::EmployeeId).to(Employees::Table, Employees::Id).to_owned()).await?;
        manager.create_foreign_key(ForeignKey::create().name("fk-refunds-store_id").from(Refunds::Table, Refunds::StoreId).to(Stores::Table, Stores::Id).to_owned()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_foreign_key(ForeignKey::drop().name("fk-employees-store_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-inventory-product_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-inventory-store_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-order_items-order_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-order_items-product_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-orders-customer_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-orders-employee_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-orders-store_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-payments-order_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-products-category_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-products-supplier_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-promotions-product_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-purchase_order_items-purchase_order_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-purchase_order_items-product_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-purchase_orders-supplier_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-purchase_orders-store_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-purchase_orders-employee_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-refund_items-refund_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-refund_items-order_item_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-refund_items-product_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-refunds-order_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-refunds-employee_id").to_owned()).await?;
        manager.drop_foreign_key(ForeignKey::drop().name("fk-refunds-store_id").to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Customers {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Employees {
    Table,
    Id,
    StoreId,
    RoleId,
}

#[derive(DeriveIden)]
enum Inventory {
    Table,
    Id,
    ProductId,
    StoreId,
}

#[derive(DeriveIden)]
enum OrderItems {
    Table,
    Id,
    OrderId,
    ProductId,
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
    CustomerId,
    EmployeeId,
    StoreId,
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    OrderId,
}

#[derive(DeriveIden)]
enum Permissions {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    CategoryId,
    SupplierId,
}

#[derive(DeriveIden)]
enum Promotions {
    Table,
    Id,
    ProductId,
}

#[derive(DeriveIden)]
enum PurchaseOrderItems {
    Table,
    Id,
    PurchaseOrderId,
    ProductId,
}

#[derive(DeriveIden)]
enum PurchaseOrders {
    Table,
    Id,
    SupplierId,
    StoreId,
    EmployeeId,
}

#[derive(DeriveIden)]
enum RefundItems {
    Table,
    Id,
    RefundId,
    OrderItemId,
    ProductId,
}

#[derive(DeriveIden)]
enum Refunds {
    Table,
    Id,
    OrderId,
    EmployeeId,
    StoreId,
}

#[derive(DeriveIden)]
enum RolePermissions {
    Table,
    RoleId,
    PermissionId,
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Stores {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Suppliers {
    Table,
    Id,
}
