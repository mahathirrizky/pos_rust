use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;

use crate::repository::promotions_repository::PromotionRepository;
use crate::entities::promotions::{CreatePromotion, UpdatePromotion};
use crate::helper::response::{ApiResponse, ApiError};
// use crate::guard::role_guard::{Claims, has_role, ErrorResponse as RoleErrorResponse};

pub async fn create_promotion(
    db: web::Data<DatabaseConnection>,
    // claims: Claims,
    payload: web::Json<CreatePromotion>,
) -> impl Responder {
    // if !has_role(&claims, &["Admin"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Only Admin can create promotions.".to_string()));
    // }
    match PromotionRepository::create(db.get_ref(), payload.into_inner()).await {
        Ok(promotion) => HttpResponse::Ok().json(ApiResponse::new(promotion)),
        Err(e) => HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to create promotion: {}", e))),
    }
}

pub async fn get_all_promotions(
    // claims: Claims,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    // if !has_role(&claims, &["Admin", "Owner"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    // }
    match PromotionRepository::get_all(db.get_ref()).await {
        Ok(promotions) => HttpResponse::Ok().json(ApiResponse::new(promotions)),
        Err(e) => HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch promotions: {}", e))),
    }
}

pub async fn get_promotion_by_id(
    // claims: Claims,
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> impl Responder {
    // if !has_role(&claims, &["Admin", "Owner"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    // }
    let id = path.into_inner();
    match PromotionRepository::find_by_id(db.get_ref(), id).await {
        Ok(Some(promotion)) => HttpResponse::Ok().json(ApiResponse::new(promotion)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new(format!("Promotion with ID {} not found", id))),
        Err(e) => HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch promotion: {}", e))),
    }
}

pub async fn update_promotion(
    db: web::Data<DatabaseConnection>,
    // claims: Claims,
    path: web::Path<i32>,
    payload: web::Json<UpdatePromotion>,
) -> impl Responder {
    // if !has_role(&claims, &["Admin"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Only Admin can update promotions.".to_string()));
    // }
    let id = path.into_inner();
    match PromotionRepository::update(db.get_ref(), id, payload.into_inner()).await {
        Ok(promotion) => HttpResponse::Ok().json(ApiResponse::new(promotion)),
        Err(e) => HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to update promotion: {}", e))),
    }
}

pub async fn delete_promotion(
    db: web::Data<DatabaseConnection>,
    // claims: Claims,
    path: web::Path<i32>,
) -> impl Responder {
    // if !has_role(&claims, &["Admin"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Only Admin can delete promotions.".to_string()));
    // }
    let id = path.into_inner();
    match PromotionRepository::delete(db.get_ref(), id).await {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                HttpResponse::Ok().json(ApiResponse::new(format!("Promotion with ID {} deleted successfully", id)))
            } else {
                HttpResponse::NotFound().json(ApiError::new(format!("Promotion with ID {} not found", id)))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to delete promotion: {}", e))),
    }
}
