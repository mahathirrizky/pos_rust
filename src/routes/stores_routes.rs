use actix_web::web;
use crate::handler::stores_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stores")
            .route("", web::get().to(stores_handler::get_all_stores))
            .route("/{id}", web::get().to(stores_handler::get_store_by_id))
            .route("/{id}", web::put().to(stores_handler::update_store))
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string()],
                    })
                    .route("", web::post().to(stores_handler::create_store))
                    .route("/{id}", web::delete().to(stores_handler::delete_store)),
            ),
    );
}
