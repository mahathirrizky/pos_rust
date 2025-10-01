use actix_web::web;
use crate::handler::dashboard_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/dashboard")
            .route(
                "/summary",
                web::get().to(dashboard_handler::get_summary).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["reports:read".to_string()],
                }),
            )
            .route(
                "/activities",
                web::get()
                    .to(dashboard_handler::get_activities)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["reports:read".to_string()],
                    }),
            )
            .route(
                "/sales",
                web::get().to(dashboard_handler::get_sales).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["reports:read".to_string()],
                }),
            ),
    );
}
