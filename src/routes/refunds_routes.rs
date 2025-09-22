use actix_web::web;

use crate::handler::refunds_handler;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/refunds")
            .route(web::post().to(refunds_handler::create_refund))
    );
}
