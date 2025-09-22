use actix_web::web;
use crate::handler::orders_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string(), "StoreManager".to_string(), "Cashier".to_string()],
                    })
                    .route("", web::get().to(orders_handler::get_all_orders))
                    .route("/{id}", web::get().to(orders_handler::get_order_by_id))
                    .route("/{id}", web::put().to(orders_handler::update_order))
                    .route("/report", web::get().to(orders_handler::get_sales_report))
            )
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["StoreManager".to_string(), "Cashier".to_string()],
                    })
                    .route("", web::post().to(orders_handler::create_order))
            )
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string(), "StoreManager".to_string()],
                    })
                    .route("/{id}", web::delete().to(orders_handler::delete_order))
            )
    );
}
