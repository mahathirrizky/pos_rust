use actix_web::web;
use crate::handler::dashboard_handler;
use crate::middleware::role::RoleMiddlewareFactory;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/dashboard")
            .wrap(RoleMiddlewareFactory {
                allowed_roles: vec!["Admin".to_string(), "Owner".to_string()],
            })
            .route("/summary", web::get().to(dashboard_handler::get_summary))
            .route("/activities", web::get().to(dashboard_handler::get_activities))
            .route("/sales", web::get().to(dashboard_handler::get_sales)),
    );
}
