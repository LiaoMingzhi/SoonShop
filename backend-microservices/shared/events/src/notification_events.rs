// 通知事件定义 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSentEventData {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub message: String,
    pub channel: String, // "email" | "sms" | "push"
} 