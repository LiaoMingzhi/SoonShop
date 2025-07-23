// 消息队列相关工具 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T> {
    pub id: Uuid,
    pub topic: String,
    pub payload: T,
    pub timestamp: i64,
}

impl<T> Message<T> {
    pub fn new(topic: String, payload: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            topic,
            payload,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
} 