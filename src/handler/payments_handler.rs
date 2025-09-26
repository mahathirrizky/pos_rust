use crate::repository::payments_repository::PaymentRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::payments::{CreatePayment, UpdatePayment};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::payment_guard::PaymentAccessGuard;
use crate::entities::orders;

pub async fn get_all_payments(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    let db_ref = db.get_ref();
    let result = if claims.0.role == "Admin" {
        PaymentRepository::get_all(db_ref).await
    } else if claims.0.role == "StoreManager" {
        if let Some(store_id) = claims.0.store_id {
            PaymentRepository::get_all_by_store(db_ref, store_id).await
        } else {
            // If a store manager has no store_id, return an empty list.
            return HttpResponse::Ok().json(ApiResponse::new(Vec::<crate::entities::payments::Model>::new()));
        }
    } else {
        PaymentRepository::get_all_by_employee(db_ref, claims.0.sub).await
    };

    match result {
        Ok(payments) => HttpResponse::Ok().json(ApiResponse::new(payments)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch payments".to_string())),
    }
}

pub async fn create_payment(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, new_payment: web::Json<CreatePayment>) -> impl Responder {
    let db_ref = db.get_ref();
    let payment_data = new_payment.into_inner();
    match orders::Entity::find_by_id(payment_data.order_id).one(db_ref).await {
        Ok(Some(order)) => {
            if (claims.0.role == "StoreManager" && claims.0.store_id.map_or(false, |id| id == order.store_id)) ||
               (claims.0.role == "Cashier" && order.employee_id == claims.0.sub) {
                match PaymentRepository::create(db_ref, payment_data).await {
                    Ok(payment) => HttpResponse::Ok().json(ApiResponse::new(payment)),
                    Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create payment".to_string())),
                }
            } else {
                HttpResponse::Forbidden().json(ApiError::new("Forbidden: You can only create payments for orders in your store or created by you.".to_string()))
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Order not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Error fetching order".to_string())),
    }
}

pub async fn get_payment_by_id(guard: PaymentAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.payment))
}

pub async fn update_payment(guard: PaymentAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdatePayment>) -> impl Responder {
    let payment_id = guard.payment.id;
    match PaymentRepository::update(db.get_ref(), payment_id, update_data.into_inner()).await {
        Ok(Some(payment)) => HttpResponse::Ok().json(ApiResponse::new(payment)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Payment not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update payment".to_string())),
    }
}

pub async fn delete_payment(guard: PaymentAccessGuard, db: web::Data<DatabaseConnection>) -> impl Responder {
    let payment_id = guard.payment.id;
    let claims = guard.claims;

    if claims.role == "Cashier" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Cashiers cannot delete payments.".to_string()));
    }

    match PaymentRepository::delete(db.get_ref(), payment_id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Payment deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Payment not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete payment".to_string())),
    }
}
