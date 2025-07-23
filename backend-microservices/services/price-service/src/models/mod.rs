use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRecord {
    pub id: Uuid,
    pub product_id: Uuid,
    pub price: f64,
    pub currency: String,
    pub market_source: String,
    pub recorded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PriceUpdateRequest {
    pub product_id: Uuid,
    #[validate(range(min = 0.01))]
    pub price: f64,
    #[validate(length(min = 1, max = 10))]
    pub currency: String,
    #[validate(length(min = 1, max = 50))]
    pub market_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceAnalysis {
    pub product_id: Uuid,
    pub current_price: f64,
    pub price_change_24h: f64,
    pub price_change_7d: f64,
    pub volatility_score: f64,
    pub manipulation_risk: ManipulationRisk,
    pub trend_direction: TrendDirection,
    pub analyzed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceAlert {
    pub id: Uuid,
    pub product_id: Uuid,
    pub alert_type: AlertType,
    pub threshold: f64,
    pub current_value: f64,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: DateTime<Utc>,
    pub is_resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManipulationRisk {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Upward,
    Downward,
    Stable,
    Volatile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    PriceSpike,
    PriceDrop,
    VolatilityHigh,
    ManipulationDetected,
    TrendChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTrend {
    pub period: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub average_price: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub price_change_percent: f64,
    pub volume_trend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    pub product_id: Uuid,
    pub records: Vec<PriceRecord>,
    pub period: String,
    pub total_records: usize,
}

impl From<PriceRecord> for serde_json::Value {
    fn from(record: PriceRecord) -> Self {
        serde_json::json!({
            "id": record.id,
            "product_id": record.product_id,
            "price": record.price,
            "currency": record.currency,
            "market_source": record.market_source,
            "recorded_at": record.recorded_at,
            "created_at": record.created_at,
        })
    }
} 