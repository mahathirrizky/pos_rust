use actix_web::{
    dev::Payload,
    error::ErrorUnauthorized,
    http::header,
    FromRequest, HttpRequest,
};
use futures_util::future::{ready, Ready};
use crate::auth::auth_service::{self, Claims};

// This is our new, self-contained JWT extractor
pub struct ClaimsExtractor(pub Claims);

impl FromRequest for ClaimsExtractor {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // 1. Get the Authorization header
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => match header.to_str() {
                Ok(s) => s,
                Err(_) => return ready(Err(ErrorUnauthorized("Invalid Authorization header"))),
            },
            None => return ready(Err(ErrorUnauthorized("Missing Authorization header"))),
        };

        // 2. Check for "Bearer " prefix
        if !auth_header.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("Invalid token format")));
        }

        // 3. Extract the token
        let token = &auth_header[7..];

        // 4. Decode and validate the token
        match auth_service::decode_jwt(token) {
            Ok(token_data) => ready(Ok(ClaimsExtractor(token_data.claims))),
            Err(_) => ready(Err(ErrorUnauthorized("Invalid or expired token"))),
        }
    }
}