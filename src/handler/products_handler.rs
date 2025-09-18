use crate::repository::products_repository::ProductRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::products::{CreateProduct, UpdateProduct};
use sea_orm::DatabaseConnection;
use crate::extractor::claims_extractor::ClaimsExtractor;

pub async fn get_all_products(_claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    // All authenticated users can view products
    match ProductRepository::get_all(db.get_ref()).await {
        Ok(products) => HttpResponse::Ok().json(ApiResponse::new(products)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch products".to_string())),
    }
}

pub async fn create_product(_claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, new_product: web::Json<CreateProduct>) -> impl Responder {
    match ProductRepository::create(db.get_ref(), new_product.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(ApiResponse::new(product)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create product".to_string())),
    }
}

pub async fn get_product_by_id(_claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    // All authenticated users can view products
    match ProductRepository::find_by_id(db.get_ref(), id.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(ApiResponse::new(product)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Product not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch product".to_string())),
    }
}

pub async fn update_product(_claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, id: web::Path<i32>, update_data: web::Json<UpdateProduct>) -> impl Responder {
    match ProductRepository::update(db.get_ref(), id.into_inner(), update_data.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(ApiResponse::new(product)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Product not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update product".to_string())),
    }
}

pub async fn delete_product(_claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    match ProductRepository::delete(db.get_ref(), id.into_inner()).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Product deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Product not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete product".to_string())),
    }
}