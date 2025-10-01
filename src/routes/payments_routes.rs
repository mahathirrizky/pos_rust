use actix_web::web;
use crate::handler::payments_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .route(
                "",
                web::get()
                    .to(payments_handler::get_all_payments)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(payments_handler::get_payment_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(payments_handler::update_payment)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:create".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(payments_handler::create_payment)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:create".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(payments_handler::delete_payment)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:create".to_string()],
                    }),
            ),
    );
}
