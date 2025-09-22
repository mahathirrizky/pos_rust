use crate::repository::products_repository::ProductRepository;
use actix_web::{web, HttpResponse};
use crate::helper::response::{ApiResponse, AppError};
use crate::entities::products::{CreateProduct, UpdateProduct};
use sea_orm::DatabaseConnection;
use crate::extractor::claims_extractor::ClaimsExtractor;

pub async fn get_all_products(
    _claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, AppError> {
    let products = ProductRepository::get_all(db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(products)))
}

pub async fn create_product(
    _claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
    new_product: web::Json<CreateProduct>,
) -> Result<HttpResponse, AppError> {
    let product = ProductRepository::create(db.get_ref(), new_product.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(product)))
}

pub async fn get_product_by_id(
    _claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let product = ProductRepository::find_by_id(db.get_ref(), id.into_inner())
        .await?
        .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(product)))
}

pub async fn update_product(
    _claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    update_data: web::Json<UpdateProduct>,
) -> Result<HttpResponse, AppError> {
    let product = ProductRepository::update(db.get_ref(), id.into_inner(), update_data.into_inner())
        .await?
        .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(product)))
}

pub async fn delete_product(
    _claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let rows_affected = ProductRepository::delete(db.get_ref(), id.into_inner()).await?;

    if rows_affected > 0 {
        Ok(HttpResponse::Ok().json(ApiResponse::new("Product deleted successfully".to_string())))
    } else {
        Err(AppError::NotFound("Product not found".to_string()))
    }
}
