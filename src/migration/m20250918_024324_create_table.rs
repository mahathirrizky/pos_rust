use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Categories Table
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(pk_auto(Categories::Id))
                    .col(string_uniq(Categories::Name))
                    .col(text_null(Categories::Description))
                    .col(timestamp_with_time_zone(Categories::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Categories::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Suppliers Table
        manager
            .create_table(
                Table::create()
                    .table(Suppliers::Table)
                    .if_not_exists()
                    .col(pk_auto(Suppliers::Id))
                    .col(string(Suppliers::Name).not_null())
                    .col(string_null(Suppliers::ContactPerson))
                    .col(string_uniq(Suppliers::Email).null())
                    .col(string_null(Suppliers::Phone))
                    .col(text_null(Suppliers::Address))
                    .col(timestamp_with_time_zone(Suppliers::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Suppliers::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Stores Table
        manager
            .create_table(
                Table::create()
                    .table(Stores::Table)
                    .if_not_exists()
                    .col(pk_auto(Stores::Id))
                    .col(string(Stores::Name).not_null())
                    .col(text_null(Stores::Address))
                    .col(string_null(Stores::Phone))
                    .col(timestamp_with_time_zone(Stores::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Stores::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Customers Table
        manager
            .create_table(
                Table::create()
                    .table(Customers::Table)
                    .if_not_exists()
                    .col(pk_auto(Customers::Id))
                    .col(string(Customers::FirstName).not_null())
                    .col(string(Customers::LastName).not_null())
                    .col(string_uniq(Customers::Email).null())
                    .col(string_null(Customers::Phone))
                    .col(text_null(Customers::Address))
                    .col(timestamp_with_time_zone(Customers::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Customers::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Employees Table
        manager
            .create_table(
                Table::create()
                    .table(Employees::Table)
                    .if_not_exists()
                    .col(pk_auto(Employees::Id))
                    .col(string(Employees::FirstName).not_null())
                    .col(string(Employees::LastName).not_null())
                    .col(string_uniq(Employees::Email).not_null())
                    .col(string_null(Employees::Phone))
                    .col(integer(Employees::StoreId).null())
                    .col(string(Employees::Role).not_null())
                    .col(string(Employees::PasswordHash).not_null())
                    .col(timestamp_with_time_zone(Employees::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Employees::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-employee-store_id")
                            .from(Employees::Table, Employees::StoreId)
                            .to(Stores::Table, Stores::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Products Table
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(pk_auto(Products::Id))
                    .col(string(Products::Name).not_null())
                    .col(text_null(Products::Description))
                    .col(decimal_len(Products::Price, 10, 2).not_null())
                    .col(string_uniq(Products::Sku).not_null())
                    .col(integer(Products::CategoryId).not_null())
                    .col(integer(Products::SupplierId).not_null())
                    .col(timestamp_with_time_zone(Products::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Products::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product-category_id")
                            .from(Products::Table, Products::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-product-supplier_id")
                            .from(Products::Table, Products::SupplierId)
                            .to(Suppliers::Table, Suppliers::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Orders Table
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(pk_auto(Orders::Id))
                    .col(integer(Orders::CustomerId).not_null())
                    .col(integer(Orders::EmployeeId).not_null())
                    .col(integer(Orders::StoreId).not_null())
                    .col(timestamp_with_time_zone(Orders::OrderDate).not_null())
                    .col(decimal_len(Orders::TotalAmount, 10, 2).not_null())
                    .col(string(Orders::Status).not_null())
                    .col(timestamp_with_time_zone(Orders::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Orders::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order-customer_id")
                            .from(Orders::Table, Orders::CustomerId)
                            .to(Customers::Table, Customers::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order-employee_id")
                            .from(Orders::Table, Orders::EmployeeId)
                            .to(Employees::Table, Employees::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order-store_id")
                            .from(Orders::Table, Orders::StoreId)
                            .to(Stores::Table, Stores::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // OrderItems Table
        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(pk_auto(OrderItems::Id))
                    .col(integer(OrderItems::OrderId).not_null())
                    .col(integer(OrderItems::ProductId).not_null())
                    .col(integer(OrderItems::Quantity).not_null())
                    .col(decimal_len(OrderItems::UnitPrice, 10, 2).not_null())
                    .col(timestamp_with_time_zone(OrderItems::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(OrderItems::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order_item-order_id")
                            .from(OrderItems::Table, OrderItems::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order_item-product_id")
                            .from(OrderItems::Table, OrderItems::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Inventory Table
        manager
            .create_table(
                Table::create()
                    .table(Inventory::Table)
                    .if_not_exists()
                    .col(pk_auto(Inventory::Id))
                    .col(integer(Inventory::ProductId).not_null())
                    .col(integer(Inventory::StoreId).not_null())
                    .col(integer(Inventory::Quantity).not_null())
                    .col(timestamp_with_time_zone_null(Inventory::LastRestocked))
                    .col(timestamp_with_time_zone(Inventory::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Inventory::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-inventory-product_id")
                            .from(Inventory::Table, Inventory::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-inventory-store_id")
                            .from(Inventory::Table, Inventory::StoreId)
                            .to(Stores::Table, Stores::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Payments Table
        manager
            .create_table(
                Table::create()
                    .table(Payments::Table)
                    .if_not_exists()
                    .col(pk_auto(Payments::Id))
                    .col(integer(Payments::OrderId).not_null())
                    .col(string(Payments::PaymentMethod).not_null())
                    .col(decimal_len(Payments::Amount, 10, 2).not_null())
                    .col(timestamp_with_time_zone(Payments::PaymentDate).not_null())
                    .col(string(Payments::Status).not_null())
                    .col(timestamp_with_time_zone(Payments::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Payments::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-payment-order_id")
                            .from(Payments::Table, Payments::OrderId)
                            .to(Orders::Table, Orders::Id)
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
            .drop_table(Table::drop().table(Payments::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Inventory::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OrderItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Employees::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Customers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Stores::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Suppliers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Categories {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Suppliers {
    Table,
    Id,
    Name,
    ContactPerson,
    Email,
    Phone,
    Address,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Stores {
    Table,
    Id,
    Name,
    Address,
    Phone,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Customers {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Phone,
    Address,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Employees {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Phone,
    StoreId,
    Role,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Products {
    Table,
    Id,
    Name,
    Description,
    Price,
    Sku,
    CategoryId,
    SupplierId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Orders {
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

#[derive(DeriveIden)]
pub enum OrderItems {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
    UnitPrice,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Inventory {
    Table,
    Id,
    ProductId,
    StoreId,
    Quantity,
    LastRestocked,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Payments {
    Table,
    Id,
    OrderId,
    PaymentMethod,
    Amount,
    PaymentDate,
    Status,
    CreatedAt,
    UpdatedAt,
}
