use crate::repository::inventory_repository::InventoryRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::inventory::{CreateInventory, UpdateInventory, InventoryReport, InventoryReportItem, InventoryReportQueryParams};
use sea_orm::{DatabaseConnection, QuerySelect, ColumnTrait, Condition, EntityTrait, QueryFilter};
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::inventory_guard::InventoryAccessGuard;
use crate::entities::inventory::Column as InventoryColumn;
use crate::entities::inventory::Entity as InventoryEntity;
use crate::entities::{products, stores};
use sea_orm::prelude::DateTimeUtc;
use std::str::FromStr;

pub async fn get_all_inventory(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    if claims.0.role == "Admin" {
        match InventoryRepository::get_all(db.get_ref()).await {
            Ok(inventory) => HttpResponse::Ok().json(ApiResponse::new(inventory)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch inventory".to_string())),
        }
    } else {
        // Store Manager and Inventory Manager can view inventory in their store
        match InventoryRepository::get_all_by_store(db.get_ref(), claims.0.store_id).await {
            Ok(inventory) => HttpResponse::Ok().json(ApiResponse::new(inventory)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch inventory".to_string())),
        }
    }
}

pub async fn create_inventory(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>, new_inventory: web::Json<CreateInventory>) -> impl Responder {
    let mut inventory_data = new_inventory.into_inner();
    // Ensure inventory is created for the user's store if not admin
    if claims.0.role != "Admin" {
        inventory_data.store_id = claims.0.store_id;
    }

    match InventoryRepository::create(db.get_ref(), inventory_data).await {
        Ok(inventory) => HttpResponse::Ok().json(ApiResponse::new(inventory)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to create inventory".to_string())),
    }
}

pub async fn get_inventory_by_id(guard: InventoryAccessGuard) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(guard.inventory))
}

pub async fn update_inventory(guard: InventoryAccessGuard, db: web::Data<DatabaseConnection>, update_data: web::Json<UpdateInventory>) -> impl Responder {
    let item_id = guard.inventory.id;
    match InventoryRepository::update(db.get_ref(), item_id, update_data.into_inner()).await {
        Ok(Some(item)) => HttpResponse::Ok().json(ApiResponse::new(item)),
        Ok(None) => HttpResponse::NotFound().json(ApiError::new("Inventory not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to update inventory".to_string())),
    }
}

pub async fn delete_inventory(guard: InventoryAccessGuard, db: web::Data<DatabaseConnection>) -> impl Responder {
    let item_id = guard.inventory.id;
    match InventoryRepository::delete(db.get_ref(), item_id).await {
        Ok(rows_affected) if rows_affected > 0 => {
            HttpResponse::Ok().json(ApiResponse::new("Inventory deleted successfully".to_string()))
        }
        Ok(_) => HttpResponse::NotFound().json(ApiError::new("Inventory not found".to_string())),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to delete inventory".to_string())),
    }
}

pub async fn get_inventory_report(
    claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
    query_params: web::Query<InventoryReportQueryParams>,
) -> impl Responder {
    if claims.0.role != "Admin" && claims.0.role != "StoreManager" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }

    let mut condition = Condition::all();

    if let Some(store_id) = query_params.store_id {
        condition = condition.add(InventoryColumn::StoreId.eq(store_id));
    }
    // If StoreManager, filter by their store_id
    if claims.0.role == "StoreManager" {
        condition = condition.add(InventoryColumn::StoreId.eq(claims.0.store_id));
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
