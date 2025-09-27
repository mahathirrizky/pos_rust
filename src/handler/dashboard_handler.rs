use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::helper::response::ApiResponse;

pub async fn get_summary() -> impl Responder {
    let summary_data = json!({
        "total_sales": 50250.75,
        "total_orders": 350,
        "new_customers": 25,
        "pending_purchase_orders": 5
    });
    HttpResponse::Ok().json(ApiResponse::new(summary_data))
}

pub async fn get_activities() -> impl Responder {
    let activities_data = json!([
        {"id": 1, "user": "Admin", "action": "Created new promotion 'Summer Sale'", "timestamp": "2025-09-26T10:00:00Z"},
        {"id": 2, "user": "owner@example.com", "action": "Adjusted stock for 'Product A'", "timestamp": "2025-09-26T09:30:00Z"},
        {"id": 3, "user": "cashier1@example.com", "action": "Completed order #12345", "timestamp": "2025-09-26T09:25:00Z"}
    ]);
    HttpResponse::Ok().json(ApiResponse::new(activities_data))
}

pub async fn get_sales() -> impl Responder {
    let sales_data = json!({
        "labels": ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
        "datasets": [{
            "label": "Weekly Sales",
            "data": [1200, 1900, 3000, 5000, 2300, 4500, 4800],
            "backgroundColor": "rgba(75, 192, 192, 0.2)",
            "borderColor": "rgba(75, 192, 192, 1)"
        }]
    });
    HttpResponse::Ok().json(ApiResponse::new(sales_data))
}
