use crate::repository::suppliers_repository::SupplierRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::suppliers::{CreateSupplier, UpdateSupplier};
use sea_orm::DatabaseConnection;

pub async fn get_all_suppliers(db: web::Data<DatabaseConnection>) -> impl Responder {
    // All authenticated users can view suppliers (assuming suppliers are global)
    match SupplierRepository::get_all(db.get_ref()).await {
        Ok(suppliers) => HttpResponse::Ok().json(ApiResponse::new(suppliers)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch suppliers".to_string())),
    }
}

pub async fn create_supplier(db: web::Data<DatabaseConnection>, new_supplier: web::Json<CreateSupplier>) -> impl Responder {
    match SupplierRepository::create(db.get_ref(), new_supplier.into_inner()).await {
        Ok(supplier) => HttpResponse::Ok().json(ApiResponse::new(supplier)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create supplier".to_string())),
    }
}

pub async fn get_supplier_by_id(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    // All authenticated users can view suppliers (assuming suppliers are global)
    match SupplierRepository::find_by_id(db.get_ref(), id.into_inner()).await {
        Ok(Some(supplier)) => HttpResponse::Ok().json(ApiResponse::new(supplier)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Supplier not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch supplier".to_string())),
    }
}

pub async fn update_supplier(db: web::Data<DatabaseConnection>, id: web::Path<i32>, update_data: web::Json<UpdateSupplier>) -> impl Responder {
    match SupplierRepository::update(db.get_ref(), id.into_inner(), update_data.into_inner()).await {
        Ok(Some(supplier)) => HttpResponse::Ok().json(ApiResponse::new(supplier)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Supplier not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update supplier".to_string())),
    }
}

pub async fn delete_supplier(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    match SupplierRepository::delete(db.get_ref(), id.into_inner()).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Supplier deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Supplier not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete supplier".to_string())),
    }
}
