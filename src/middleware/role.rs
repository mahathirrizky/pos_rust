
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

pub struct RoleMiddlewareFactory {
    pub allowed_roles: Vec<String>,
}

impl<S, B> Transform<S, ServiceRequest> for RoleMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RoleMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleMiddleware {
            service: std::sync::Arc::new(service),
            allowed_roles: self.allowed_roles.clone(),
        }))
    }
}

pub struct RoleMiddleware<S> {
    service: std::sync::Arc<S>,
    allowed_roles: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for RoleMiddleware<S>
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
        let allowed_roles = self.allowed_roles.clone();

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
                if allowed_roles.is_empty() || allowed_roles.contains(&claims.role) {
                    // Add claims to request extensions for handlers to use
                    req.extensions_mut().insert(claims);
                    service.call(req).await
                } else {
                    Err(ErrorForbidden("Forbidden: Insufficient role"))
                }
            } else {
                Err(ErrorForbidden("Forbidden: Invalid or missing token"))
            }
        })
    }
}
