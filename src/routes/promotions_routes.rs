use actix_web::web;

use crate::handler::promotions_handler;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/promotions")
            .route("", web::post().to(promotions_handler::create_promotion))
            .route("", web::get().to(promotions_handler::get_all_promotions))
            .route("/{id}", web::get().to(promotions_handler::get_promotion_by_id))
            .route("/{id}", web::put().to(promotions_handler::update_promotion))
            .route("/{id}", web::delete().to(promotions_handler::delete_promotion)),
    );
}
