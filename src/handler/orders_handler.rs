use crate::repository::orders_repository::OrderRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::orders::{CreateOrder, UpdateOrder};
use sea_orm::{DatabaseConnection};
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::order_guard::OrderAccessGuard;

pub async fn get_all_orders(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    if claims.0.role == "Admin" {
        match OrderRepository::get_all(db.get_ref()).await {
            Ok(orders) => HttpResponse::Ok().json(ApiResponse::new(orders)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch orders".to_string())),
        }
    } else if claims.0.role == "StoreManager" {
        // Store Manager can view orders in their store
        match OrderRepository::get_all_by_store(db.get_ref(), claims.0.store_id).await { 
            Ok(orders) => HttpResponse::Ok().json(ApiResponse::new(orders)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch orders".to_string())),
        }
    } else {
        // Cashier can view orders they created
        match OrderRepository::get_all_by_employee(db.get_ref(), claims.0.sub).await { 
            Ok(orders) => HttpResponse::Ok().json(ApiResponse::new(orders)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch orders".to_string())),
        }
    }
}

pub async fn create_order(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, new_order: web::Json<CreateOrder>) -> impl Responder {
    let mut order_data = new_order.into_inner();
    // Ensure order is created for the employee's store if not admin
    if claims.0.role != "Admin" {
        order_data.store_id = claims.0.store_id;
        // Ensure cashier can only create order for themselves
        if claims.0.role == "Cashier" {
            order_data.employee_id = claims.0.sub;
        }
    }

    match OrderRepository::create(db.get_ref(), order_data).await {
        Ok(order) => HttpResponse::Ok().json(ApiResponse::new(order)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create order".to_string())),
    }
}

pub async fn get_order_by_id(guard: OrderAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.order))
}

pub async fn update_order(guard: OrderAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdateOrder>) -> impl Responder {
    let order_id = guard.order.id;
    match OrderRepository::update(db.get_ref(), order_id, update_data.into_inner()).await {
        Ok(Some(order)) => HttpResponse::Ok().json(ApiResponse::new(order)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Order not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update order".to_string())),
    }
}

pub async fn delete_order(guard: OrderAccessGuard, db: web::Data<DatabaseConnection>) -> impl Responder {
    let order_id = guard.order.id;
    let claims = guard.claims;

    // Additional check: Cashiers are not allowed to delete orders.
    if claims.role == "Cashier" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Cashiers cannot delete orders.".to_string()));
    }

    match OrderRepository::delete(db.get_ref(), order_id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Order deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Order not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete order".to_string())),
    }
}
