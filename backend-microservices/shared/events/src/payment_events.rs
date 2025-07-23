// 支付事件定义 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProcessedEventData {
    pub payment_id: Uuid,
    pub order_id: Uuid,
    pub amount: f64,
    pub status: String,
} 