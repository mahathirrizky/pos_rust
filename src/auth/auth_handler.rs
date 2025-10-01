use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::employees;
use crate::auth::auth_service;
use crate::repository::employees_repository::EmployeeRepository;
use crate::repository::settings_repository;
use crate::helper::email;

use serde::{Serialize, Deserialize};

use crate::repository::permissions_repository::PermissionsRepository;
use crate::repository::password_reset_tokens_repository::PasswordResetTokenRepository;

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

#[derive(Deserialize, Debug)]
pub struct SetPasswordPayload {
    pub token: String,
    pub new_password: String,
}

#[derive(Deserialize, Debug)]
pub struct ForgotPasswordPayload {
    pub email: String,
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
                                    let permissions: Vec<String> = permissions.into_iter().map(|p| p.name).collect();
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

pub async fn set_password(db: web::Data<DatabaseConnection>, payload: web::Json<SetPasswordPayload>) -> impl Responder {
    let db = db.get_ref();
    // 1. Find the token
    match PasswordResetTokenRepository::find_by_token(db, payload.token.clone()).await {
        Ok(Some(token_model)) => {
            // 2. Check if expired
            if chrono::Utc::now().naive_utc() > token_model.expires_at {
                // Clean up expired token
                let _ = PasswordResetTokenRepository::delete(db, token_model.id).await;
                return HttpResponse::BadRequest().json(ApiError::new("Token has expired.".to_string()));
            }

            // 3. Hash the new password
            let hashed_password = match auth_service::hash_password(&payload.new_password) {
                Ok(hash) => hash,
                Err(_) => return HttpResponse::InternalServerError().json(ApiError::new("Failed to hash password.".to_string())),
            };

            // 4. Update employee's password
            let employee_update = employees::UpdateEmployee {
                password: Some(hashed_password),
                ..Default::default() // Ensure other fields are not updated
            };

            match EmployeeRepository::update(db, token_model.employee_id, employee_update).await {
                Ok(Some(_)) => {
                    // 5. Delete the token so it can't be reused
                    let _ = PasswordResetTokenRepository::delete(db, token_model.id).await;
                    HttpResponse::Ok().json(ApiResponse::new("Password has been set successfully.".to_string()))
                }
                Ok(None) => HttpResponse::NotFound().json(ApiError::new("Employee not found.".to_string())),
                Err(e) => {
                    eprintln!("Failed to update password: {}", e);
                    HttpResponse::InternalServerError().json(ApiError::new("Failed to update password.".to_string()))
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Invalid or expired token.".to_string())),
        Err(e) => {
            eprintln!("Database error while finding token: {}", e);
            HttpResponse::InternalServerError().json(ApiError::new("An error occurred.".to_string()))
        }
    }
}

pub async fn forgot_password(db: web::Data<DatabaseConnection>, payload: web::Json<ForgotPasswordPayload>) -> impl Responder {
    let db = db.get_ref();

    // Attempt to find the user, but don't let the outcome affect the response time or result.
    if let Ok(Some(employee)) = EmployeeRepository::find_by_email(db, payload.email.clone()).await {
        // User found, so we spawn a background task to do the work.
        let db_clone = db.clone();
        tokio::spawn(async move {
            
            // 1. Generate Token
            let token = ::uuid::Uuid::new_v4().to_string();
            let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::hours(1); // Shorter expiry for password resets

            // 2. Store Token
            if let Err(e) = PasswordResetTokenRepository::create(&db_clone, employee.id, token.clone(), expires_at).await {
                eprintln!("Failed to create password reset token for forgot password: {}", e);
                return;
            }

            // 3. Fetch Email Settings
            match settings_repository::get_settings(&db_clone).await {
                Ok(settings) => {
                    // 4. Send Email
                    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
                    let set_password_url = format!("{}/set-password?token={}", frontend_url, token);

                    let subject = "Reset Your Password";
                    let body = format!(
                        "Hello {},\n\nYou requested to reset your password. Please click the link below to set a new one:\n\n{}\n\nThe link will expire in 1 hour.",
                        employee.first_name,
                        set_password_url
                    );

                    if let Err(e) = email::send_email(
                        &settings.email,
                        &employee.email,
                        Some(&employee.first_name),
                        subject,
                        &body,
                    ).await {
                        eprintln!("Failed to send forgot-password email: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch settings for sending forgot-password email: {}", e);
                }
            }
        });
    }

    // Always return a success response to prevent email enumeration.
    HttpResponse::Ok().json(ApiResponse::new("If an account with that email exists, a password reset link has been sent.".to_string()))
}


