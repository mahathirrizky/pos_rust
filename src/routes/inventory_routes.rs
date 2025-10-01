use actix_web::web;
use crate::handler::inventory_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/inventory")
            .route(
                "",
                web::get()
                    .to(inventory_handler::get_all_inventory)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["inventory:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(inventory_handler::create_inventory)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["inventory:update".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(inventory_handler::get_inventory_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["inventory:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(inventory_handler::update_inventory)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["inventory:update".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(inventory_handler::delete_inventory)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["inventory:update".to_string()],
                    }),
            )
            .route(
                "/report",
                web::get()
                    .to(inventory_handler::get_inventory_report)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["reports:read".to_string()],
                    }),
            ),
    );
}
