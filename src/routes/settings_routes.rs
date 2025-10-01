use actix_web::web;

use crate::handler::settings_handler::{get_settings, save_settings};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(get_settings)
            .service(save_settings)
    );
}