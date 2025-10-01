use actix_web::web;
use crate::handler::customers_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/customers")
            .route(
                "",
                web::get()
                    .to(customers_handler::get_all_customers)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["customers:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(customers_handler::create_customer)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["customers:create".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(customers_handler::get_customer_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["customers:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(customers_handler::update_customer)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["customers:update".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(customers_handler::delete_customer)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["customers:delete".to_string()],
                    }),
            ),
    );
}
