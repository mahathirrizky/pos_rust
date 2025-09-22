use serde::Serialize;
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use sea_orm::DbErr;
use std::fmt;

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

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self {
            AppError::NotFound(msg) => msg.clone(),
            AppError::InternalError(_) => "An unexpected internal error occurred.".to_string(),
        };

        let error_json = ApiError::new(message);

        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .json(error_json)
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::InternalError(err.to_string())
    }
}
