use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    ServiceUnavailable(String),
    TooManyRequests(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::ServiceUnavailable(msg) => write!(f, "Service Unavailable: {}", msg),
            ApiError::TooManyRequests(msg) => write!(f, "Too Many Requests: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "message": msg
                }))
            },
            ApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Bad Request",
                    "message": msg
                }))
            },
            ApiError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Unauthorized",
                    "message": msg
                }))
            },
            ApiError::Forbidden(msg) => {
                HttpResponse::Forbidden().json(json!({
                    "error": "Forbidden",
                    "message": msg
                }))
            },
            ApiError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({
                    "error": "Not Found",
                    "message": msg
                }))
            },
            ApiError::ServiceUnavailable(msg) => {
                HttpResponse::ServiceUnavailable().json(json!({
                    "error": "Service Unavailable",
                    "message": msg
                }))
            },
            ApiError::TooManyRequests(msg) => {
                HttpResponse::TooManyRequests().json(json!({
                    "error": "Too Many Requests",
                    "message": msg
                }))
            },
        }
    }
} 