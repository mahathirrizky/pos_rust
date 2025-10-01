use actix_web::web;
use crate::handler::categories_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route(
                "",
                web::get()
                    .to(categories_handler::get_all_categories)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["categories:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(categories_handler::get_category_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["categories:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(categories_handler::create_category)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["categories:create".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(categories_handler::update_category)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["categories:update".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(categories_handler::delete_category)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["categories:delete".to_string()],
                    }),
            ),
    );
}
