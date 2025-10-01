use actix_web::web;
use crate::handler::bills_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/bills")
            .route(
                "",
                web::get().to(bills_handler::get_all_bills).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["orders:read".to_string()],
                }),
            ),
    );
}
