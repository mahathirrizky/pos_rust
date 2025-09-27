use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect, ColumnTrait, QueryFilter};
use crate::entities::{orders, customers, employees, stores, order_items, products};
use crate::helper::response::{ApiResponse, ApiError};
use crate::auth::auth_service::Claims;
use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;
use std::str::FromStr;
use sea_orm::sea_query::Expr;

#[derive(Serialize)]
pub struct BillItemResponse {
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub discount_amount: f64,
    pub total: f64,
}

#[derive(Serialize)]
pub struct BillResponse {
    pub id: i32,
    pub customer_name: String,
    pub employee_name: String,
    pub store_name: String,
    pub total_amount: f64,
    pub status: String,
    pub order_date: DateTimeUtc,
    pub items: Vec<BillItemResponse>,
}

pub async fn get_all_bills(db: web::Data<DatabaseConnection>, claims: web::ReqData<Claims>) -> impl Responder {
    // Only Admin and Owner can access this endpoint
    if claims.role != "Admin" && claims.role != "Owner" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }

    let orders_result = orders::Entity::find()
        .left_join(customers::Entity)
        .left_join(employees::Entity)
        .left_join(stores::Entity)
        .select_only()
        .column(orders::Column::Id)
        .column(orders::Column::TotalAmount)
        .column(orders::Column::Status)
        .column(orders::Column::OrderDate)
        .column_as(
            Expr::cust_with_exprs(
                "CONCAT(?, ' ', ?)",
                [
                    Expr::col((customers::Entity, customers::Column::FirstName)).into(),
                    Expr::col((customers::Entity, customers::Column::LastName)).into(),
                ],
            ),
            "customer_name",
        )
        .column_as(
            Expr::cust_with_exprs(
                "CONCAT(?, ' ', ?)",
                [
                    Expr::col((employees::Entity, employees::Column::FirstName)).into(),
                    Expr::col((employees::Entity, employees::Column::LastName)).into(),
                ],
            ),
            "employee_name",
        )
        .column_as(stores::Column::Name, "store_name")
        .into_json()
        .all(db.get_ref())
        .await;

    match orders_result {
        Ok(orders_json) => {
            let mut bills_response: Vec<BillResponse> = Vec::new();

            for order_json in orders_json {
                let order_id = order_json["id"].as_i64().unwrap() as i32;

                let items_result = order_items::Entity::find()
                    .filter(order_items::Column::OrderId.eq(order_id))
                    .left_join(products::Entity)
                    .select_only()
                    .column_as(products::Column::Name, "product_name")
                    .column(order_items::Column::Quantity)
                    .column(order_items::Column::UnitPrice)
                    .column(order_items::Column::DiscountAmount)
                    .into_json()
                    .all(db.get_ref())
                    .await;

                let items_response = match items_result {
                    Ok(items_json) => {
                        items_json.into_iter().map(|item_json| {
                            let quantity = item_json["quantity"].as_i64().unwrap() as i32;
                            let unit_price = item_json["unit_price"].as_f64().unwrap();
                            let discount_amount = item_json["discount_amount"].as_f64().unwrap_or(0.0);

                            BillItemResponse {
                                product_name: item_json["product_name"].as_str().unwrap().to_string(),
                                quantity,
                                unit_price,
                                discount_amount,
                                total: (quantity as f64 * unit_price) - discount_amount,
                            }
                        }).collect()
                    },
                    Err(_) => Vec::new(), // Handle error or return empty items
                };

                bills_response.push(BillResponse {
                    id: order_json["id"].as_i64().unwrap() as i32,
                    customer_name: order_json["customer_name"].as_str().unwrap_or("N/A").to_string(),
                    employee_name: order_json["employee_name"].as_str().unwrap_or("N/A").to_string(),
                    store_name: order_json["store_name"].as_str().unwrap_or("N/A").to_string(),
                    total_amount: order_json["total_amount"].as_f64().unwrap(),
                    status: order_json["status"].as_str().unwrap().to_string(),
                    order_date: DateTimeUtc::from_str(order_json["order_date"].as_str().unwrap()).unwrap(),
                    items: items_response,
                });
            }
            HttpResponse::Ok().json(ApiResponse::new(bills_response))
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch bills".to_string())),
    }
}

