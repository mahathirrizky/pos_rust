use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, TransactionTrait, Set, prelude::Decimal, ActiveModelTrait};
use std::collections::HashMap;
use serde::Serialize;
use chrono::Utc;

use crate::entities::{refunds, refund_items, orders, payments};
use crate::repository::{
    orders_repository::OrderRepository,
    order_items_repository::OrderItemRepository,
    inventory_repository::InventoryRepository,
    payments_repository::PaymentRepository,
    refunds_repository,
};
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::helper::response::{ApiResponse, ApiError};

#[derive(Serialize)]
pub struct FullRefund {
    refund: refunds::Model,
    items: Vec<refund_items::Model>,
}

pub async fn create_refund(
    db: web::Data<DatabaseConnection>,
    claims: ClaimsExtractor,
    payload: web::Json<refunds::CreateRefund>,
) -> impl Responder {
    if claims.0.role != "StoreManager" && claims.0.role != "Admin" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }

    let txn = match db.begin().await {
        Ok(txn) => txn,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to start transaction: {}", e))),
    };

    // 1. Fetch original order and its items
    let order = match OrderRepository::find_by_id(&txn, payload.order_id).await {
        Ok(Some(order)) => order,
        Ok(None) => return HttpResponse::NotFound().json(ApiError::new("Order not found".to_string())),
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch order: {}", e))),
    };

    let original_items = match OrderItemRepository::get_all_by_order_id(&txn, order.id).await {
        Ok(items) => items,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch order items: {}", e))),
    };

    let original_items_map: HashMap<i32, &crate::entities::order_items::Model> = original_items.iter().map(|item| (item.id, item)).collect();
    let mut total_refund_amount = Decimal::new(0, 2);
    let mut refund_item_models = vec![];
    let mut total_original_quantity = 0;
    let mut total_refund_quantity = 0;

    // 2. Validate payload and calculate amounts
    for item_to_refund in &payload.items {
        total_refund_quantity += item_to_refund.quantity;
        if let Some(original_item) = original_items_map.get(&item_to_refund.order_item_id) {
            if item_to_refund.quantity > original_item.quantity {
                return HttpResponse::BadRequest().json(ApiError::new(format!("Cannot refund more items than were purchased for item ID {}", original_item.id)));
            }

            let item_amount = original_item.unit_price * Decimal::from(item_to_refund.quantity);
            total_refund_amount += item_amount;

            refund_item_models.push(refund_items::ActiveModel {
                order_item_id: Set(item_to_refund.order_item_id),
                product_id: Set(original_item.product_id),
                quantity: Set(item_to_refund.quantity),
                amount: Set(item_amount),
                ..Default::default()
            });
        } else {
            return HttpResponse::BadRequest().json(ApiError::new(format!("Item with ID {} not found in original order", item_to_refund.order_item_id)));
        }
    }

    for item in &original_items {
        total_original_quantity += item.quantity;
    }

    // 3. Create the main refund record
    let refund_model = refunds::ActiveModel {
        order_id: Set(payload.order_id),
        employee_id: Set(claims.0.sub),
        store_id: Set(claims.0.store_id),
        reason: Set(payload.reason.clone()),
        total_amount: Set(total_refund_amount),
        ..Default::default()
    };

    let (refund, refund_items) = match refunds_repository::create_refund(&txn, refund_model, refund_item_models).await {
        Ok(data) => data,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to create refund records: {}", e))),
    };

    // 4. Increase inventory for each refunded item
    for item in &refund_items {
        if let Err(e) = InventoryRepository::increase_quantity(&txn, item.product_id, claims.0.store_id, item.quantity).await {
            return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to update inventory for product {}: {}", item.product_id, e)));
        }
    }

    // 5. Update original order status
    let new_status = if total_refund_quantity >= total_original_quantity { "Refunded" } else { "Partially Refunded" };
    let mut order_active_model: orders::ActiveModel = order.into();
    order_active_model.status = Set(new_status.to_string());
    if let Err(e) = order_active_model.update(&txn).await {
        return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to update order status: {}", e)));
    }

    // 6. Create a negative payment record for the refund
    let refund_payment = payments::CreatePayment {
        order_id: payload.order_id,
        payment_method: "REFUND".to_string(),
        amount: -total_refund_amount, // Negative amount
        payment_date: Utc::now(),
        status: "Completed".to_string(),
    };
    if let Err(e) = PaymentRepository::create(&txn, refund_payment).await {
        return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to create refund payment record: {}", e)));
    }

    // 7. Commit transaction
    if let Err(e) = txn.commit().await {
        return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to commit transaction: {}", e)));
    }

    let full_refund = FullRefund {
        refund,
        items: refund_items,
    };

    HttpResponse::Ok().json(ApiResponse::new(full_refund))
}
