use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorNotFound},
    web, FromRequest, HttpRequest,
};
use std::pin::Pin;
use std::future::Future;

use crate::entities::inventory;
use crate::auth::auth_service::Claims;
use crate::repository::inventory_repository::InventoryRepository;
use sea_orm::DatabaseConnection;

#[allow(dead_code)]
pub struct InventoryAccessGuard {
    pub inventory: inventory::Model,
}

impl FromRequest for InventoryAccessGuard {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        let mut payload = payload.take();

        Box::pin(async move {
            let claims = Claims::from_request(&req, &mut payload).await?;

            let target_id: i32 = req.match_info()
                .get("id")
                .and_then(|id| id.parse().ok())
                .ok_or_else(|| ErrorNotFound("Missing or invalid inventory ID in path"))?;

            let db = req.app_data::<web::Data<DatabaseConnection>>()
                .ok_or_else(|| ErrorForbidden("Database connection not found"))?;

            let inventory = InventoryRepository::find_by_id(db.get_ref(), target_id)
                .await
                .map_err(|_| ErrorForbidden("Error fetching inventory"))?
                .ok_or_else(|| ErrorNotFound("Inventory not found"))?;

            let has_access = if claims.role == "Admin" {
                true
            } else {
                claims.store_id == Some(inventory.store_id)
            };

            if has_access {
                Ok(InventoryAccessGuard { inventory })
            } else {
                Err(ErrorForbidden("Forbidden: Access denied to this inventory resource"))
            }
        })
    }
}
