use actix_web::web;
use crate::handler::bills_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/bills")
            .wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Admin".to_string(), "Owner".to_string()],
            })
            .route("", web::get().to(bills_handler::get_all_bills)),
    );
}
