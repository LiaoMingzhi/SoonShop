use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 事件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: Uuid,
    pub event_type: String,
    pub event_version: String,
    pub source: String,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub tags: HashMap<String, String>,
}

impl EventMetadata {
    pub fn new(event_type: String, source: String) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type,
            event_version: "1.0".to_string(),
            source,
            correlation_id: None,
            causation_id: None,
            timestamp: Utc::now(),
            user_id: None,
            session_id: None,
            tags: HashMap::new(),
        }
    }
    
    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    
    pub fn with_causation_id(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }
    
    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }
    
    pub fn with_tag(mut self, key: String, value: String) -> Self {
        self.tags.insert(key, value);
        self
    }
}

/// 简化的事件接口 - object safe
pub trait Event: Send + Sync + std::fmt::Debug {
    fn event_type(&self) -> &'static str;
    fn event_version(&self) -> &'static str { "1.0" }
    fn to_json(&self) -> serde_json::Result<serde_json::Value>;
}

/// 事件包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub metadata: EventMetadata,
    pub payload: serde_json::Value,
}

impl EventEnvelope {
    pub fn new<E: Event>(event: &E, metadata: EventMetadata) -> serde_json::Result<Self> {
        Ok(Self {
            metadata,
            payload: event.to_json()?,
        })
    }
}

/// 领域事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub event_type: String,
    pub event_version: String,
    pub sequence_number: u64,
    pub payload: serde_json::Value,
    pub metadata: EventMetadata,
}

impl DomainEvent {
    pub fn new<T: Event + Serialize>(
        aggregate_id: Uuid,
        aggregate_type: String,
        sequence_number: u64,
        event: T,
        metadata: EventMetadata,
    ) -> Self {
        Self {
            aggregate_id,
            aggregate_type,
            event_type: event.event_type().to_string(),
            event_version: event.event_version().to_string(),
            sequence_number,
            payload: serde_json::to_value(event).unwrap(),
            metadata,
        }
    }
}

/// 系统事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEventType {
    ServiceStarted,
    ServiceStopped,
    ServiceHealthCheck,
    DatabaseConnected,
    DatabaseDisconnected,
    MessageProcessed,
    MessageFailed,
    Custom(String),
}

impl std::fmt::Display for SystemEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemEventType::ServiceStarted => write!(f, "service.started"),
            SystemEventType::ServiceStopped => write!(f, "service.stopped"),
            SystemEventType::ServiceHealthCheck => write!(f, "service.health_check"),
            SystemEventType::DatabaseConnected => write!(f, "database.connected"),
            SystemEventType::DatabaseDisconnected => write!(f, "database.disconnected"),
            SystemEventType::MessageProcessed => write!(f, "message.processed"),
            SystemEventType::MessageFailed => write!(f, "message.failed"),
            SystemEventType::Custom(name) => write!(f, "custom.{}", name),
        }
    }
}

// 预定义的业务事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

impl Event for UserCreatedEvent {
    fn event_type(&self) -> &'static str {
        "user.created"
    }
    
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub total_amount: rust_decimal::Decimal,
    pub items: Vec<OrderItemEvent>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemEvent {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: rust_decimal::Decimal,
}

impl Event for OrderCreatedEvent {
    fn event_type(&self) -> &'static str {
        "order.created"
    }
    
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProcessedEvent {
    pub payment_id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub payment_method: String,
    pub status: String,
    pub processed_at: DateTime<Utc>,
}

impl Event for PaymentProcessedEvent {
    fn event_type(&self) -> &'static str {
        "payment.processed"
    }
    
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryReservedEvent {
    pub reservation_id: Uuid,
    pub product_id: Uuid,
    pub order_id: Uuid,
    pub quantity: i32,
    pub warehouse_id: Uuid,
    pub reserved_at: DateTime<Utc>,
}

impl Event for InventoryReservedEvent {
    fn event_type(&self) -> &'static str {
        "inventory.reserved"
    }
    
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSentEvent {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub channel: String,
    pub status: String,
    pub sent_at: DateTime<Utc>,
}

impl Event for NotificationSentEvent {
    fn event_type(&self) -> &'static str {
        "notification.sent"
    }
    
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoucherCreatedEvent {
    pub voucher_id: Uuid,
    pub user_id: Uuid,
    pub voucher_type: String,
    pub amount: rust_decimal::Decimal,
    pub multiplier: f64,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Event for VoucherCreatedEvent {
    fn event_type(&self) -> &'static str {
        "voucher.created"
    }
    
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCompletedEvent {
    pub evaluation_id: Uuid,
    pub enterprise_id: Uuid,
    pub score: f64,
    pub multiplier: f64,
    pub evaluated_at: DateTime<Utc>,
    pub evaluator_id: Uuid,
}

impl Event for EvaluationCompletedEvent {
    fn event_type(&self) -> &'static str {
        "evaluation.completed"
    }
    
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculatedEvent {
    pub reward_id: Uuid,
    pub user_id: Uuid,
    pub enterprise_id: Uuid,
    pub base_amount: rust_decimal::Decimal,
    pub multiplier: f64,
    pub final_amount: rust_decimal::Decimal,
    pub calculated_at: DateTime<Utc>,
}

impl Event for RewardCalculatedEvent {
    fn event_type(&self) -> &'static str {
        "reward.calculated"
    }
    
    fn to_json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

// 事件流
#[derive(Debug, Clone)]
pub struct EventStream {
    pub stream_id: String,
    pub events: Vec<DomainEvent>,
    pub version: u64,
}

impl EventStream {
    pub fn new(stream_id: String) -> Self {
        Self {
            stream_id,
            events: Vec::new(),
            version: 0,
        }
    }
    
    pub fn append_event(&mut self, event: DomainEvent) {
        self.version += 1;
        self.events.push(event);
    }
    
    pub fn uncommitted_events(&self) -> &[DomainEvent] {
        &self.events
    }
    
    pub fn mark_committed(&mut self) {
        self.events.clear();
    }
} 