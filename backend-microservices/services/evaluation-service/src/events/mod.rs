use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationSubmittedEvent {
    pub evaluation_id: Uuid,
    pub enterprise_id: Uuid,
    pub evaluator_id: Uuid,
    pub overall_score: f64,
    pub calculated_multiplier: f64,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplierUpdatedEvent {
    pub enterprise_id: Uuid,
    pub old_multiplier: f64,
    pub new_multiplier: f64,
    pub updated_by: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReportGeneratedEvent {
    pub enterprise_id: Uuid,
    pub report_id: Uuid,
    pub average_score: f64,
    pub total_evaluations: u32,
    pub generated_at: DateTime<Utc>,
} 