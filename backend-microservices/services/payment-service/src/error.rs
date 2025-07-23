use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use thiserror::Error;

#[derive(Debug)]
pub enum ServiceError {
    InternalError(String),
    NotFound(String),
    ValidationError(String),
    UnauthorizedError(String),
    PaymentError(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            ServiceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ServiceError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ServiceError::UnauthorizedError(msg) => write!(f, "Unauthorized: {}", msg),
            ServiceError::PaymentError(msg) => write!(f, "Payment error: {}", msg),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_error",
                    "message": msg
                }))
            }
            ServiceError::NotFound(msg) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "not_found",
                    "message": msg
                }))
            }
            ServiceError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "validation_error",
                    "message": msg
                }))
            }
            ServiceError::UnauthorizedError(msg) => {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "unauthorized",
                    "message": msg
                }))
            }
            ServiceError::PaymentError(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "payment_error",
                    "message": msg
                }))
            }
        }
    }
}

// PaymentError for provider modules
#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Provider error: {0}")]
    ProviderError(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Payment not found")]
    PaymentNotFound,
    
    #[error("Operation not supported: {0}")]
    NotSupported(String),
}

impl From<PaymentError> for ServiceError {
    fn from(err: PaymentError) -> Self {
        ServiceError::PaymentError(err.to_string())
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(err: anyhow::Error) -> Self {
        ServiceError::InternalError(err.to_string())
    }
} 