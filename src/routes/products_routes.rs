use actix_web::web;
use crate::handler::products_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(products_handler::get_all_products))
            .route("/{id}", web::get().to(products_handler::get_product_by_id))
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string(), "StoreManager".to_string(), "InventoryManager".to_string()],
                    })
                    .route("", web::post().to(products_handler::create_product))
                    .route("/{id}", web::put().to(products_handler::update_product))
                    .route("/{id}", web::delete().to(products_handler::delete_product)),
            ),
    );
}
