use crate::repository::orders_repository::OrderRepository;
use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::entities::orders::{CreateOrderPayload, UpdateOrder};
use sea_orm::{DatabaseConnection, TransactionTrait, prelude::Decimal, ActiveValue, ActiveModelTrait};
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::guard::order_guard::OrderAccessGuard;
use crate::repository::products_repository::ProductRepository;
use crate::repository::promotions_repository::PromotionRepository;
use crate::repository::inventory_repository::InventoryRepository;
use crate::repository::payments_repository::PaymentRepository;
use crate::entities::{order_items, promotions, products, employees, payments, orders};
use std::collections::HashMap;
use chrono::Utc;
use crate::entities::orders::{SalesReport, ProductSalesReport, EmployeeSalesReport, SalesReportQueryParams};
use crate::entities::orders::Column as OrderColumn;
use crate::entities::orders::Entity as OrderEntity;
use sea_orm::{QuerySelect, ColumnTrait, Condition, self, EntityTrait, QueryFilter};
use std::str::FromStr;

pub async fn get_all_orders(claims: ClaimsExtractor, db: web::Data<DatabaseConnection>) -> impl Responder {
    if claims.0.role == "Admin" {
        match OrderRepository::get_all(db.get_ref()).await {
            Ok(orders) => HttpResponse::Ok().json(ApiResponse::new(orders)),
            Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch orders".to_string())),
        }
    } else if claims.0.role == "StoreManager" {
        if let Some(store_id) = claims.0.store_id {
            match OrderRepository::get_all_by_store(db.get_ref(), store_id).await { 
                Ok(orders) => HttpResponse::Ok().json(ApiResponse::new(orders)),
                Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch orders".to_string())),
            }
        } else {
            HttpResponse::Forbidden().json(ApiError::new("StoreManager has no assigned store".to_string()))
        }
    } else { // Cashier and other roles
        if let Some(store_id) = claims.0.store_id {
            match OrderRepository::get_all_by_employee_and_store(db.get_ref(), claims.0.sub, store_id).await {
                Ok(orders) => HttpResponse::Ok().json(ApiResponse::new(orders)),
                Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch orders for cashier in store".to_string())),
            }
        } else {
            // Fallback for users with roles other than Admin/StoreManager who might not have a store_id
            // Or if a cashier somehow doesn't have a store_id in their token
            HttpResponse::Forbidden().json(ApiError::new("User is not assigned to a store session".to_string()))
        }
    }
}

