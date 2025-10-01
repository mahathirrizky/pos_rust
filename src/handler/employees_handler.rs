use crate::auth::auth_service::{self, Claims};
use crate::repository::employees_repository::EmployeeRepository;
use crate::repository::roles_repository::RoleRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::employees::{CreateEmployee, UpdateEmployee, EmployeeReportData, EmployeeResponse, CreateAdminPayload};
use sea_orm::DatabaseConnection;
use crate::guard::employee_guard::EmployeeAccessGuard;
use crate::repository::password_reset_tokens_repository::PasswordResetTokenRepository;
use crate::repository::settings_repository;
use crate::helper::email;

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
        Ok(employees) => {
            let employee_responses: Vec<EmployeeResponse> = employees.into_iter().map(EmployeeResponse::from).collect();
            HttpResponse::Ok().json(ApiResponse::new(employee_responses))
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch employees".to_string())),
    }
}

pub async fn create_employee_general(db: web::Data<DatabaseConnection>, new_employee: web::Json<CreateEmployee>, _claims: web::ReqData<Claims>) -> impl Responder {
    let employee_data = new_employee.into_inner();

    match EmployeeRepository::create(db.get_ref(), employee_data).await {
        Ok(employee) => {
            // --- Send Set Password Email ---
            let db_clone = db.get_ref().clone();
            let created_employee = employee.clone();
            tokio::spawn(async move {
                // 1. Generate Token
                let token = ::uuid::Uuid::new_v4().to_string();
                let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::hours(24);

                // 2. Store Token
                if let Err(e) = PasswordResetTokenRepository::create(&db_clone, created_employee.id, token.clone(), expires_at).await {
                    eprintln!("Failed to create password reset token: {}", e);
                    return; // Don't send email if token wasn't saved
                }

                // 3. Fetch Email Settings
                match settings_repository::get_settings(&db_clone).await {
                    Ok(settings) => {
                        // 4. Send Email
                        let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
                        let set_password_url = format!("{}/set-password?token={}", frontend_url, token);

                        let subject = "Set Your Account Password";
                        let body = format!(
                            "Hello {},\n\nWelcome! Your account has been created. Please set your password by clicking the link below:\n\n{}\n\nThe link will expire in 24 hours.",
                            created_employee.first_name,
                            set_password_url
                        );

                        if let Err(e) = email::send_email(
                            &settings.email,
                            &created_employee.email,
                            Some(&created_employee.first_name),
                            subject,
                            &body,
                        ).await {
                            eprintln!("Failed to send set-password email: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch settings for sending email: {}", e);
                    }
                }
            });
            // --- End Email Logic ---

            let employee_response = EmployeeResponse::from(employee);
            HttpResponse::Ok().json(ApiResponse::new(employee_response))
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to create employee: {}", e.to_string()))),
    }
}

pub async fn create_admin(db: web::Data<DatabaseConnection>, new_admin_payload: web::Json<CreateAdminPayload>) -> impl Responder {
    // Find the Admin role ID
    let admin_role = match RoleRepository::find_by_name(db.get_ref(), "Admin".to_string()).await {
        Ok(Some(role)) => role,
        Ok(None) => return HttpResponse::InternalServerError().json(ApiError::new("Admin role not found in database".to_string())),
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to query admin role: {}", e.to_string()))),
    };

    let payload = new_admin_payload.into_inner();

    let employee_data = CreateEmployee {
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        phone: payload.phone,
        store_id: payload.store_id,
        role_id: admin_role.id,
        photo_url: payload.photo_url,
    };

    match EmployeeRepository::create(db.get_ref(), employee_data).await {
        Ok(employee) => {
            // --- Send Set Password Email ---
            let db_clone = db.get_ref().clone();
            let created_employee = employee.clone();
            tokio::spawn(async move {
                // 1. Generate Token
                let token = ::uuid::Uuid::new_v4().to_string();
                let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::hours(24);

                // 2. Store Token
                if let Err(e) = PasswordResetTokenRepository::create(&db_clone, created_employee.id, token.clone(), expires_at).await {
                    eprintln!("Failed to create password reset token: {}", e);
                    return; // Don't send email if token wasn't saved
                }

                // 3. Fetch Email Settings
                match settings_repository::get_settings(&db_clone).await {
                    Ok(settings) => {
                        // 4. Send Email
                        let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
                        let set_password_url = format!("{}/set-password?token={}", frontend_url, token);

                        let subject = "Set Your Account Password";
                        let body = format!(
                            "Hello {},\n\nWelcome! Your account has been created. Please set your password by clicking the link below:\n\n{}\n\nThe link will expire in 24 hours.",
                            created_employee.first_name,
                            set_password_url
                        );

                        if let Err(e) = email::send_email(
                            &settings.email,
                            &created_employee.email,
                            Some(&created_employee.first_name),
                            subject,
                            &body,
                        ).await {
                            eprintln!("Failed to send set-password email: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch settings for sending email: {}", e);
                    }
                }
            });
            // --- End Email Logic ---

            let employee_response = EmployeeResponse::from(employee);
            HttpResponse::Ok().json(ApiResponse::new(employee_response))
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to create admin: {}", e.to_string()))),
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
    if let Some(password) = employee_data.password.clone() {
        if !password.is_empty() {
            let hashed_password = match auth_service::hash_password(&password) {
                Ok(hash) => hash,
                Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(e.to_string())),
            };
            employee_data.password = Some(hashed_password);
        } else {
            // If password is an empty string, do not update it.
            employee_data.password = None;
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

