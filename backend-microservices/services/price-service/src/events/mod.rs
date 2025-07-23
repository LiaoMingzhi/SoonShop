use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdatedEvent {
    pub product_id: Uuid,
    pub old_price: f64,
    pub new_price: f64,
    pub currency: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceAnomalyDetectedEvent {
    pub product_id: Uuid,
    pub anomaly_type: String,
    pub severity: String,
    pub current_price: f64,
    pub threshold: f64,
    pub detected_at: DateTime<Utc>,
} 