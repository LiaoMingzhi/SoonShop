// 库存事件定义 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryUpdatedEventData {
    pub product_id: Uuid,
    pub quantity: i32,
    pub operation: String, // "increase" | "decrease"
} 