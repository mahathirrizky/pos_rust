use actix_web::web;
use crate::handler::categories_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::get().to(categories_handler::get_all_categories))
            .route("/{id}", web::get().to(categories_handler::get_category_by_id))
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Admin".to_string()],
                    })
                    .route("", web::post().to(categories_handler::create_category))
                    .route("/{id}", web::put().to(categories_handler::update_category))
                    .route("/{id}", web::delete().to(categories_handler::delete_category)),
            ),
    );
}
