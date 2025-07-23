// 用户事件定义 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEventData {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEventData {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub updated_at: DateTime<Utc>,
} 