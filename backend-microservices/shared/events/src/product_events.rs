// 产品事件定义 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCreatedEventData {
    pub product_id: Uuid,
    pub name: String,
    pub price: f64,
    pub created_at: DateTime<Utc>,
} 