use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn get_current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
    if auth_header.starts_with("Bearer ") {
        Some(&auth_header[7..])
    } else {
        None
    }
}

pub fn validate_pagination_params(page: Option<u32>, limit: Option<u32>) -> (u32, u32) {
    let page = page.unwrap_or(1).max(1);
    let limit = limit.unwrap_or(20).min(100).max(1);
    (page, limit)
}

pub fn create_response_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("X-Request-ID".to_string(), generate_request_id());
    headers.insert("X-Timestamp".to_string(), get_current_timestamp().to_rfc3339());
    headers
}

#[derive(Debug, Serialize)]
pub struct HealthCheck {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub uptime: u64,
}

impl HealthCheck {
    pub fn new(uptime: u64) -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime,
        }
    }
} 