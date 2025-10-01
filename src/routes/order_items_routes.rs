use actix_web::web;
use crate::handler::order_items_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order_items")
            .route(
                "",
                web::get()
                    .to(order_items_handler::get_all_order_items)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(order_items_handler::get_order_item_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(order_items_handler::update_order_item)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:create".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(order_items_handler::delete_order_item)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:create".to_string()],
                    }),
            ),
    );
}
