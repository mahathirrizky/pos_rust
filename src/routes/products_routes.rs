use actix_web::web;
use crate::handler::products_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route(
                "",
                web::get()
                    .to(products_handler::get_all_products)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["products:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(products_handler::create_product)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["products:create".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(products_handler::get_product_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["products:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(products_handler::update_product)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["products:update".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(products_handler::delete_product)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["products:delete".to_string()],
                    }),
            ),
    );
}
