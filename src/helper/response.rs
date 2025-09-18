use serde::Serialize;
use serde_json;
use actix_web::{HttpResponse, body::BoxBody, http::header::ContentType};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        ApiResponse {
            success: true,
            data,
        }
    }
}

#[derive(Serialize)]
pub struct ApiError {
    pub success: bool,
    pub message: String,
}

impl ApiError {
    pub fn new(message: String) -> Self {
        ApiError {
            success: false,
            message,
        }
    }
}

// This is a bit more advanced, but allows to use ApiError as a response type directly
impl actix_web::Responder for ApiError {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(body)
    }
}
