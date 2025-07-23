use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enterprise {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub current_multiplier: f64,
    pub total_evaluations: u32,
    pub average_score: f64,
    pub last_evaluation_at: Option<DateTime<Utc>>,
    pub total_rewards_earned: u64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EnterpriseCreateRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub owner_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EnterpriseUpdateRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub current_multiplier: f64,
    pub total_evaluations: u32,
    pub average_score: f64,
    pub last_evaluation_at: Option<DateTime<Utc>>,
    pub total_rewards_earned: u64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseStats {
    pub total_enterprises: u64,
    pub active_enterprises: u64,
    pub average_multiplier: f64,
    pub total_rewards_distributed: u64,
    pub evaluations_this_month: u64,
}

impl From<Enterprise> for EnterpriseResponse {
    fn from(enterprise: Enterprise) -> Self {
        EnterpriseResponse {
            id: enterprise.id,
            owner_id: enterprise.owner_id,
            name: enterprise.name,
            description: enterprise.description,
            current_multiplier: enterprise.current_multiplier,
            total_evaluations: enterprise.total_evaluations,
            average_score: enterprise.average_score,
            last_evaluation_at: enterprise.last_evaluation_at,
            total_rewards_earned: enterprise.total_rewards_earned,
            is_active: enterprise.is_active,
            created_at: enterprise.created_at,
            updated_at: enterprise.updated_at,
        }
    }
} 