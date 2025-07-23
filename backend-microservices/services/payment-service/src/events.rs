use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentEvent {
    PaymentCreated {
        payment_id: Uuid,
        order_id: Uuid,
        user_id: Uuid,
        amount: Decimal,
        currency: String,
        payment_method: String,
        created_at: DateTime<Utc>,
    },
    PaymentProcessing {
        payment_id: Uuid,
        provider_payment_id: Option<String>,
        processing_at: DateTime<Utc>,
    },
    PaymentCompleted {
        payment_id: Uuid,
        provider_payment_id: String,
        transaction_hash: Option<String>,
        completed_at: DateTime<Utc>,
    },
    PaymentFailed {
        payment_id: Uuid,
        failure_reason: String,
        failed_at: DateTime<Utc>,
    },
    PaymentCancelled {
        payment_id: Uuid,
        reason: String,
        cancelled_at: DateTime<Utc>,
    },
    RefundCreated {
        refund_id: Uuid,
        payment_id: Uuid,
        amount: Decimal,
        reason: String,
        created_at: DateTime<Utc>,
    },
    RefundCompleted {
        refund_id: Uuid,
        provider_refund_id: String,
        completed_at: DateTime<Utc>,
    },
    RefundFailed {
        refund_id: Uuid,
        failure_reason: String,
        failed_at: DateTime<Utc>,
    },
}

impl PaymentEvent {
    pub fn payment_id(&self) -> Uuid {
        match self {
            PaymentEvent::PaymentCreated { payment_id, .. } => *payment_id,
            PaymentEvent::PaymentProcessing { payment_id, .. } => *payment_id,
            PaymentEvent::PaymentCompleted { payment_id, .. } => *payment_id,
            PaymentEvent::PaymentFailed { payment_id, .. } => *payment_id,
            PaymentEvent::PaymentCancelled { payment_id, .. } => *payment_id,
            PaymentEvent::RefundCreated { payment_id, .. } => *payment_id,
            PaymentEvent::RefundCompleted { .. } => Uuid::nil(), // Refund events don't have payment_id directly
            PaymentEvent::RefundFailed { .. } => Uuid::nil(),
        }
    }

    pub fn event_type(&self) -> &'static str {
        match self {
            PaymentEvent::PaymentCreated { .. } => "payment.created",
            PaymentEvent::PaymentProcessing { .. } => "payment.processing",
            PaymentEvent::PaymentCompleted { .. } => "payment.completed",
            PaymentEvent::PaymentFailed { .. } => "payment.failed",
            PaymentEvent::PaymentCancelled { .. } => "payment.cancelled",
            PaymentEvent::RefundCreated { .. } => "refund.created",
            PaymentEvent::RefundCompleted { .. } => "refund.completed",
            PaymentEvent::RefundFailed { .. } => "refund.failed",
        }
    }
}

pub struct PaymentEventPublisher {
    // TODO: Add event publisher implementation
}

impl PaymentEventPublisher {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn publish(&self, event: PaymentEvent) -> anyhow::Result<()> {
        // TODO: Implement event publishing logic
        tracing::info!("Publishing payment event: {:?}", event);
        Ok(())
    }
} 