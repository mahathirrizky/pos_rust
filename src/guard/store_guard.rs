use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorNotFound},
    web, FromRequest, HttpRequest,
};
use std::pin::Pin;
use std::future::Future;

use crate::entities::stores;
use crate::auth::auth_service::Claims;
use crate::repository::stores_repository::StoreRepository;
use sea_orm::DatabaseConnection;

#[allow(dead_code)]
pub struct StoreAccessGuard {
    pub store: stores::Model,
}

impl FromRequest for StoreAccessGuard {
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
                .ok_or_else(|| ErrorNotFound("Missing or invalid store ID in path"))?;

            let db = req.app_data::<web::Data<DatabaseConnection>>()
                .ok_or_else(|| ErrorForbidden("Database connection not found"))?;

            let store = StoreRepository::find_by_id(db.get_ref(), target_id)
                .await
                .map_err(|_| ErrorForbidden("Error fetching store"))?
                .ok_or_else(|| ErrorNotFound("Store not found"))?;

            let has_access = if claims.role == "Admin" || claims.role == "Owner" {
                true
            } else {
                // Ensure both store_id are Some and match
                claims.store_id == Some(store.id)
            };

            if has_access {
                Ok(StoreAccessGuard { store })
            } else {
                Err(ErrorForbidden("Forbidden: Access denied to this store resource"))
            }
        })
    }
}
