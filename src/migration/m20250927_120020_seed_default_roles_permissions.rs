#![allow(dead_code)]
use sea_orm_migration::prelude::*;
use sea_orm::{ConnectionTrait, Statement, DbBackend, FromQueryResult};

#[derive(DeriveMigrationName)]
pub struct Migration;

async fn seed_permissions(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let permissions = vec![
        // Products & Inventory
        ("products:create", "Can create products"),
        ("products:read", "Can read products"),
        ("products:update", "Can update products"),
        ("products:delete", "Can delete products"),
        ("categories:create", "Can create categories"),
        ("categories:read", "Can read categories"),
        ("categories:update", "Can update categories"),
        ("categories:delete", "Can delete categories"),
        ("inventory:read", "Can read inventory"),
        ("inventory:update", "Can update inventory"),

        // Sales & Orders
        ("orders:create", "Can create orders/process sales"),
        ("orders:read", "Can read orders"),
        ("refunds:create", "Can create refunds"),
        ("refunds:read", "Can read refunds"),

        // Users & Roles
        ("employees:create", "Can create employees"),
        ("employees:read", "Can read employees"),
        ("employees:update", "Can update employees"),
        ("employees:delete", "Can delete employees"),
        ("customers:create", "Can create customers"),
        ("customers:read", "Can read customers"),
        ("customers:update", "Can update customers"),
        ("customers:delete", "Can delete customers"),
        ("roles:create", "Can create roles"),
        ("roles:read", "Can read roles"),
        ("roles:update", "Can update roles"),
        ("roles:delete", "Can delete roles"),
        ("permissions:assign", "Can assign permissions to roles"),
        ("permissions:read", "Can read permissions"),
        ("permissions:create", "Can create permissions"),
        ("permissions:update", "Can update permissions"),
        ("permissions:delete", "Can delete permissions"),

        // Miscellaneous
        ("reports:read", "Can view reports"),
        ("promotions:create", "Can create promotions"),
        ("promotions:read", "Can read promotions"),
        ("promotions:update", "Can update promotions"),
        ("promotions:delete", "Can delete promotions"),
        ("suppliers:create", "Can create suppliers"),
        ("suppliers:read", "Can read suppliers"),
        ("suppliers:update", "Can update suppliers"),
        ("suppliers:delete", "Can delete suppliers"),
        ("purchase_orders:create", "Can create purchase orders"),
        ("purchase_orders:read", "Can read purchase orders"),
        ("purchase_orders:update", "Can update purchase orders"),
        ("purchase_orders:delete", "Can delete purchase orders"),
        ("stores:create", "Can create stores"),
        ("stores:read", "Can read stores"),
        ("stores:update", "Can update stores"),
        ("stores:delete", "Can delete stores"),
    ];

    for (name, description) in permissions {
        let stmt = Statement::from_string(
            DbBackend::MySql,
            format!(
                "INSERT IGNORE INTO permissions (name, description) VALUES ('{}', '{}');",
                name, description
            ),
        );
        manager.get_connection().execute(stmt).await?;
    }
    Ok(())
}

async fn assign_permissions_to_role(manager: &SchemaManager<'_>, role_name: &str, permissions: Vec<&str>) -> Result<(), DbErr> {
    #[derive(FromQueryResult)]
    struct IdWrapper { id: i32, }

    let role_id: i32 = IdWrapper::find_by_statement(Statement::from_string(
        DbBackend::MySql,
        format!("SELECT id FROM roles WHERE name = '{}'", role_name),
    ))
    .one(manager.get_connection())
    .await?
    .unwrap()
    .id;

    for perm_name in permissions {
        let perm_id: i32 = IdWrapper::find_by_statement(Statement::from_string(
            DbBackend::MySql,
            format!("SELECT id FROM permissions WHERE name = '{}'", perm_name),
        ))
        .one(manager.get_connection())
        .await?
        .unwrap()
        .id;

        let stmt = Statement::from_string(
            DbBackend::MySql,
            format!(
                "INSERT IGNORE INTO role_permissions (role_id, permission_id) VALUES ({}, {});",
                role_id, perm_id
            ),
        );
        manager.get_connection().execute(stmt).await?;
    }
    Ok(())
}


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Seed initial roles
        let roles = vec![
            ("Owner", "System owner with full control"),
            ("Admin", "Administrator with broad management capabilities"),
            ("StoreManager", "Manages a specific store"),
            ("InventoryManager", "Manages inventory for a store"),
            ("Cashier", "Handles sales and customer transactions"),
        ];
        for (name, description) in roles {
            let stmt = Statement::from_string(
                DbBackend::MySql,
                format!(
                    "INSERT IGNORE INTO roles (name, description) VALUES ('{}', '{}');",
                    name, description
                ),
            );
            manager.get_connection().execute(stmt).await?;
        }

        // Seed initial permissions
        seed_permissions(manager).await?;

        // --- Assign permissions to roles ---

        // Owner gets all permissions
        let all_permissions_query = "SELECT name FROM permissions";
        let all_permission_names: Vec<String> = manager.get_connection()
            .query_all(Statement::from_string(DbBackend::MySql, all_permissions_query))
            .await?
            .into_iter()
            .map(|row| row.try_get("", "name").unwrap())
            .collect();
        assign_permissions_to_role(manager, "Owner", all_permission_names.iter().map(|s| s.as_str()).collect()).await?;
        
        // Admin gets most permissions
        assign_permissions_to_role(manager, "Admin", all_permission_names.iter().map(|s| s.as_str()).collect()).await?;


        // StoreManager permissions
        let store_manager_permissions = vec![
            "reports:read", "products:create", "products:read", "products:update", "products:delete",
            "inventory:read", "inventory:update", "orders:create", "orders:read", "refunds:create", "refunds:read",
            "suppliers:create", "suppliers:read", "suppliers:update", "suppliers:delete",
            "purchase_orders:create", "purchase_orders:read", "purchase_orders:update", "purchase_orders:delete",
            "employees:create", "employees:read", "employees:update", "employees:delete",
            "customers:create", "customers:read", "customers:update", "customers:delete",
            "promotions:create", "promotions:read", "promotions:update", "promotions:delete",
            "categories:read",
        ];
        assign_permissions_to_role(manager, "StoreManager", store_manager_permissions).await?;

        // InventoryManager permissions
        let inventory_manager_permissions = vec![
            "products:read", "inventory:read", "inventory:update", "suppliers:read",
            "purchase_orders:create", "purchase_orders:read", "categories:read",
        ];
        assign_permissions_to_role(manager, "InventoryManager", inventory_manager_permissions).await?;

        // Cashier permissions
        let cashier_permissions = vec![
            "orders:create", "orders:read", "promotions:read", "reports:read",
            "customers:create", "customers:read", "customers:update", "products:read",
        ];
        assign_permissions_to_role(manager, "Cashier", cashier_permissions).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "SET FOREIGN_KEY_CHECKS = 0;")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "TRUNCATE TABLE role_permissions;")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "TRUNCATE TABLE permissions;")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "TRUNCATE TABLE roles;")).await?;
        manager.get_connection().execute(Statement::from_string(DbBackend::MySql, "SET FOREIGN_KEY_CHECKS = 1;")).await?;
        Ok(())
    }
}
