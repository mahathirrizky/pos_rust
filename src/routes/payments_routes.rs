use actix_web::web;
use crate::handler::payments_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string(), "StoreManager".to_string(), "Cashier".to_string()],
                    })
                    .route("", web::get().to(payments_handler::get_all_payments))
                    .route("/{id}", web::get().to(payments_handler::get_payment_by_id))
                    .route("/{id}", web::put().to(payments_handler::update_payment))
            )
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["StoreManager".to_string(), "Cashier".to_string()],
                    })
                    .route("", web::post().to(payments_handler::create_payment))
            )
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string(), "StoreManager".to_string()],
                    })
                    .route("/{id}", web::delete().to(payments_handler::delete_payment))
            )
    );
}
