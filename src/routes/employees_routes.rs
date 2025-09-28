use actix_web::web;
use crate::handler::employees_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/employees")
            // Endpoint for Owner to create an Admin
            .route("/admin", web::post().to(employees_handler::create_admin).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string()],
            }))

            // Endpoint for Owner and Admin to create other employee roles
            .route("", web::post().to(employees_handler::create_employee_general).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string(), "Admin".to_string()],
            }))

            // GET all employees (with optional role filter)
            .route("", web::get().to(employees_handler::get_all_employees).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string(), "Admin".to_string(), "StoreManager".to_string()],
            }))
            
            .route("/me", web::get().to(employees_handler::get_my_profile))
            .route("/report", web::get().to(employees_handler::get_employee_report).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string(), "Admin".to_string(), "StoreManager".to_string()],
            }))
            
            .service(
                web::scope("/{id}")
                    .route("", web::get().to(employees_handler::get_employee_by_id).wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Owner".to_string(), "Admin".to_string(), "StoreManager".to_string()],
                    }))
                    .route("", web::put().to(employees_handler::update_employee).wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string(), "StoreManager".to_string()],
                    }))
                    .route("", web::delete().to(employees_handler::delete_employee).wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string(), "StoreManager".to_string()],
                    }))
            )
    );
}
