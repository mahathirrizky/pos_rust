use actix_web::web;
use crate::handler::orders_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route(
                "",
                web::get()
                    .to(orders_handler::get_all_orders)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(orders_handler::get_order_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(orders_handler::update_order)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:create".to_string()],
                    }),
            )
            .route(
                "/report",
                web::get()
                    .to(orders_handler::get_sales_report)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["reports:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(orders_handler::create_order)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:create".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(orders_handler::delete_order)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["orders:create".to_string()],
                    }),
            ),
    );
}
