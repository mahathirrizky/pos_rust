use actix_web::web;
use crate::handler::customers_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/customers")
            .wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Admin".to_string()],
            })
            .route("", web::get().to(customers_handler::get_all_customers))
            .route("", web::post().to(customers_handler::create_customer))
            .route("/{id}", web::get().to(customers_handler::get_customer_by_id))
            .route("/{id}", web::put().to(customers_handler::update_customer))
            .route("/{id}", web::delete().to(customers_handler::delete_customer)),
    );
}
