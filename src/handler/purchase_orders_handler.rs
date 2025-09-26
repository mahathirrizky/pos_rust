use actix_web::{web, HttpResponse, Responder};
use crate::repository::purchase_orders_repository::{PurchaseOrderRepository};
use crate::helper::response::{ApiResponse, ApiError};
use sea_orm::{DatabaseConnection, prelude::Decimal};
use serde::{Deserialize, Serialize};
use crate::entities::{purchase_orders, purchase_order_items, suppliers, stores, products};

#[derive(Deserialize)]
pub struct CreatePurchaseOrderPayload {
    pub supplier_id: i32,
    pub store_id: i32,
    // employee_id will be extracted from claims later
}

// DTO for the list response
#[derive(Serialize)]
pub struct PurchaseOrderListResponse {
    #[serde(flatten)]
    purchase_order: purchase_orders::Model,
    supplier: Option<suppliers::Model>,
    store: Option<stores::Model>,
}

// DTO for the detailed purchase order response
#[derive(Serialize)]
pub struct PurchaseOrderDetailsResponse {
    #[serde(flatten)]
    purchase_order: purchase_orders::Model,
    supplier: Option<suppliers::Model>,
    store: Option<stores::Model>,
    items: Vec<PurchaseOrderItemDetails>,
}

// DTO for the items within the detailed response
#[derive(Serialize)]
pub struct PurchaseOrderItemDetails {
    #[serde(flatten)]
    item: purchase_order_items::Model,
    product: Option<products::Model>,
}

#[derive(Deserialize)]
pub struct UpdateStatusPayload {
    pub status: String,
}

#[derive(Clone, Deserialize)]
pub struct ReceiveItem {
    pub purchase_order_item_id: i32,
    pub quantity_received: i32,
}

#[derive(Deserialize)]
pub struct ReceiveStockPayload {
    pub items: Vec<ReceiveItem>,
}

pub async fn get_all_purchase_orders(db: web::Data<DatabaseConnection>) -> impl Responder {
    match PurchaseOrderRepository::find_all_pos_with_relations(db.get_ref()).await {
        Ok(pos) => {
            let response = pos
                .into_iter()
                .map(|(purchase_order, supplier, store)| PurchaseOrderListResponse {
                    purchase_order,
                    supplier,
                    store,
                })
                .collect::<Vec<_>>();
            HttpResponse::Ok().json(ApiResponse::new(response))
        }
        Err(e) => {
            log::error!("Failed to get all purchase orders: {:?}", e);
            HttpResponse::InternalServerError().json(ApiError::new(e.to_string()))
        }
    }
}

pub async fn create_purchase_order(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<CreatePurchaseOrderPayload>,
    // claims: web::ReqData<YourClaimsStruct> // TODO: Add claims extraction
) -> impl Responder {
    // let employee_id = claims.employee_id;
    let employee_id = None; // Placeholder

    match PurchaseOrderRepository::create_purchase_order(db.get_ref(), payload.supplier_id, payload.store_id, employee_id).await {
        Ok(po) => HttpResponse::Ok().json(ApiResponse::new(po)),
        Err(e) => {
            log::error!("Failed to create purchase order: {:?}", e);
            HttpResponse::InternalServerError().json(ApiError::new(e.to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct AddItemPayload {
    pub product_id: i32,
    pub quantity_ordered: i32,
    pub unit_price: Decimal,
}

pub async fn add_item_to_purchase_order(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    payload: web::Json<AddItemPayload>,
) -> impl Responder {
    let purchase_order_id = path.into_inner();

    match PurchaseOrderRepository::add_item_to_purchase_order(
        db.get_ref(),
        purchase_order_id,
        payload.product_id,
        payload.quantity_ordered,
        payload.unit_price,
    ).await {
        Ok(item) => HttpResponse::Ok().json(ApiResponse::new(item)),
        Err(e) => {
            log::error!("Failed to add item to purchase order: {:?}", e);
            HttpResponse::InternalServerError().json(ApiError::new(e.to_string()))
        }
    }
}

pub async fn get_purchase_order(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let db_ref = db.get_ref();

    match PurchaseOrderRepository::find_po_with_relations(db_ref, id).await {
        Ok(Some((po, supplier, store))) => {
            match PurchaseOrderRepository::find_items_for_po(db_ref, po.id).await {
                Ok(items_with_products) => {
                    let response_items = items_with_products
                        .into_iter()
                        .map(|(item, product)| PurchaseOrderItemDetails { item, product })
                        .collect();

                    let response = PurchaseOrderDetailsResponse {
                        purchase_order: po,
                        supplier,
                        store,
                        items: response_items,
                    };
                    HttpResponse::Ok().json(ApiResponse::new(response))
                }
                Err(e) => {
                    log::error!("Failed to get purchase order items: {:?}", e);
                    HttpResponse::InternalServerError().json(ApiError::new(e.to_string()))
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiError::new(format!("Purchase order with id {} not found", id))),
        Err(e) => {
            log::error!("Failed to get purchase order: {:?}", e);
            HttpResponse::InternalServerError().json(ApiError::new(e.to_string()))
        }
    }
}

pub async fn update_purchase_order_status(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    payload: web::Json<UpdateStatusPayload>,
) -> impl Responder {
    let id = path.into_inner();
    match PurchaseOrderRepository::update_po_status(db.get_ref(), id, payload.status.clone()).await {
        Ok(po) => HttpResponse::Ok().json(ApiResponse::new(po)),
        Err(e) => {
            log::error!("Failed to update purchase order status: {:?}", e);
            HttpResponse::InternalServerError().json(ApiError::new(e.to_string()))
        }
    }
}

pub async fn receive_stock(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    payload: web::Json<ReceiveStockPayload>,
) -> impl Responder {
    let po_id = path.into_inner();
    let items = payload.into_inner().items;

    match PurchaseOrderRepository::receive_purchase_order_items(db.get_ref(), po_id, items).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::new("Stock received successfully".to_string())),
        Err(e) => {
            log::error!("Failed to receive stock: {:?}", e);
            HttpResponse::InternalServerError().json(ApiError::new(e.to_string()))
        }
    }
}
