use crate::repository::employees_repository::EmployeeRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::employees::{CreateEmployee, UpdateEmployee};
use sea_orm::DatabaseConnection;
use crate::guard::employee_guard::EmployeeAccessGuard;
use crate::auth::auth_service;

pub async fn get_all_employees(db: web::Data<DatabaseConnection>) -> impl Responder {
    match EmployeeRepository::get_all(db.get_ref()).await {
        Ok(employees) => HttpResponse::Ok().json(ApiResponse::new(employees)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch employees".to_string())),
    }
}

pub async fn create_employee(db: web::Data<DatabaseConnection>, new_employee: web::Json<CreateEmployee>) -> impl Responder {
    // Hash the password before saving
    let mut employee_data = new_employee.into_inner();
    let hashed_password = match auth_service::hash_password(&employee_data.password_hash) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(e.to_string())),
    };
    employee_data.password_hash = hashed_password;

    match EmployeeRepository::create(db.get_ref(), employee_data).await {
        Ok(employee) => HttpResponse::Ok().json(ApiResponse::new(employee)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create employee".to_string())),
    }
}

pub async fn get_employee_by_id(guard: EmployeeAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.employee))
}

pub async fn get_my_profile(guard: EmployeeAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.employee))
}

pub async fn update_employee(guard: EmployeeAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdateEmployee>) -> impl Responder {
    let employee_id = guard.employee.id;
    let mut employee_data = update_data.into_inner();

    // Hash password if it is provided in the update
    if let Some(password) = employee_data.password_hash.clone() {
        if !password.is_empty() {
            let hashed_password = match auth_service::hash_password(&password) {
                Ok(hash) => hash,
                Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(e.to_string())),
            };
            employee_data.password_hash = Some(hashed_password);
        } else {
            // If password is an empty string, keep the old one by setting it to None
            employee_data.password_hash = None;
        }
    }

    match EmployeeRepository::update(db.get_ref(), employee_id, employee_data).await {
        Ok(Some(employee)) => HttpResponse::Ok().json(ApiResponse::new(employee)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Employee not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update employee".to_string())),
    }
}

pub async fn delete_employee(db: web::Data<DatabaseConnection>, id: web::Path<i32>) -> impl Responder {
    let employee_id = id.into_inner(); // Extract once
    match EmployeeRepository::delete(db.get_ref(), employee_id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Employee deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Employee not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete employee".to_string())),
    }
}

