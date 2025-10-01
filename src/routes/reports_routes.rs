use actix_web::web;
use crate::handler::reports_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reports")
            .route(
                "/sales",
                web::get()
                    .to(reports_handler::get_sales_summary)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["reports:read".to_string()],
                    }),
            )
            .route(
                "/summary",
                web::get()
                    .to(reports_handler::get_reports_summary)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["reports:read".to_string()],
                    }),
            ),
    );
}
