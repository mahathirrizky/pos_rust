use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorNotFound},
    web, FromRequest, HttpRequest,
};
use std::pin::Pin;
use std::future::Future;

use crate::auth::auth_service::Claims;
use crate::entities::orders;
use crate::extractor::claims_extractor::ClaimsExtractor;
use crate::repository::orders_repository::OrderRepository;
use sea_orm::DatabaseConnection;

#[allow(dead_code)]
pub struct OrderAccessGuard {
    pub claims: Claims,
    pub order: orders::Model,
}

impl FromRequest for OrderAccessGuard {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        let mut payload = payload.take();

        Box::pin(async move {
            let claims_extractor = ClaimsExtractor::from_request(&req, &mut payload).await?;
            let claims = claims_extractor.0;

            let target_id: i32 = req.match_info()
                .get("id")
                .and_then(|id| id.parse().ok())
                .ok_or_else(|| ErrorNotFound("Missing or invalid order ID in path"))?;

            let db = req.app_data::<web::Data<DatabaseConnection>>()
                .ok_or_else(|| ErrorForbidden("Database connection not found"))?;

            let order = OrderRepository::find_by_id(db.get_ref(), target_id)
                .await
                .map_err(|_| ErrorForbidden("Error fetching order"))?
                .ok_or_else(|| ErrorNotFound("Order not found"))?;

            let has_access = if claims.role == "Admin" {
                true
            } else if claims.role == "StoreManager" {
                order.store_id == claims.store_id.unwrap_or(-1)
            } else {
                order.employee_id == claims.sub
            };

            if has_access {
                Ok(OrderAccessGuard { claims, order })
            } else {
                Err(ErrorForbidden("Forbidden: Access denied to this order resource"))
            }
        })
    }
}
