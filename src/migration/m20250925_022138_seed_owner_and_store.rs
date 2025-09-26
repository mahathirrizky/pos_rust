use sea_orm_migration::prelude::*;
use sea_orm::{entity::*, query::*};
use chrono::Utc;

// Import the hashing function from your auth service
use crate::auth::auth_service::hash_password;
use crate::entities::{employees, stores};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Hash the password using the project's own function
        let password = "password123";
        let hashed_password = hash_password(password)?;

        // 1. Seed the Owner first, without a store_id
        let owner = employees::ActiveModel {
            first_name: Set("Default".to_owned()),
            last_name: Set("Owner".to_owned()),
            email: Set("owner@example.com".to_owned()),
            password_hash: Set(hashed_password),
            role: Set("Owner".to_owned()),
            store_id: Set(None), // Set store_id to NULL for the owner
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let owner_res = employees::Entity::insert(owner).exec(db).await?;

        // 2. Seed the first Store
        let store = stores::ActiveModel {
            name: Set("Main Branch".to_owned()),
            address: Set(Some("123 Main Street".to_owned())),
            phone: Set(Some("555-0101".to_owned())),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let store_res = stores::Entity::insert(store).exec(db).await?;

        // 3. Update the owner to be associated with the new store
        let mut owner_active_model: employees::ActiveModel = employees::Entity::find_by_id(owner_res.last_insert_id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Failed to find owner after creation".into()))?
            .into();

        owner_active_model.store_id = Set(Some(store_res.last_insert_id));
        owner_active_model.update(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        
        stores::Entity::delete_many()
            .filter(stores::Column::Name.eq("Main Branch"))
            .exec(db)
            .await?;

        employees::Entity::delete_many()
            .filter(employees::Column::Email.eq("owner@example.com"))
            .exec(db)
            .await?;

        Ok(())
    }
}
