use crate::auth::auth_service::Claims;
use crate::repository::employees_repository::EmployeeRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::employees::{CreateEmployee, UpdateEmployee, EmployeeReportData, EmployeeResponse};
use sea_orm::DatabaseConnection;
use crate::guard::employee_guard::EmployeeAccessGuard;
use crate::auth::auth_service;
// use crate::guard::role_guard::{Claims, has_role, ErrorResponse as RoleErrorResponse};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmployeeFilter {
    pub role: Option<String>,
    pub store_id: Option<i32>,
    #[serde(rename = "roles_to_exclude")]
    pub roles_to_exclude: Option<String>,
}

pub async fn get_all_employees(db: web::Data<DatabaseConnection>, query: web::Query<EmployeeFilter>, claims: web::ReqData<Claims>) -> impl Responder {
    let roles_to_exclude_vec = query.roles_to_exclude.as_ref().map(|s| {
        s.split(',').map(|role_str| role_str.trim().to_string()).collect()
    });

    let effective_role_filter = query.role.clone();
    let mut effective_store_id_filter = query.store_id;

    // Apply role-based filtering
    if claims.role == "StoreManager" {
        // StoreManager can only see employees in their own store
        if let Some(store_id) = claims.store_id {
            effective_store_id_filter = Some(store_id);
        } else {
            // If StoreManager has no store_id, they shouldn't see any employees
            return HttpResponse::Forbidden().json(ApiError::new("StoreManager has no assigned store".to_string()));
        }
        // StoreManager can only see roles they manage (e.g., Cashier, InventoryManager)
        // For now, let's assume they can only see employees in their store, regardless of role filter from query
        // If you want to restrict roles they can see, add logic here.
    } else if claims.role == "Admin" || claims.role == "Owner" {
        // Admin and Owner can see all employees (subject to query filters)
        // No additional restrictions needed here based on their own store_id
    }

    match EmployeeRepository::get_all(db.get_ref(), effective_role_filter, effective_store_id_filter, roles_to_exclude_vec).await {
        Ok(employees) => HttpResponse::Ok().json(ApiResponse::new(employees)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch employees".to_string())),
    }
}

pub async fn create_admin(db: web::Data<DatabaseConnection>, new_employee: web::Json<CreateEmployee>) -> impl Responder {
    let mut employee_data = new_employee.into_inner();
    let hashed_password = match auth_service::hash_password(&employee_data.password_hash) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(e.to_string())),
    };
    employee_data.password_hash = hashed_password;

    match EmployeeRepository::create(db.get_ref(), employee_data, "Admin".to_string()).await {
        Ok(employee) => {
            let employee_response = EmployeeResponse::from(employee);
            HttpResponse::Ok().json(ApiResponse::new(employee_response))
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create admin".to_string())),
    }
}

pub async fn get_employee_by_id(guard: EmployeeAccessGuard) -> impl Responder {
    let employee_response = EmployeeResponse::from(guard.employee);
    HttpResponse::Ok().json(ApiResponse::new(employee_response))
}

pub async fn get_my_profile(guard: EmployeeAccessGuard) -> impl Responder {
    let employee_response = EmployeeResponse::from(guard.employee);
    HttpResponse::Ok().json(ApiResponse::new(employee_response))
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
        Ok(Some(employee)) => {
            let employee_response = EmployeeResponse::from(employee);
            HttpResponse::Ok().json(ApiResponse::new(employee_response))
        },
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

pub async fn get_employee_report(
    db: web::Data<DatabaseConnection>,
    claims: web::ReqData<Claims>,
) -> impl Responder {
    let employees_result = if claims.role == "Admin" || claims.role == "Owner" {
        EmployeeRepository::get_all(db.get_ref(), None, None, None).await
    } else if claims.role == "StoreManager" {
        if let Some(store_id) = claims.store_id {
            EmployeeRepository::get_all_by_store(db.get_ref(), store_id).await
        } else {
            return HttpResponse::Forbidden().json(ApiError::new("StoreManager has no assigned store".to_string()));
        }
    } else {
        // Other roles like Cashier cannot access this report
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    };

    match employees_result {
        Ok(employees) => {
            let report_data: Vec<EmployeeReportData> = employees.into_iter().map(EmployeeReportData::from).collect();
            HttpResponse::Ok().json(ApiResponse::new(report_data))
        }
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch employee report".to_string())),
    }
}