pub async fn create_order(
    claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
    new_order_payload: web::Json<CreateOrderPayload>,
) -> impl Responder {
    let store_id = match claims.0.store_id {
        Some(id) => id,
        None => return HttpResponse::Forbidden().json(ApiError::new("User is not assigned to a store".to_string())),
    };

    let txn = match db.begin().await {
        Ok(txn) => txn,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to start transaction: {}", e))),
    };

    let employee_id = claims.0.sub;
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
                if promotion.is_active != 0 && promotion.start_date <= Utc::now() && promotion.end_date >= Utc::now() {
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
        if let Err(e) = InventoryRepository::decrease_quantity(&txn, product.id, store_id, item_payload.quantity).await {
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

    // Create the payment
    let payment_to_create = payments::CreatePayment {
        order_id: order.id,
        payment_method: new_order_payload.payment_method.clone(),
        amount: order.total_amount, // Assuming amount paid is the total amount for now
        payment_date: Utc::now(),
        status: "Completed".to_string(),
    };

    if let Err(e) = PaymentRepository::create_in_txn(&txn, payment_to_create).await {
        return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to create payment: {}", e)));
    }

    // Update the order status
    let mut order_active_model: orders::ActiveModel = order.into();
    order_active_model.status = ActiveValue::Set("Completed".to_string());
    let updated_order = match order_active_model.update(&txn).await {
        Ok(o) => o,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to update order status: {}", e))),
    };

    // Commit transaction
    if let Err(e) = txn.commit().await {
        return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to commit transaction: {}", e)));
    }

    HttpResponse::Ok().json(ApiResponse::new(updated_order))
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

pub async fn get_sales_report(
    claims: ClaimsExtractor,
    db: web::Data<DatabaseConnection>,
    query_params: web::Query<SalesReportQueryParams>,
) -> impl Responder {
    if claims.0.role != "Admin" && claims.0.role != "StoreManager" {
        return HttpResponse::Forbidden().json(ApiError::new("Forbidden: Insufficient privileges".to_string()));
    }

    let mut condition = Condition::all();

    if let Some(start_date) = query_params.start_date {
        condition = condition.add(OrderColumn::OrderDate.gte(start_date));
    }
    if let Some(end_date) = query_params.end_date {
        condition = condition.add(OrderColumn::OrderDate.lte(end_date));
    }
    if let Some(store_id) = query_params.store_id {
        condition = condition.add(OrderColumn::StoreId.eq(store_id));
    }
    // If StoreManager, filter by their store_id
    if claims.0.role == "StoreManager" {
        if let Some(store_id) = claims.0.store_id {
            condition = condition.add(OrderColumn::StoreId.eq(store_id));
        }
    }
    if let Some(employee_id) = query_params.employee_id {
        condition = condition.add(OrderColumn::EmployeeId.eq(employee_id));
    }

    // Fetch all relevant orders with their items, products, and employees
    let orders_with_items_and_employees = match OrderEntity::find()
        .filter(condition)
        .left_join(order_items::Entity)
        .left_join(employees::Entity)
        .select_only()
        .column(OrderColumn::Id)
        .column(OrderColumn::TotalAmount)
        .column(OrderColumn::EmployeeId)
        .column(employees::Column::FirstName)
        .column(employees::Column::LastName)
        .column(order_items::Column::ProductId)
        .column(order_items::Column::Quantity)
        .column(order_items::Column::UnitPrice)
        .column(order_items::Column::DiscountAmount)
        .into_json()
        .all(db.get_ref())
        .await
    {
        Ok(data) => data,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch sales data: {}", e))),
    };

    let all_products = match products::Entity::find().all(db.get_ref()).await {
        Ok(p) => p,
        Err(e) => return HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch products: {}", e))),
    };
    let products_map: HashMap<i32, products::Model> = all_products.into_iter().map(|p| (p.id, p)).collect();

    let mut total_sales_amount = Decimal::new(0, 2);
    let mut total_orders = 0u64;
    let mut product_sales_map: HashMap<i32, ProductSalesReport> = HashMap::new();
    let mut employee_sales_map: HashMap<i32, EmployeeSalesReport> = HashMap::new();

    let mut processed_order_ids = std::collections::HashSet::new();

    for row in orders_with_items_and_employees {
        let order_id: i32 = row["id"].as_i64().unwrap() as i32;
        let order_total_amount: Decimal = Decimal::from_str(&row["total_amount"].as_str().unwrap()).unwrap();
        let employee_id: i32 = row["employee_id"].as_i64().unwrap() as i32;
        let employee_first_name: String = row["employees"].as_object().unwrap()["first_name"].as_str().unwrap().to_string();
        let employee_last_name: String = row["employees"].as_object().unwrap()["last_name"].as_str().unwrap().to_string();
        let product_id: i32 = row["order_items"].as_object().unwrap()["product_id"].as_i64().unwrap() as i32;
        let quantity: i32 = row["order_items"].as_object().unwrap()["quantity"].as_i64().unwrap() as i32;
        let unit_price: Decimal = Decimal::from_str(&row["order_items"].as_object().unwrap()["unit_price"].as_str().unwrap()).unwrap();
        let discount_amount: Decimal = Decimal::from_str(&row["order_items"].as_object().unwrap()["discount_amount"].as_str().unwrap()).unwrap();
        let product_name: String = products_map.get(&product_id).map(|p| p.name.clone()).unwrap_or_else(|| "Unknown Product".to_string());

        // Aggregate total sales amount and total orders
        if processed_order_ids.insert(order_id) {
            total_sales_amount += order_total_amount;
            total_orders += 1;
        }

        // Aggregate product sales
        let product_revenue = (unit_price - discount_amount) * Decimal::from(quantity);
        product_sales_map.entry(product_id)
            .and_modify(|e| {
                e.quantity_sold += quantity;
                e.total_revenue += product_revenue;
            })
            .or_insert(ProductSalesReport {
                product_id,
                product_name: product_name.clone(),
                quantity_sold: quantity,
                total_revenue: product_revenue,
            });

        // Aggregate employee sales
        employee_sales_map.entry(employee_id)
            .and_modify(|e| {
                e.total_sales_handled += order_total_amount;
            })
            .or_insert(EmployeeSalesReport {
                employee_id,
                employee_name: format!("{} {}", employee_first_name, employee_last_name),
                total_sales_handled: order_total_amount,
            });
    }

    let sales_report = SalesReport {
        total_sales_amount,
        total_orders,
        product_sales: product_sales_map.into_values().collect(),
        employee_sales: employee_sales_map.into_values().collect(),
    };

    HttpResponse::Ok().json(ApiResponse::new(sales_report))
}


