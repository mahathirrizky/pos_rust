use actix_web::web;

pub mod categories_routes;
pub mod customers_routes;
pub mod employees_routes;
pub mod inventory_routes;
pub mod order_items_routes;
pub mod orders_routes;
pub mod payments_routes;
pub mod products_routes;
pub mod promotions_routes;
pub mod refunds_routes;
pub mod stores_routes;
pub mod suppliers_routes;
pub mod reports_routes;
pub mod purchase_orders_routes;
pub mod bills_routes; // New module
pub mod roles_routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(categories_routes::configure_routes)
       .configure(customers_routes::configure_routes)
       .configure(employees_routes::configure_routes)
       .configure(inventory_routes::configure_routes)
       .configure(order_items_routes::configure_routes)
       .configure(orders_routes::configure_routes)
       .configure(payments_routes::configure_routes)
       .configure(products_routes::configure_routes)
       .configure(promotions_routes::configure_routes)
       .configure(refunds_routes::configure_routes)
       .configure(stores_routes::configure_routes)
       .configure(suppliers_routes::configure_routes)
       .configure(reports_routes::configure_routes)
       .configure(purchase_orders_routes::configure_routes)
       .configure(bills_routes::configure_routes)
       .configure(roles_routes::configure_routes); // Configure new routes
}