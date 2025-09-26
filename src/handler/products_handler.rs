use crate::repository::products_repository::ProductRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::guard::product_guard::ProductAccessGuard;
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::products::{CreateProduct, UpdateProduct};
use sea_orm::DatabaseConnection;

pub async fn get_all_products(db: web::Data<DatabaseConnection>) -> impl Responder {
    // TODO: Re-implement authorization with actix middleware
    match ProductRepository::get_all(db.get_ref()).await {
        Ok(products) => HttpResponse::Ok().json(ApiResponse::new(products)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch products".to_string())),
    }
}

pub async fn create_product(
    db: web::Data<DatabaseConnection>,
    new_product: web::Json<CreateProduct>,
) -> impl Responder {
    // TODO: Re-implement authorization with actix middleware
    match ProductRepository::create(db.get_ref(), new_product.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(ApiResponse::new(product)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create product".to_string())),
    }
}

pub async fn get_product_by_id(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> impl Responder {
    // TODO: Re-implement authorization with actix middleware
    match ProductRepository::find_by_id(db.get_ref(), id.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(ApiResponse::new(product)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Product not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch product".to_string())),
    }
}

pub async fn update_product(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    update_data: web::Json<UpdateProduct>,
) -> impl Responder {
    // TODO: Re-implement authorization with actix middleware
    match ProductRepository::update(db.get_ref(), id.into_inner(), update_data.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(ApiResponse::new(product)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Product not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update product".to_string())),
    }
}

pub async fn delete_product(
    db: web::Data<DatabaseConnection>,
    guard: ProductAccessGuard,
) -> impl Responder {
    // Only Admin and Owner should be able to delete products
    if guard.claims.role != "Admin" && guard.claims.role != "Owner" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges to delete product".to_string()));
    }
    match ProductRepository::delete(db.get_ref(), guard.product.id).await {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                HttpResponse::Ok().json(ApiResponse::new("Product deleted successfully".to_string()))
            } else {
                HttpResponse::NotFound().json(ApiError::new("Product not found".to_string()))
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete product".to_string())),
    }
}
