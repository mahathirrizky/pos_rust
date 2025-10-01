use actix_web::{
    get,
    post,
    web::{Data, Json},
    HttpResponse,
    Responder,
};

use crate::entities::settings_model::Settings as SettingsModel;
use crate::repository::settings_repository;
use crate::AppState;

#[get("/settings")]
pub async fn get_settings(data: Data<AppState>) -> impl Responder {
    let db = &data.db;
    match settings_repository::get_settings(db).await {
        Ok(settings) => HttpResponse::Ok().json(serde_json::json!({ "data": settings })),
        Err(e) => {
            eprintln!("Error getting settings: {}", e);
            HttpResponse::InternalServerError().json("Failed to retrieve settings")
        }
    }
}

#[post("/settings")]
pub async fn save_settings(
    data: Data<AppState>,
    payload: Json<SettingsModel>,
) -> impl Responder {
    let db = &data.db;
    let settings_data = payload.into_inner();

    match settings_repository::update_settings(db, settings_data).await {
        Ok(updated_settings) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Settings saved successfully.",
                "data": updated_settings
            }))
        }
        Err(e) => {
            eprintln!("Error saving settings: {}", e);
            HttpResponse::InternalServerError().json("Failed to save settings")
        }
    }
}
