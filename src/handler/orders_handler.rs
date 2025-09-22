use crate::repository::orders_repository::OrderRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::orders::{CreateOrderPayload, UpdateOrder};
use sea_orm::{DatabaseConnection, TransactionTrait, prelude::Decimal, ActiveValue};
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::order_guard::OrderAccessGuard;
use crate::repository::products_repository::ProductRepository;
use crate::repository::promotions_repository::PromotionRepository;
use crate::repository::inventory_repository::InventoryRepository;
use crate::entities::{order_items, promotions, products};
use std::collections::HashMap;
use chrono::Utc;

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

pub async fn create_order(
    claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
    new_order_payload: web::Json<CreateOrderPayload>,
) -> impl Responder {
    let txn = match db.begin().await {
        Ok(txn) => txn,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to start transaction: {}", e))),
    };

    let employee_id = claims.0.sub;
    let store_id = claims.0.store_id;
    let customer_id = new_order_payload.customer_id;

    let mut total_order_amount = Decimal::new(0, 2);
    let mut order_items_active_models = Vec::new();

    // Fetch all products and promotions needed in one go to minimize DB calls
    let product_ids: Vec<i32> = new_order_payload.items.iter().map(|item| item.product_id).collect();
    let promotion_ids: Vec<i32> = new_order_payload.items.iter().filter_map(|item| item.promotion_id).collect();

    let products = match ProductRepository::find_by_ids(&txn, product_ids).await {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch products: {}", e))),
    };
    let products_map: HashMap<i32, products::Model> = products.into_iter().map(|p| (p.id, p)).collect();

    let promotions = match PromotionRepository::find_by_ids(&txn, promotion_ids).await {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch promotions: {}", e))),
    };
    let promotions_map: HashMap<i32, promotions::Model> = promotions.into_iter().map(|p| (p.id, p)).collect();

    for item_payload in &new_order_payload.items {
        let product = match products_map.get(&item_payload.product_id) {
            Some(p) => p,
            None => return HttpResponse::BadRequest().json(ApiError::new(format!("Product with ID {} not found", item_payload.product_id))),
        };

        // Check inventory
        let inventory_item = match InventoryRepository::find_by_product_and_store(&txn, product.id, store_id).await {
            Ok(Some(inv)) => inv,
            Ok(None) => return HttpResponse::BadRequest().json(ApiError::new(format!("Product {} not available in store {}", product.name, store_id))),
            Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to check inventory: {}", e))),
        };

        if inventory_item.quantity < item_payload.quantity {
            return HttpResponse::BadRequest().json(ApiError::new(format!("Insufficient stock for product {}. Available: {}, Requested: {}", product.name, inventory_item.quantity, item_payload.quantity)));
        }

        let mut unit_price = product.price;
        let mut discount_amount = Decimal::new(0, 2);

        if let Some(promo_id) = item_payload.promotion_id {
            if let Some(promotion) = promotions_map.get(&promo_id) {
                // Apply promotion logic
                if promotion.is_active && promotion.start_date <= Utc::now() && promotion.end_date >= Utc::now() {
                    // Check if promotion is for a specific product and matches
                    if promotion.product_id.is_none() || promotion.product_id == Some(product.id) {
                        match promotion.promotion_type.as_str() {
                            "PERCENTAGE" => {
                                discount_amount = unit_price * (promotion.value / Decimal::new(100, 0));
                                unit_price -= discount_amount;
                            },
                            "FIXED_AMOUNT" => {
                                discount_amount = promotion.value;
                                unit_price -= discount_amount;
                            },
                            _ => { /* Unknown promotion type, ignore */ }
                        }
                    }
                }
            }
        }

        total_order_amount += unit_price * Decimal::from(item_payload.quantity);

        order_items_active_models.push(order_items::ActiveModel {
            product_id: ActiveValue::Set(product.id),
            quantity: ActiveValue::Set(item_payload.quantity),
            unit_price: ActiveValue::Set(unit_price),
            discount_amount: ActiveValue::Set(discount_amount),
            ..Default::default()
        });

        // Deduct from inventory
        if let Err(e) = InventoryRepository::increase_quantity(&txn, product.id, store_id, -item_payload.quantity).await {
            return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to deduct inventory for product {}: {}", product.name, e)));
        }
    }

    // Create the order and its items
    let order = match OrderRepository::create(
        &txn,
        customer_id,
        employee_id,
        store_id,
        total_order_amount,
        "Pending".to_string(), // Initial status
        order_items_active_models,
    ).await {
        Ok(o) => o,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to create order: {}", e))),
    };

    // Commit transaction
    if let Err(e) = txn.commit().await {
        return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to commit transaction: {}", e)));
    }

    HttpResponse::Ok().json(ApiResponse::new(order))
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


