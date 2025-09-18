use crate::repository::stores_repository::StoreRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::stores::{CreateStore, UpdateStore};
use sea_orm::DatabaseConnection;
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::store_guard::StoreAccessGuard;

pub async fn get_all_stores(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    if claims.0.role == "Admin" {
        match StoreRepository::get_all(db.get_ref()).await {
            Ok(stores) => HttpResponse::Ok().json(ApiResponse::new(stores)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch stores".to_string())),
        }
    } else {
        // Store Managers and other employees can only see their own store
        match StoreRepository::find_by_id(db.get_ref(), claims.0.store_id).await {
            Ok(Some(store)) => HttpResponse::Ok().json(ApiResponse::new(vec![store])), // Return as a vec for consistency
            Ok(None) => HttpResponse::NotFound().json(ApiError::new("Store not found for this employee".to_string())),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch store".to_string())),
        }
    }
}

pub async fn create_store(db: web::Data<DatabaseConnection>, new_store: web::Json<CreateStore>) -> impl Responder {
    match StoreRepository::create(db.get_ref(), new_store.into_inner()).await {
        Ok(store) => HttpResponse::Ok().json(ApiResponse::new(store)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create store".to_string())),
    }
}

pub async fn get_store_by_id(guard: StoreAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.store))
}

pub async fn update_store(guard: StoreAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdateStore>) -> impl Responder {
    let store_id = guard.store.id;
    match StoreRepository::update(db.get_ref(), store_id, update_data.into_inner()).await {
        Ok(Some(store)) => HttpResponse::Ok().json(ApiResponse::new(store)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Store not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update store".to_string())),
    }
}

pub async fn delete_store(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    match StoreRepository::delete(db.get_ref(), id.into_inner()).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Store deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Store not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete store".to_string())),
    }
}
