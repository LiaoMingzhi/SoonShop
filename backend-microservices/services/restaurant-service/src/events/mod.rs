// 餐饮服务事件定义
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestaurantEvent {
    ReservationCreated {
        reservation_id: Uuid,
        user_id: Uuid,
        restaurant_id: Uuid,
        guest_count: u32,
        reservation_date: DateTime<Utc>,
        timestamp: DateTime<Utc>,
    },
    ReservationConfirmed {
        reservation_id: Uuid,
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ReservationCancelled {
        reservation_id: Uuid,
        user_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
} 