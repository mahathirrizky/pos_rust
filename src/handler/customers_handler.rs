use crate::repository::customers_repository::CustomerRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::customers::{CreateCustomer, UpdateCustomer};
use sea_orm::DatabaseConnection;

pub async fn get_all_customers(db: web::Data<DatabaseConnection>) -> impl Responder {
    match CustomerRepository::get_all(db.get_ref()).await {
        Ok(customers) => HttpResponse::Ok().json(ApiResponse::new(customers)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch customers".to_string())),
    }
}

pub async fn create_customer(db: web::Data<DatabaseConnection>, new_customer: web::Json<CreateCustomer>) -> impl Responder {
    match CustomerRepository::create(db.get_ref(), new_customer.into_inner()).await {
        Ok(customer) => HttpResponse::Ok().json(ApiResponse::new(customer)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create customer".to_string())),
    }
}

pub async fn get_customer_by_id(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    match CustomerRepository::find_by_id(db.get_ref(), id.into_inner()).await {
        Ok(Some(customer)) => HttpResponse::Ok().json(ApiResponse::new(customer)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Customer not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch customer".to_string())),
    }
}

pub async fn update_customer(db: web::Data<DatabaseConnection>, id: web::Path<i32>, update_data: web::Json<UpdateCustomer>) -> impl Responder {
    match CustomerRepository::update(db.get_ref(), id.into_inner(), update_data.into_inner()).await {
        Ok(Some(customer)) => HttpResponse::Ok().json(ApiResponse::new(customer)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Customer not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update customer".to_string())),
    }
}

pub async fn delete_customer(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    match CustomerRepository::delete(db.get_ref(), id.into_inner()).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Customer deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Customer not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete customer".to_string())),
    }
}