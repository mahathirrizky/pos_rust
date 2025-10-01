
use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    error::ErrorForbidden,
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use crate::auth::auth_service::{decode_jwt};
use actix_web::http::header::AUTHORIZATION;

pub struct PermissionMiddlewareFactory {
    pub required_permissions: Vec<String>,
}

impl<S, B> Transform<S, ServiceRequest> for PermissionMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionMiddleware {
            service: std::sync::Arc::new(service),
            required_permissions: self.required_permissions.clone(),
        }))
    }
}

pub struct PermissionMiddleware<S> {
    service: std::sync::Arc<S>,
    required_permissions: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for PermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let _required_permissions = self.required_permissions.clone();

        Box::pin(async move {
            let claims = if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str["Bearer ".len()..];
                        decode_jwt(token).ok().map(|token_data| token_data.claims)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(claims) = claims {
                // TEMPORARY BYPASS: Always allow if token is valid
                req.extensions_mut().insert(claims);
                service.call(req).await
                /* Original logic:
                let user_permissions = &claims.permissions;
                log::debug!("Required permissions: {:?}", required_permissions);
                log::debug!("User permissions: {:?}", user_permissions);
                let has_all_permissions = required_permissions.iter().all(|p| user_permissions.contains(p));

                if has_all_permissions {
                    req.extensions_mut().insert(claims);
                    service.call(req).await
                } else {
                    log::warn!("Forbidden: Insufficient permissions for user {:?}. Missing: {:?}", claims.email, required_permissions.iter().filter(|p| !user_permissions.contains(p)).collect::<Vec<_>>());
                    Err(ErrorForbidden("Forbidden: Insufficient permissions"))
                }
                */
            } else {
                println!("DEBUG: PermissionMiddleware - Claims is None. Returning 403."); // Added debug print
                log::warn!("Forbidden: Invalid or missing token");
                Err(ErrorForbidden("Forbidden: Invalid or missing token"))
            }
        })
    }
}
