use actix_web::web;
use crate::handler::refunds_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/refunds")
            .route(
                "",
                web::post()
                    .to(refunds_handler::create_refund)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["refunds:create".to_string()],
                    }),
            )
            .route(
                "",
                web::get()
                    .to(refunds_handler::get_all_refunds)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["refunds:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(refunds_handler::get_refund_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["refunds:read".to_string()],
                    }),
            ),
    );
}
