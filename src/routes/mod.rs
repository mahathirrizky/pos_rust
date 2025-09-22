use actix_web::web;

mod categories_routes;
mod suppliers_routes;
mod stores_routes;
mod customers_routes;
mod employees_routes;
mod products_routes;
mod orders_routes;
mod order_items_routes;
mod inventory_routes;
mod payments_routes;
mod refunds_routes;
mod promotions_routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(categories_routes::configure_routes)
            .configure(suppliers_routes::configure_routes)
            .configure(stores_routes::configure_routes)
            .configure(customers_routes::configure_routes)
            .configure(employees_routes::configure_routes)
            .configure(products_routes::configure_routes)
            .configure(orders_routes::configure_routes)
            .configure(order_items_routes::configure_routes)
            .configure(inventory_routes::configure_routes)
            .configure(payments_routes::configure_routes)
            .configure(refunds_routes::configure_routes)
            .configure(promotions_routes::configure_routes),
    );
}
