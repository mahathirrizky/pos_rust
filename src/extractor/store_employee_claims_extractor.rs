use actix_web::{
    dev::Payload,
    error::ErrorForbidden,
    FromRequest, HttpRequest,
    web::Path,
};
use futures_util::future::{Future};
use std::pin::Pin;
use crate::auth::auth_service::Claims;
use super::claims_extractor::ClaimsExtractor;

#[allow(dead_code)]
pub struct StoreEmployeeAccess {
    pub claims: Claims,
    pub id: i32,
}

impl FromRequest for StoreEmployeeAccess {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let (claims_future, path_future) = (ClaimsExtractor::from_request(req, payload), Path::<i32>::from_request(req, payload));

        Box::pin(async move {
            let claims_extractor = claims_future.await?;
            let path_id = path_future.await?;
            let claims = claims_extractor.0;
            let id = path_id.into_inner();

            // Placeholder for actual authorization logic
            // This will be replaced with specific logic for each handler
            // For now, just allow if Admin
            if claims.role == "Admin" {
                Ok(StoreEmployeeAccess { claims, id })
            } else {
                Err(ErrorForbidden("Forbidden: Insufficient role"))
            }
        })
    }
}
