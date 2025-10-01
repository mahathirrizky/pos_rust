use actix_web::web;
use crate::handler::roles_handler;
use crate::middleware::permission::PermissionMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .route(
                "",
                web::get().to(roles_handler::get_all_roles).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["roles:read".to_string()],
                }),
            )
            .route(
                "/{id}",
                web::get().to(roles_handler::get_role_by_id).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["roles:read".to_string()],
                }),
            )
            .route(
                "",
                web::post().to(roles_handler::create_role).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["roles:create".to_string()],
                }),
            )
            .route(
                "/{id}",
                web::put().to(roles_handler::update_role).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["roles:update".to_string()],
                }),
            )
            .route(
                "/{id}",
                web::delete().to(roles_handler::delete_role).wrap(PermissionMiddlewareFactory {
                    required_permissions: vec!["roles:delete".to_string()],
                }),
            )
            .route(
                "/{id}/permissions",
                web::get()
                    .to(roles_handler::get_role_permissions)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["permissions:read".to_string()],
                    }),
            )
            .route(
                "/assign-permission",
                web::post()
                    .to(roles_handler::assign_permission_to_role)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["permissions:assign".to_string()],
                    }),
            )
            .route(
                "/remove-permission",
                web::post()
                    .to(roles_handler::remove_permission_from_role)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["permissions:assign".to_string()],
                    }),
            ),
    );
    cfg.service(
        web::scope("/permissions")
            .route(
                "",
                web::get()
                    .to(roles_handler::get_all_permissions)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["permissions:read".to_string()],
                    }),
            )
            .route(
                "",
                web::post()
                    .to(roles_handler::create_permission)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["permissions:assign".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(roles_handler::update_permission)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["permissions:assign".to_string()],
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(roles_handler::delete_permission)
                    .wrap(PermissionMiddlewareFactory {
                        required_permissions: vec!["permissions:assign".to_string()],
                    }),
            ),
    );
}
