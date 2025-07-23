// 事件类型定义
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 系统事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    ServiceStarted {
        service_name: String,
        version: String,
        timestamp: DateTime<Utc>,
    },
    ServiceStopped {
        service_name: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    HealthCheckFailed {
        service_name: String,
        error: String,
        timestamp: DateTime<Utc>,
    },
    MetricsReported {
        service_name: String,
        metrics: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
}

impl SystemEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            SystemEvent::ServiceStarted { .. } => "system.service.started",
            SystemEvent::ServiceStopped { .. } => "system.service.stopped",
            SystemEvent::HealthCheckFailed { .. } => "system.health.failed",
            SystemEvent::MetricsReported { .. } => "system.metrics.reported",
        }
    }
}

/// 业务事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessEvent {
    UserRegistered {
        user_id: Uuid,
        email: String,
        timestamp: DateTime<Utc>,
    },
    OrderCreated {
        order_id: Uuid,
        user_id: Uuid,
        total_amount: f64,
        timestamp: DateTime<Utc>,
    },
    PaymentProcessed {
        payment_id: Uuid,
        order_id: Uuid,
        amount: f64,
        timestamp: DateTime<Utc>,
    },
    ProductUpdated {
        product_id: Uuid,
        changes: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
}

impl BusinessEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            BusinessEvent::UserRegistered { .. } => "business.user.registered",
            BusinessEvent::OrderCreated { .. } => "business.order.created",
            BusinessEvent::PaymentProcessed { .. } => "business.payment.processed",
            BusinessEvent::ProductUpdated { .. } => "business.product.updated",
        }
    }
} 