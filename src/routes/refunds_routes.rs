use actix_web::web;
use crate::handler::refunds_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/refunds")
            .wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string(), "Admin".to_string(), "StoreManager".to_string()],
            })
            .route("", web::post().to(refunds_handler::create_refund))
            .route("", web::get().to(refunds_handler::get_all_refunds))
            .route("/{id}", web::get().to(refunds_handler::get_refund_by_id)),
    );
}
