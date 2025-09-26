use actix_web::web;
use crate::handler::order_items_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order_items")
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Owner".to_string(), "Admin".to_string(), "StoreManager".to_string(), "Cashier".to_string()],
                    })
                    .route("", web::get().to(order_items_handler::get_all_order_items))
                    .route("/{id}", web::get().to(order_items_handler::get_order_item_by_id))
                    .route("/{id}", web::put().to(order_items_handler::update_order_item))
            )
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["StoreManager".to_string(), "Cashier".to_string()],
                    })
                    // .route("", web::post().to(order_items_handler::create_order_item))
            )
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Owner".to_string(), "Admin".to_string(), "StoreManager".to_string()],
                    })
                    .route("/{id}", web::delete().to(order_items_handler::delete_order_item))
            )
    );
}
