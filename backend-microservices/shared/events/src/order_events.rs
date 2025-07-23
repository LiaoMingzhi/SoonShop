// 订单事件定义 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEventData {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub total: f64,
} 