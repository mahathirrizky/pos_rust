use actix_web::web;
use crate::handler::suppliers_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/suppliers")
            .route("", web::get().to(suppliers_handler::get_all_suppliers))
            .route("/{id}", web::get().to(suppliers_handler::get_supplier_by_id))
            .service(
                web::scope("")
                    .wrap(RoleMiddlewareFactory {
                        allowed_roles: vec!["Owner".to_string(), "Admin".to_string()],
                    })
                    .route("", web::post().to(suppliers_handler::create_supplier))
                    .route("/{id}", web::put().to(suppliers_handler::update_supplier))
                    .route("/{id}", web::delete().to(suppliers_handler::delete_supplier)),
            ),
    );
}
