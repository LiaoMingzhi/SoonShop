use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// B2C服务相关事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum B2CEvent {
    // 购物车事件
    CartCreated {
        cart_id: Uuid,
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    CartUpdated {
        cart_id: Uuid,
        user_id: Uuid,
        total_amount: f64,
        timestamp: DateTime<Utc>,
    },
    CartCleared {
        cart_id: Uuid,
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ItemAddedToCart {
        cart_id: Uuid,
        user_id: Uuid,
        product_id: Uuid,
        quantity: u32,
        timestamp: DateTime<Utc>,
    },
    ItemRemovedFromCart {
        cart_id: Uuid,
        user_id: Uuid,
        product_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    // 订单事件
    OrderCreated {
        order_id: Uuid,
        user_id: Uuid,
        order_number: String,
        total_amount: f64,
        timestamp: DateTime<Utc>,
    },
    OrderStatusUpdated {
        order_id: Uuid,
        user_id: Uuid,
        old_status: String,
        new_status: String,
        timestamp: DateTime<Utc>,
    },
    OrderCancelled {
        order_id: Uuid,
        user_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    OrderCompleted {
        order_id: Uuid,
        user_id: Uuid,
        total_amount: f64,
        timestamp: DateTime<Utc>,
    },
    
    // 商品浏览事件
    ProductViewed {
        user_id: Uuid,
        product_id: Uuid,
        category_id: Uuid,
        duration: Option<u32>,
        timestamp: DateTime<Utc>,
    },
    
    // 推荐事件
    RecommendationRequested {
        user_id: Uuid,
        recommendation_type: String,
        timestamp: DateTime<Utc>,
    },
    RecommendationGenerated {
        user_id: Uuid,
        product_ids: Vec<Uuid>,
        algorithm: String,
        timestamp: DateTime<Utc>,
    },
}

impl B2CEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            B2CEvent::CartCreated { .. } => "cart_created",
            B2CEvent::CartUpdated { .. } => "cart_updated",
            B2CEvent::CartCleared { .. } => "cart_cleared",
            B2CEvent::ItemAddedToCart { .. } => "item_added_to_cart",
            B2CEvent::ItemRemovedFromCart { .. } => "item_removed_from_cart",
            B2CEvent::OrderCreated { .. } => "order_created",
            B2CEvent::OrderStatusUpdated { .. } => "order_status_updated",
            B2CEvent::OrderCancelled { .. } => "order_cancelled",
            B2CEvent::OrderCompleted { .. } => "order_completed",
            B2CEvent::ProductViewed { .. } => "product_viewed",
            B2CEvent::RecommendationRequested { .. } => "recommendation_requested",
            B2CEvent::RecommendationGenerated { .. } => "recommendation_generated",
        }
    }
    
    pub fn get_user_id(&self) -> Uuid {
        match self {
            B2CEvent::CartCreated { user_id, .. } => *user_id,
            B2CEvent::CartUpdated { user_id, .. } => *user_id,
            B2CEvent::CartCleared { user_id, .. } => *user_id,
            B2CEvent::ItemAddedToCart { user_id, .. } => *user_id,
            B2CEvent::ItemRemovedFromCart { user_id, .. } => *user_id,
            B2CEvent::OrderCreated { user_id, .. } => *user_id,
            B2CEvent::OrderStatusUpdated { user_id, .. } => *user_id,
            B2CEvent::OrderCancelled { user_id, .. } => *user_id,
            B2CEvent::OrderCompleted { user_id, .. } => *user_id,
            B2CEvent::ProductViewed { user_id, .. } => *user_id,
            B2CEvent::RecommendationRequested { user_id, .. } => *user_id,
            B2CEvent::RecommendationGenerated { user_id, .. } => *user_id,
        }
    }
    
    pub fn get_timestamp(&self) -> DateTime<Utc> {
        match self {
            B2CEvent::CartCreated { timestamp, .. } => *timestamp,
            B2CEvent::CartUpdated { timestamp, .. } => *timestamp,
            B2CEvent::CartCleared { timestamp, .. } => *timestamp,
            B2CEvent::ItemAddedToCart { timestamp, .. } => *timestamp,
            B2CEvent::ItemRemovedFromCart { timestamp, .. } => *timestamp,
            B2CEvent::OrderCreated { timestamp, .. } => *timestamp,
            B2CEvent::OrderStatusUpdated { timestamp, .. } => *timestamp,
            B2CEvent::OrderCancelled { timestamp, .. } => *timestamp,
            B2CEvent::OrderCompleted { timestamp, .. } => *timestamp,
            B2CEvent::ProductViewed { timestamp, .. } => *timestamp,
            B2CEvent::RecommendationRequested { timestamp, .. } => *timestamp,
            B2CEvent::RecommendationGenerated { timestamp, .. } => *timestamp,
        }
    }
} 