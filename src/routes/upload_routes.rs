use actix_web::web;
use crate::handler::upload_handler;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/upload/product") 
            .route(web::post().to(upload_handler::upload_product_image))
            .route(web::delete().to(upload_handler::delete_file)) 
    );
    cfg.service(
        web::resource("/upload/employee") 
            .route(web::post().to(upload_handler::upload_employee_photo))
            .route(web::delete().to(upload_handler::delete_file)) 
    );
}
