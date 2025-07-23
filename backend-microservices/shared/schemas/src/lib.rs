pub mod user;
pub mod product;
pub mod order;
pub mod payment;
pub mod inventory;
pub mod notification;

pub use user::*;
pub use product::*;
pub use order::*;
pub use payment::*;
pub use inventory::*;
pub use notification::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
}

impl PaginationInfo {
    pub fn new(page: u64, limit: u64, total: u64) -> Self {
        let total_pages = (total + limit - 1) / limit;
        Self {
            page,
            limit,
            total,
            total_pages,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdResponse {
    pub id: Uuid,
}

impl IdResponse {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
} 