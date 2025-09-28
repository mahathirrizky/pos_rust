use actix_web::web;
use crate::handler::roles_handler;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .route("", web::get().to(roles_handler::get_all_roles))
            .route("/{id}", web::get().to(roles_handler::get_role_by_id))
            .route("", web::post().to(roles_handler::create_role))
            .route("/{id}", web::put().to(roles_handler::update_role))
            .route("/{id}", web::delete().to(roles_handler::delete_role))
            .route("/{id}/permissions", web::get().to(roles_handler::get_role_permissions))
            .route("/assign-permission", web::post().to(roles_handler::assign_permission_to_role))
            .route("/remove-permission", web::post().to(roles_handler::remove_permission_from_role))
    );
    cfg.service(
        web::scope("/permissions")
            .route("", web::get().to(roles_handler::get_all_permissions))
            .route("", web::post().to(roles_handler::create_permission))
            .route("/{id}", web::put().to(roles_handler::update_permission))
            .route("/{id}", web::delete().to(roles_handler::delete_permission))
    );
}
