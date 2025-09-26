use crate::repository::categories_repository::CategoryRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::categories::{CreateCategory, UpdateCategory};
use sea_orm::DatabaseConnection;
// use crate::guard::role_guard::{Claims, has_role, ErrorResponse as RoleErrorResponse};

pub async fn get_all_categories(db: web::Data<DatabaseConnection>) -> impl Responder {
    // if !has_role(&claims, &["Admin", "Owner"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    // }
    match CategoryRepository::get_all(db.get_ref()).await {
        Ok(categories) => HttpResponse::Ok().json(ApiResponse::new(categories)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch categories".to_string())),
    }
}

pub async fn create_category(db: web::Data<DatabaseConnection>, new_category: web::Json<CreateCategory>) -> impl Responder {
    // if !has_role(&claims, &["Admin"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Only Admin can create categories.".to_string()));
    // }
    match CategoryRepository::create(db.get_ref(), new_category.into_inner()).await {
        Ok(category) => HttpResponse::Ok().json(ApiResponse::new(category)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create category".to_string())),
    }
}

pub async fn get_category_by_id(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    // if !has_role(&claims, &["Admin", "Owner"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    // }
    match CategoryRepository::find_by_id(db.get_ref(), id.into_inner()).await {
        Ok(Some(category)) => HttpResponse::Ok().json(ApiResponse::new(category)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Category not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch category".to_string())),
    }
}

pub async fn update_category(db: web::Data<DatabaseConnection>, id: web::Path<i32>, update_data: web::Json<UpdateCategory>) -> impl Responder {
    // if !has_role(&claims, &["Admin"]) {
    //     return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Only Admin can update categories.".to_string()));
    // }
    match CategoryRepository::update(db.get_ref(), id.into_inner(), update_data.into_inner()).await {
        Ok(Some(category)) => HttpResponse::Ok().json(ApiResponse::new(category)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Category not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update category".to_string())),
    }
}

pub async fn delete_category(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    match CategoryRepository::delete(db.get_ref(), id.into_inner()).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Category deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Category not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete category".to_string())),
    }
}