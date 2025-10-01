use actix_web::web;
use crate::auth::auth_handler::{login, set_password, forgot_password};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/set-password", web::post().to(set_password))
            .route("/forgot-password", web::post().to(forgot_password))
    );
}
