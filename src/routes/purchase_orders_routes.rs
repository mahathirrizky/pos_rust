use actix_web::web;
use crate::handler::purchase_orders_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let write_roles = vec!["Admin".to_string(), "Inventory Manager".to_string(), "Store Manager".to_string()];
    let read_roles = vec!["Owner".to_string(), "Admin".to_string(), "Inventory Manager".to_string(), "Store Manager".to_string()];

    cfg.service(
        web::scope("/purchase-orders")
            .route("", web::get().to(purchase_orders_handler::get_all_purchase_orders))
            .route("/{id}", web::get().to(purchase_orders_handler::get_purchase_order))
            .wrap(RoleMiddlewareFactory { allowed_roles: read_roles.clone() })
    );

    cfg.service(
        web::scope("/purchase-orders")
            .route("", web::post().to(purchase_orders_handler::create_purchase_order))
            .route("/{id}/items", web::post().to(purchase_orders_handler::add_item_to_purchase_order))
            .route("/{id}/status", web::put().to(purchase_orders_handler::update_purchase_order_status))
            .route("/{id}/receive", web::post().to(purchase_orders_handler::receive_stock))
            .wrap(RoleMiddlewareFactory { allowed_roles: write_roles.clone() })
    );
}
