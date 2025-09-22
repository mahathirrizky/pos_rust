use crate::repository::order_items_repository::OrderItemRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
// use crate::entities::order_items::{CreateOrderItem, UpdateOrderItem};
use crate::entities::order_items::UpdateOrderItem;
use sea_orm::{DatabaseConnection};
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::order_item_guard::OrderItemAccessGuard;

pub async fn get_all_order_items(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    if claims.0.role == "Admin" {
        match OrderItemRepository::get_all(db.get_ref()).await {
            Ok(order_items) => HttpResponse::Ok().json(ApiResponse::new(order_items)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch order items".to_string())),
        }
    } else if claims.0.role == "StoreManager" {
        match OrderItemRepository::get_all_by_store(db.get_ref(), claims.0.store_id).await {
            Ok(order_items) => HttpResponse::Ok().json(ApiResponse::new(order_items)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch order items".to_string())),
        }
    } else {
        match OrderItemRepository::get_all_by_employee(db.get_ref(), claims.0.sub).await {
            Ok(order_items) => HttpResponse::Ok().json(ApiResponse::new(order_items)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch order items".to_string())),
        }
    }
}

// pub async fn create_order_item(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, new_order_item: web::Json<CreateOrderItem>) -> impl Responder {
//     let mut order_item_data = new_order_item.into_inner();
//     // Ensure order item is created for the employee's store if not admin
//     if claims.0.role != "Admin" {
//         // This logic needs to be handled when creating the order itself
//     }

//     match OrderItemRepository::create(db.get_ref(), order_item_data).await {
//         Ok(order_item) => HttpResponse::Ok().json(ApiResponse::new(order_item)),
//         Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create order item".to_string())),
//     }
// }

pub async fn get_order_item_by_id(guard: OrderItemAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.order_item))
}

pub async fn update_order_item(guard: OrderItemAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdateOrderItem>) -> impl Responder {
    let order_item_id = guard.order_item.id;
    match OrderItemRepository::update(db.get_ref(), order_item_id, update_data.into_inner()).await {
        Ok(Some(order_item)) => HttpResponse::Ok().json(ApiResponse::new(order_item)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Order item not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update order item".to_string())),
    }
}

pub async fn delete_order_item(guard: OrderItemAccessGuard, db: web::Data<DatabaseConnection>) -> impl Responder {
    let order_item_id = guard.order_item.id;
    let claims = guard.claims;

    if claims.role == "Cashier" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Cashiers cannot delete order items.".to_string()));
    }

    match OrderItemRepository::delete(db.get_ref(), order_item_id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Order item deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Order item not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete order item".to_string())),
    }
}

