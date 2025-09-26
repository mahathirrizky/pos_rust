use actix_web::{web, HttpResponse, Responder};
use crate::helper::response::{ApiResponse, ApiError};
use crate::repository::reports_repository::ReportRepository;
use sea_orm::{DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use log;

#[derive(Deserialize)]
pub struct SalesSummaryQuery {
    pub period: String,
}

pub async fn get_sales_summary(db: web::Data<DatabaseConnection>, query: web::Query<SalesSummaryQuery>) -> impl Responder {
    match ReportRepository::get_total_sales_by_period(db.get_ref(), &query.period).await {
        Ok(total_sales) => HttpResponse::Ok().json(ApiResponse::new(total_sales)),
        Err(DbErr::Custom(e)) if e == "Invalid period specified" => HttpResponse::BadRequest().json(ApiError::new(e)),
        Err(_) => HttpResponse::InternalServerError().json(ApiError::new("Failed to fetch sales summary".to_string())),
    }
}


#[derive(Deserialize)]
pub struct ReportQuery {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub store_id: Option<i32>,
}

#[derive(Serialize)]
pub struct ReportSummary {
    total_revenue: f64,
    total_transactions: i64,
    sales_over_time: Vec<(String, f64)>,
    sales_by_store: Vec<(String, f64)>,
}

pub async fn get_reports_summary(db: web::Data<DatabaseConnection>, query: web::Query<ReportQuery>) -> impl Responder {
    let db_ref = db.get_ref();
    let store_id = query.store_id;

    // Use NaiveDateTime for the start and end of the day
    let start_datetime = query.start_date.and_hms_opt(0, 0, 0).unwrap();
    let end_datetime = query.end_date.and_hms_opt(23, 59, 59).unwrap();

    let summary_result = tokio::try_join!(
        ReportRepository::get_total_revenue(db_ref, start_datetime, end_datetime, store_id),
        ReportRepository::get_total_transactions(db_ref, start_datetime, end_datetime, store_id),
        ReportRepository::get_sales_over_time(db_ref, start_datetime, end_datetime, store_id),
        ReportRepository::get_sales_by_store(db_ref, start_datetime, end_datetime, store_id)
    );

    match summary_result {
        Ok((total_revenue, total_transactions, sales_over_time, sales_by_store)) => {
            let summary = ReportSummary {
                total_revenue,
                total_transactions,
                sales_over_time,
                sales_by_store,
            };
            HttpResponse::Ok().json(ApiResponse::new(summary))
        }
        Err(e) => {
            log::error!("Failed to fetch report summary. Error: {:?}", e);
            HttpResponse::InternalServerError().json(ApiError::new(format!("Failed to fetch report summary: {}", e)))
        }
    }
}
