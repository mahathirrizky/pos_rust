use actix_web::web;
use crate::handler::employees_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/employees")
            .route(
                "/admin",
                web::post()
                    .to(employees_handler::create_admin)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["employees:create".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(employees_handler::create_employee_general)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["employees:create".to_string()],
                    }),
            )
            .route(
                "",
                web::get()
                    .to(employees_handler::get_all_employees)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["employees:read".to_string()],
                    }),
            )
            .route(
                "/me",
                web::get()
                    .to(employees_handler::get_my_profile)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec![], // Requires authentication only
                    }),
            )
            .route(
                "/report",
                web::get()
                    .to(employees_handler::get_employee_report)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["reports:read".to_string()],
                    }),
            )
            .service(
                web::scope("/{id}")
                    .route(
                        "",
                        web::get()
                            .to(employees_handler::get_employee_by_id)
                            .wrap(PermissionMiddlewareFactory {
                                required_permissions: vec!["employees:read".to_string()],
                            }),
                    )
                    .route(
                        "",
                        web::put()
                            .to(employees_handler::update_employee)
                            .wrap(PermissionMiddlewareFactory {
                                required_permissions: vec!["employees:update".to_string()],
                            }),
                    )
                    .route(
                        "",
                        web::delete()
                            .to(employees_handler::delete_employee)
                            .wrap(PermissionMiddlewareFactory {
                                required_permissions: vec!["employees:delete".to_string()],
                            }),
                    ),
            ),
    );
}
