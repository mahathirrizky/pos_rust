use crate::repository::inventory_repository::InventoryRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::inventory::{CreateInventory, UpdateInventory};
use sea_orm::DatabaseConnection;
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::inventory_guard::InventoryAccessGuard;

pub async fn get_all_inventory(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    if claims.0.role == "Admin" {
        match InventoryRepository::get_all(db.get_ref()).await {
            Ok(inventory) => HttpResponse::Ok().json(ApiResponse::new(inventory)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch inventory".to_string())),
        }
    } else {
        // Store Manager and Inventory Manager can view inventory in their store
        match InventoryRepository::get_all_by_store(db.get_ref(), claims.0.store_id).await {
            Ok(inventory) => HttpResponse::Ok().json(ApiResponse::new(inventory)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch inventory".to_string())),
        }
    }
}

pub async fn create_inventory(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, new_inventory: web::Json<CreateInventory>) -> impl Responder {
    let mut inventory_data = new_inventory.into_inner();
    // Ensure inventory is created for the user's store if not admin
    if claims.0.role != "Admin" {
        inventory_data.store_id = claims.0.store_id;
    }

    match InventoryRepository::create(db.get_ref(), inventory_data).await {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse::new(inventory)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create inventory".to_string())),
    }
}

pub async fn get_inventory_by_id(guard: InventoryAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.inventory))
}

pub async fn update_inventory(guard: InventoryAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdateInventory>) -> impl Responder {
    let item_id = guard.inventory.id;
    match InventoryRepository::update(db.get_ref(), item_id, update_data.into_inner()).await {
        Ok(Some(item)) => HttpResponse::Ok().json(ApiResponse::new(item)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Inventory not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update inventory".to_string())),
    }
}

pub async fn delete_inventory(guard: InventoryAccessGuard, db: web::Data<DatabaseConnection>) -> impl Responder {
    let item_id = guard.inventory.id;
    match InventoryRepository::delete(db.get_ref(), item_id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Inventory deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Inventory not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete inventory".to_string())),
    }
}
