use actix_web::web;
use crate::handler::inventory_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/inventory")
            .wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Admin".to_string(), "StoreManager".to_string(), "InventoryManager".to_string()],
            })
            .route("", web::get().to(inventory_handler::get_all_inventory))
            .route("", web::post().to(inventory_handler::create_inventory))
            .route("/{id}", web::get().to(inventory_handler::get_inventory_by_id))
            .route("/{id}", web::put().to(inventory_handler::update_inventory))
            .route("/{id}", web::delete().to(inventory_handler::delete_inventory))
            .route("/report", web::get().to(inventory_handler::get_inventory_report)),
    );
}
