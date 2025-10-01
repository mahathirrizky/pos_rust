use actix_web::web;
use crate::handler::suppliers_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/suppliers")
            .route(
                "",
                web::get()
                    .to(suppliers_handler::get_all_suppliers)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["suppliers:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(suppliers_handler::get_supplier_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["suppliers:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(suppliers_handler::create_supplier)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["suppliers:create".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(suppliers_handler::update_supplier)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["suppliers:update".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(suppliers_handler::delete_supplier)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["suppliers:delete".to_string()],
                    }),
            ),
    );
}
