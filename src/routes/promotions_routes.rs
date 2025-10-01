use actix_web::web;
use crate::handler::promotions_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/promotions")
            .route(
                "",
                web::get()
                    .to(promotions_handler::get_all_promotions)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["promotions:read".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(promotions_handler::get_promotion_by_id)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["promotions:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(promotions_handler::create_promotion)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["promotions:create".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(promotions_handler::update_promotion)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["promotions:update".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(promotions_handler::delete_promotion)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["promotions:delete".to_string()],
                    }),
            ),
    );
}
