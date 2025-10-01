use actix_web::web;
use crate::handler::purchase_orders_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/purchase-orders")
            .route(
                "",
                web::get()
                    .to(purchase_orders_handler::get_all_purchase_orders)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["purchase_orders:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(purchase_orders_handler::get_purchase_order)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["purchase_orders:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(purchase_orders_handler::create_purchase_order)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["purchase_orders:create".to_string()],
                    }),
            )
            .route(
                "/{id}/items",
                web::post()
                    .to(purchase_orders_handler::add_item_to_purchase_order)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["purchase_orders:update".to_string()],
                    }),
            )
            .route(
                "/{id}/status",
                web::put()
                    .to(purchase_orders_handler::update_purchase_order_status)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["purchase_orders:update".to_string()],
                    }),
            )
            .route(
                "/{id}/receive",
                web::post()
                    .to(purchase_orders_handler::receive_stock)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["purchase_orders:update".to_string()],
                    }),
            ),
    );
}
