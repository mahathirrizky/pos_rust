use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::employees;
use crate::auth::auth_service;
use crate::repository::employees_repository::EmployeeRepository;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub employee: employees::Model,
}

pub async fn login(db: web::Data<DatabaseConnection>, payload: web::Json<LoginPayload>) -> impl Responder {
    match EmployeeRepository::find_by_email(&db, payload.email.clone()).await {
        Ok(Some(employee)) => {
            match auth_service::verify_password(&payload.password, &employee.password_hash) {
                Ok(true) => {
                    match auth_service::create_jwt(&employee) {
                        Ok(token) => HttpResponse::Ok().json(ApiResponse::new(LoginResponse { token, employee })),
                        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create token".to_string())),
                    }
                }
                Ok(false) => HttpResponse::Unauthorized().json(ApiError::new("Invalid credentials".to_string())),
                Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Password verification failed".to_string())),
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json(ApiError::new("Invalid credentials".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch employee".to_string())),
    }
}
