use crate::repository::order_items_repository::OrderItemRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::order_items::{CreateOrderItem, UpdateOrderItem};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::order_item_guard::OrderItemAccessGuard;
use crate::entities::orders;

pub async fn get_all_order_items(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    let db_ref = db.get_ref();
    let result = if claims.0.role == "Admin" {
        OrderItemRepository::get_all(db_ref).await
    } else if claims.0.role == "StoreManager" {
        OrderItemRepository::get_all_by_store(db_ref, claims.0.store_id).await
    } else {
        OrderItemRepository::get_all_by_employee(db_ref, claims.0.sub).await
    };

    match result {
        Ok(order_items) => HttpResponse::Ok().json(ApiResponse::new(order_items)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch order items".to_string())),
    }
}

pub async fn create_order_item(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, new_order_item: web::Json<CreateOrderItem>) -> impl Responder {
    let db_ref = db.get_ref();
    let order_item_data = new_order_item.into_inner();
    match orders::Entity::find_by_id(order_item_data.order_id).one(db_ref).await {
        Ok(Some(order)) => {
            if (claims.0.role == "StoreManager" && order.store_id == claims.0.store_id) ||
               (claims.0.role == "Cashier" && order.employee_id == claims.0.sub) {
                match OrderItemRepository::create(db_ref, order_item_data).await {
                    Ok(order_item) => HttpResponse::Ok().json(ApiResponse::new(order_item)),
                    Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create order item".to_string())),
                }
            } else {
                HttpResponse::Forbidden().json(ApiError::new("Forbidden: You can only create order items for orders in your store or created by you.".to_string()))
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Order not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Error fetching order".to_string())),
    }
}

pub async fn get_order_item_by_id(guard: OrderItemAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.order_item))
}

pub async fn update_order_item(guard: OrderItemAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdateOrderItem>) -> impl Responder {
    let item_id = guard.order_item.id;
    match OrderItemRepository::update(db.get_ref(), item_id, update_data.into_inner()).await {
        Ok(Some(item)) => HttpResponse::Ok().json(ApiResponse::new(item)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Order item not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update order item".to_string())),
    }
}

pub async fn delete_order_item(guard: OrderItemAccessGuard, db: web::Data<DatabaseConnection>) -> impl Responder {
    let item_id = guard.order_item.id;
    let claims = guard.claims;

    if claims.role == "Cashier" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Cashiers cannot delete order items.".to_string()));
    }

    match OrderItemRepository::delete(db.get_ref(), item_id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Order item deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Order item not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete order item".to_string())),
    }
}
