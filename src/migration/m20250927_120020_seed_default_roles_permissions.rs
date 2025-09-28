#![allow(dead_code)]
#![allow(dead_code)]
use sea_orm_migration::prelude::*;
use sea_orm::{ConnectionTrait, Statement, DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Seed initial roles
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO roles (name, description) VALUES ('Owner', 'System owner with full control');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO roles (name, description) VALUES ('Admin', 'Administrator with broad management capabilities');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO roles (name, description) VALUES ('StoreManager', 'Manages a specific store');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO roles (name, description) VALUES ('InventoryManager', 'Manages inventory for a store');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO roles (name, description) VALUES ('Cashier', 'Handles sales and customer transactions');")).await?;

        // Seed initial permissions
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_employees', 'Can create, update, and delete employees');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('view_reports', 'Can view various business reports');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_products', 'Can create, update, and delete products');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_inventory', 'Can manage inventory levels and stock');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('process_sales', 'Can process sales transactions');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('view_promotions', 'Can view promotions');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('create_promotions', 'Can create promotions');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('update_promotions', 'Can update promotions');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('delete_promotions', 'Can delete promotions');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_refunds', 'Can process customer refunds');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_suppliers', 'Can manage supplier information');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_purchase_orders', 'Can create and manage purchase orders');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_stores', 'Can create, update, and delete stores');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_categories', 'Can create, update, and delete product categories');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_customers', 'Can create, update, and delete customer information');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_roles', 'Can create, read, update, and delete roles');")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "INSERT IGNORE INTO permissions (name, description) VALUES ('manage_permissions', 'Can assign and remove permissions from roles');")).await?;

        // Seed initial role permissions
        let owner_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, "SELECT id FROM roles WHERE name = 'Owner'")).await?.unwrap().try_get("", "id")?;
        let admin_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, "SELECT id FROM roles WHERE name = 'Admin'")).await?.unwrap().try_get("", "id")?;
        let store_manager_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, "SELECT id FROM roles WHERE name = 'StoreManager'")).await?.unwrap().try_get("", "id")?;
        let cashier_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, "SELECT id FROM roles WHERE name = 'Cashier'")).await?.unwrap().try_get("", "id")?;

        let all_permissions: Vec<i32> = manager.get_connection().query_all(Statement::from_string(DbBackend::MySql, "SELECT id FROM permissions")).await?.into_iter().map(|row| row.try_get("", "id").unwrap()).collect();
        for perm_id in &all_permissions {
            manager.get_connection().execute(Statement::from_string(DbBackend::MySql, &format!("INSERT IGNORE INTO role_permissions (role_id, permission_id) VALUES ({}, {});", owner_id, perm_id))).await?;
        }

        // Assign specific permissions to Admin
        let admin_permissions = vec![
            "manage_employees", "view_reports", "manage_products", "manage_inventory",
            "process_sales", "view_promotions", "create_promotions", "update_promotions", "delete_promotions",
            "manage_refunds", "manage_suppliers", "manage_purchase_orders", "manage_stores",
            "manage_categories", "manage_customers", "manage_roles", "manage_permissions"
        ];
        for perm_name in admin_permissions {
            let perm_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, &format!("SELECT id FROM permissions WHERE name = '{}'", perm_name))).await?.unwrap().try_get("", "id")?;
            manager.get_connection().execute(Statement::from_string(DbBackend::MySql, &format!("INSERT IGNORE INTO role_permissions (role_id, permission_id) VALUES ({}, {});", admin_id, perm_id))).await?;
        }

        // Assign specific permissions to StoreManager
        let store_manager_permissions = vec![
            "view_reports", "manage_products", "manage_inventory", "process_sales",
            "view_promotions", "update_promotions", "manage_refunds", "manage_suppliers",
            "manage_purchase_orders", "manage_employees", "manage_customers"
        ];
        for perm_name in store_manager_permissions {
            let perm_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, &format!("SELECT id FROM permissions WHERE name = '{}'", perm_name))).await?.unwrap().try_get("", "id")?;
            manager.get_connection().execute(Statement::from_string(DbBackend::MySql, &format!("INSERT IGNORE INTO role_permissions (role_id, permission_id) VALUES ({}, {});", store_manager_id, perm_id))).await?;
        }

        // Assign specific permissions to Cashier
        let cashier_permissions = vec![
            "process_sales", "view_promotions", "view_reports", "manage_customers"
        ];
        for perm_name in cashier_permissions {
            let perm_id: i32 = manager.get_connection().query_one(Statement::from_string(DbBackend::MySql, &format!("SELECT id FROM permissions WHERE name = '{}'", perm_name))).await?.unwrap().try_get("", "id")?;
            manager.get_connection().execute(Statement::from_string(DbBackend::MySql, &format!("INSERT IGNORE INTO role_permissions (role_id, permission_id) VALUES ({}, {});", cashier_id, perm_id))).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Delete seeded roles and permissions
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "DELETE FROM role_permissions;")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "DELETE FROM permissions;")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "DELETE FROM roles;")).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
enum Permissions {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
enum RolePermissions {
    Table,
    RoleId,
    PermissionId,
}
