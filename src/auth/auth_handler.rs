use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::employees;
use crate::auth::auth_service;
use crate::repository::employees_repository::EmployeeRepository;
use serde::{Serialize, Deserialize};

use crate::repository::permissions_repository::PermissionsRepository;

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct LoginSuccessResponse {
    pub token: String,
    pub user: employees::Model,
    pub permissions: Vec<String>,
}

pub async fn login(db: web::Data<DatabaseConnection>, payload: web::Json<LoginPayload>) -> impl Responder {
    println!("Received login attempt: {:?}", payload);
    match EmployeeRepository::find_by_email(&db, payload.email.clone()).await {
        Ok(Some(user)) => {
            // First, verify the password
            match auth_service::verify_password(&payload.password, &user.password_hash) {
                Ok(true) => {
                    // After password is ok, check the role
                    if user.role != payload.role {
                        return HttpResponse::Unauthorized().json(ApiError::new("Invalid credentials for this role!".to_string()));
                    }

                    // If role is also ok, create token
                    match auth_service::create_jwt(&db, &user).await {
                        Ok(token) => {
                            match PermissionsRepository::find_permissions_for_role(&db, user.role_id).await {
                                Ok(permissions) => {
                                    let response_data = LoginSuccessResponse { token, user, permissions };
                                    println!("Sending success response");
                                    HttpResponse::Ok().json(ApiResponse::new(response_data))
                                }
                                Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch permissions".to_string())),
                            }
                        }
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
