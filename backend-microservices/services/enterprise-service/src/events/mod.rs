use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseCreatedEvent {
    pub enterprise_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseUpdatedEvent {
    pub enterprise_id: Uuid,
    pub name: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseMultiplierUpdatedEvent {
    pub enterprise_id: Uuid,
    pub old_multiplier: f64,
    pub new_multiplier: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseDeactivatedEvent {
    pub enterprise_id: Uuid,
    pub deactivated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseActivatedEvent {
    pub enterprise_id: Uuid,
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseRewardsEarnedEvent {
    pub enterprise_id: Uuid,
    pub amount: u64,
    pub total_rewards: u64,
    pub earned_at: DateTime<Utc>,
} 