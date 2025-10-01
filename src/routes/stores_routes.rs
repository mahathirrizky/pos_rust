use actix_web::web;
use crate::handler::stores_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stores")
            .route(
                "",
                web::get().to(stores_handler::get_all_stores).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["stores:read".to_string()],
                }),
            )
            .route(
                "",
                web::post().to(stores_handler::create_store).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["stores:create".to_string()],
                }),
            )
            .service(
                web::scope("/{id}")
                    .route(
                        "",
                        web::get().to(stores_handler::get_store_by_id).wrap(PermissionMiddlewareFactory {
                            required_permissions: vec!["stores:read".to_string()],
                        }),
                    )
                    .route(
                        "",
                        web::put().to(stores_handler::update_store).wrap(PermissionMiddlewareFactory {
                            required_permissions: vec!["stores:update".to_string()],
                        }),
                    )
                    .route(
                        "",
                        web::delete().to(stores_handler::delete_store).wrap(PermissionMiddlewareFactory {
                            required_permissions: vec!["stores:delete".to_string()],
                        }),
                    ),
            ),
    );
}
