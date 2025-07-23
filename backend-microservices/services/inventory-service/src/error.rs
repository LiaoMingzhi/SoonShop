use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum InventoryError {
    DatabaseError(String),
    ValidationError(String),
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            InventoryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            InventoryError::NotFound(msg) => write!(f, "Not found: {}", msg),
            InventoryError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for InventoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            InventoryError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Database error")
            }
            InventoryError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(msg)
            }
            InventoryError::NotFound(msg) => {
                HttpResponse::NotFound().json(msg)
            }
            InventoryError::InternalError(_) => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }
} 