use actix_web::web;
use crate::handler::employees_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/employees")
            .route("/me", web::get().to(employees_handler::get_my_profile)) // New route
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string()],
                    })
                    .route("", web::get().to(employees_handler::get_all_employees))
                    .route("", web::post().to(employees_handler::create_employee))
                    .route("/{id}", web::delete().to(employees_handler::delete_employee)),
            )
            .route("/{id}", web::get().to(employees_handler::get_employee_by_id))
            .route("/{id}", web::put().to(employees_handler::update_employee)),
    );
}
