use crate::repository::inventory_repository::InventoryRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::auth::auth_service::Claims;
use crate::entities::inventory::{CreateInventory, UpdateInventory, InventoryReport, InventoryReportQueryParams, InventoryReportItem};
use sea_orm::{DatabaseConnection, QuerySelect, ColumnTrait, Condition, EntityTrait, QueryFilter};
use crate::guard::inventory_guard::InventoryAccessGuard;
use crate::entities::inventory::Column as InventoryColumn;
use crate::entities::inventory::Entity as InventoryEntity;
use crate::entities::{products, stores};
use sea_orm::prelude::DateTimeUtc;
use std::str::FromStr;
use actix::Addr;
use crate::websocket::broadcaster::Broadcaster;

pub async fn get_all_inventory(db: web::Data<DatabaseConnection>, claims: web::ReqData<Claims>) -> impl Responder {
    let result = if claims.role == "Admin" || claims.role == "Owner" {
        InventoryRepository::get_all(db.get_ref()).await
    } else {
        if let Some(store_id) = claims.store_id {
            InventoryRepository::get_all_by_store(db.get_ref(), store_id).await
        } else {
            return HttpResponse::Forbidden().json(ApiError::new("User has no assigned store".to_string()));
        }
    };

    match result {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse::new(inventory)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch inventory".to_string())),
    }
}

pub async fn create_inventory(db: web::Data<DatabaseConnection>, new_inventory: web::Json<CreateInventory>, broadcaster: web::Data<Addr<Broadcaster>>) -> impl Responder {
    let inventory_data = new_inventory.into_inner();

    match InventoryRepository::create(db.get_ref(), inventory_data).await {
        Ok(inventory) => {
            broadcaster.do_send(crate::websocket::broadcaster::BroadcastMessage("inventory_updated".to_string()));
            HttpResponse::Ok().json(ApiResponse::new(inventory))
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create inventory".to_string())),
    }
}

pub async fn get_inventory_by_id(guard: InventoryAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.inventory))
}

pub async fn update_inventory(guard: InventoryAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdateInventory>, broadcaster: web::Data<Addr<Broadcaster>>) -> impl Responder {
    let item_id = guard.inventory.id;
    match InventoryRepository::update(db.get_ref(), item_id, update_data.into_inner(), broadcaster.get_ref()).await {
        Ok(Some(item)) => HttpResponse::Ok().json(ApiResponse::new(item)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Inventory not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update inventory".to_string())),
    }
}

pub async fn delete_inventory(guard: InventoryAccessGuard, db: web::Data<DatabaseConnection>, broadcaster: web::Data<Addr<Broadcaster>>) -> impl Responder {
    let item_id = guard.inventory.id;
    match InventoryRepository::delete(db.get_ref(), item_id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            broadcaster.do_send(crate::websocket::broadcaster::BroadcastMessage("inventory_updated".to_string()));
            HttpResponse::Ok().json(ApiResponse::new("Inventory deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Inventory not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete inventory".to_string())),
    }
}

pub async fn get_inventory_report(
    db: web::Data<DatabaseConnection>,
    query_params: web::Query<InventoryReportQueryParams>,
) -> impl Responder {

    let mut condition = Condition::all();

    if let Some(store_id) = query_params.store_id {
        condition = condition.add(InventoryColumn::StoreId.eq(store_id));
    }
    if let Some(product_id) = query_params.product_id {
        condition = condition.add(InventoryColumn::ProductId.eq(product_id));
    }

    let inventory_data = match InventoryEntity::find()
        .filter(condition)
        .left_join(products::Entity)
        .left_join(stores::Entity)
        .select_only()
        .column(InventoryColumn::Id)
        .column(InventoryColumn::ProductId)
        .column(products::Column::Name)
        .column(InventoryColumn::StoreId)
        .column(stores::Column::Name)
        .column(InventoryColumn::Quantity)
        .column(InventoryColumn::LastRestocked)
        .column(InventoryColumn::UpdatedAt)
        .into_json()
        .all(db.get_ref())
        .await
    {
        Ok(data) => data,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch inventory data: {}", e))),
    };

    let mut report_items = Vec::new();
    for row in &inventory_data {
        report_items.push(InventoryReportItem {
            inventory_id: row["id"].as_i64().unwrap() as i32,
            product_id: row["product_id"].as_i64().unwrap() as i32,
            product_name: row["products"].as_object().unwrap()["name"].as_str().unwrap().to_string(),
            store_id: row["store_id"].as_i64().unwrap() as i32,
            store_name: row["stores"].as_object().unwrap()["name"].as_str().unwrap().to_string(),
            quantity: row["quantity"].as_i64().unwrap() as i32,
            last_restocked: row["last_restocked"].as_str().map(|s| DateTimeUtc::from_str(s).unwrap()),
            updated_at: DateTimeUtc::from_str(row["updated_at"].as_str().unwrap()).unwrap(),
        });
    }

    let inventory_report = InventoryReport {
        items: report_items,
        total_items: inventory_data.len() as u64,
    };

    HttpResponse::Ok().json(ApiResponse::new(inventory_report))
}
