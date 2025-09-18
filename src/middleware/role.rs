
use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    error::ErrorForbidden,
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use crate::auth::auth_service::Claims;

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
            let claims = req.extensions().get::<Claims>().cloned();
            if let Some(claims) = claims {
                if allowed_roles.contains(&claims.role) {
                    let fut = service.call(req);
                    fut.await
                } else {
                    Err(ErrorForbidden("Forbidden: Insufficient role"))
                }
            } else {
                Err(ErrorForbidden("Forbidden: No claims found"))
            }
        })
    }
}
