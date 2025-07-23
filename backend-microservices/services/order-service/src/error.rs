use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum OrderError {
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl ResponseError for OrderError {
    fn error_response(&self) -> HttpResponse {
        let error_response = json!({
            "error": self.to_string(),
            "type": self.error_type()
        });
        
        match self {
            OrderError::InternalError(_) => HttpResponse::InternalServerError().json(error_response),
            OrderError::NotFound(_) => HttpResponse::NotFound().json(error_response),
            OrderError::ValidationError(_) => HttpResponse::BadRequest().json(error_response),
        }
    }
}

impl OrderError {
    fn error_type(&self) -> &'static str {
        match self {
            OrderError::InternalError(_) => "internal_error",
            OrderError::NotFound(_) => "not_found",
            OrderError::ValidationError(_) => "validation_error",
        }
    }
} 