pub use sea_orm_migration::prelude::*;

// Table Creations
mod m20250927_120000_create_roles_table;
mod m20250927_120005_create_permissions_table;
mod m20250927_120010_create_role_permissions_table;
mod m20250927_130000_create_categories_table;
mod m20250927_130005_create_customers_table;
mod m20250927_130010_create_employees_table;
mod m20250927_130015_create_stores_table;
mod m20250927_130020_create_suppliers_table;
mod m20250927_130025_create_products_table;
mod m20250927_130030_create_inventory_table;
mod m20250927_130035_create_orders_table;
mod m20250927_130040_create_order_items_table;
mod m20250927_130045_create_payments_table;
mod m20250927_130050_create_promotions_table;
mod m20250927_130055_create_purchase_orders_table;
mod m20250927_130100_create_purchase_order_items_table;
mod m20250927_130105_create_refunds_table;
mod m20250927_130110_create_refund_items_table;

// Alterations and Foreign Keys
mod m20250927_120015_alter_employees_add_role_id;
mod m20250927_130115_create_foreign_keys;

// Seeding
mod m20250927_120020_seed_default_roles_permissions;
mod m20250927_130120_seed_main_store;
mod m20250927_140000_seed_owner_admin_accounts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Create Tables
            Box::new(m20250927_120000_create_roles_table::Migration),
            Box::new(m20250927_120005_create_permissions_table::Migration),
            Box::new(m20250927_120010_create_role_permissions_table::Migration),
            Box::new(m20250927_130000_create_categories_table::Migration),
            Box::new(m20250927_130005_create_customers_table::Migration),
            Box::new(m20250927_130010_create_employees_table::Migration),
            Box::new(m20250927_130015_create_stores_table::Migration),
            Box::new(m20250927_130020_create_suppliers_table::Migration),
            Box::new(m20250927_130025_create_products_table::Migration),
            Box::new(m20250927_130030_create_inventory_table::Migration),
            Box::new(m20250927_130035_create_orders_table::Migration),
            Box::new(m20250927_130040_create_order_items_table::Migration),
            Box::new(m20250927_130045_create_payments_table::Migration),
            Box::new(m20250927_130050_create_promotions_table::Migration),
            Box::new(m20250927_130055_create_purchase_orders_table::Migration),
            Box::new(m20250927_130100_create_purchase_order_items_table::Migration),
            Box::new(m20250927_130105_create_refunds_table::Migration),
            Box::new(m20250927_130110_create_refund_items_table::Migration),

            // Alterations and Foreign Keys
            Box::new(m20250927_120015_alter_employees_add_role_id::Migration),
            Box::new(m20250927_130115_create_foreign_keys::Migration),

            // Seed Data
            Box::new(m20250927_120020_seed_default_roles_permissions::Migration),
            Box::new(m20250927_130120_seed_main_store::Migration),
            Box::new(m20250927_140000_seed_owner_admin_accounts::Migration),
        ]
    }
}