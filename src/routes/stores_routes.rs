use actix_web::web;
use crate::handler::stores_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stores")
            // Route for getting all stores (Owner, Admin)
            .route("", web::get().to(stores_handler::get_all_stores).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string(), "Admin".to_string()],
            }))
            // Route for creating a store (Owner only)
            .route("", web::post().to(stores_handler::create_store).wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Owner".to_string()],
            }))
            // Routes for specific ID
            .service(
                web::scope("/{id}")
                    // Route for getting a single store (Owner, Admin)
                    .route("", web::get().to(stores_handler::get_store_by_id).wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Owner".to_string(), "Admin".to_string()],
                    }))
                    // Route for updating a store (Owner only)
                    .route("", web::put().to(stores_handler::update_store).wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Owner".to_string()],
                    }))
                    // Route for deleting a store (Owner only)
                    .route("", web::delete().to(stores_handler::delete_store).wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Owner".to_string()],
                    }))
            )
    );
}
