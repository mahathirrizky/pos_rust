use actix_web::web;
use crate::handler::products_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            // GET all products
            .route("", web::get().to(products_handler::get_all_products).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string(), "Admin".to_string(), "StoreManager".to_string(), "InventoryManager".to_string()],
            }))
            // POST create product
            .route("", web::post().to(products_handler::create_product).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Admin".to_string(), "InventoryManager".to_string()],
            }))
            // GET product by ID
            .route("/{id}", web::get().to(products_handler::get_product_by_id).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string(), "Admin".to_string(), "StoreManager".to_string(), "InventoryManager".to_string()],
            }))
            // PUT update product
            .route("/{id}", web::put().to(products_handler::update_product).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Admin".to_string(), "InventoryManager".to_string()],
            }))
            // DELETE product
            .route("/{id}", web::delete().to(products_handler::delete_product).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Admin".to_string(), "InventoryManager".to_string()],
            }))
    );
}
