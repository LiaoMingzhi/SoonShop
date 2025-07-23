use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evaluation {
    pub id: Uuid,
    pub enterprise_id: Uuid,
    pub evaluator_id: Uuid,
    pub production_capacity_score: u8,    // 生产能力评分 (0-100)
    pub product_quality_score: u8,        // 产品质量评分 (0-100)
    pub service_level_score: u8,          // 服务水平评分 (0-100)
    pub social_responsibility_score: u8,  // 社会责任评分 (0-100)
    pub innovation_score: u8,             // 创新能力评分 (0-100)
    pub overall_score: f64,               // 综合评分
    pub calculated_multiplier: f64,       // 计算出的倍增系数
    pub comments: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EvaluationCreateRequest {
    pub enterprise_id: Uuid,
    pub evaluator_id: Uuid,
    
    #[validate(range(min = 0, max = 100))]
    pub production_capacity_score: u8,
    
    #[validate(range(min = 0, max = 100))]
    pub product_quality_score: u8,
    
    #[validate(range(min = 0, max = 100))]
    pub service_level_score: u8,
    
    #[validate(range(min = 0, max = 100))]
    pub social_responsibility_score: u8,
    
    #[validate(range(min = 0, max = 100))]
    pub innovation_score: u8,
    
    #[validate(length(max = 1000))]
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResponse {
    pub id: Uuid,
    pub enterprise_id: Uuid,
    pub evaluator_id: Uuid,
    pub production_capacity_score: u8,
    pub product_quality_score: u8,
    pub service_level_score: u8,
    pub social_responsibility_score: u8,
    pub innovation_score: u8,
    pub overall_score: f64,
    pub calculated_multiplier: f64,
    pub comments: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReport {
    pub enterprise_id: Uuid,
    pub enterprise_name: String,
    pub total_evaluations: u32,
    pub average_overall_score: f64,
    pub current_multiplier: f64,
    pub score_breakdown: ScoreBreakdown,
    pub trend_analysis: TrendAnalysis,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub production_capacity_avg: f64,
    pub product_quality_avg: f64,
    pub service_level_avg: f64,
    pub social_responsibility_avg: f64,
    pub innovation_avg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub score_trend: String,     // "improving", "stable", "declining"
    pub trend_percentage: f64,   // 变化百分比
    pub period_days: u32,        // 分析周期天数
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplierUpdateRequest {
    pub enterprise_id: Uuid,
    pub new_multiplier: f64,
}

impl From<Evaluation> for EvaluationResponse {
    fn from(evaluation: Evaluation) -> Self {
        EvaluationResponse {
            id: evaluation.id,
            enterprise_id: evaluation.enterprise_id,
            evaluator_id: evaluation.evaluator_id,
            production_capacity_score: evaluation.production_capacity_score,
            product_quality_score: evaluation.product_quality_score,
            service_level_score: evaluation.service_level_score,
            social_responsibility_score: evaluation.social_responsibility_score,
            innovation_score: evaluation.innovation_score,
            overall_score: evaluation.overall_score,
            calculated_multiplier: evaluation.calculated_multiplier,
            comments: evaluation.comments,
            created_at: evaluation.created_at,
            updated_at: evaluation.updated_at,
        }
    }
}

// 计算综合评分的权重
pub const WEIGHTS: [f64; 5] = [0.25, 0.25, 0.20, 0.15, 0.15]; // 对应生产能力、产品质量、服务水平、社会责任、创新能力

// 计算倍增系数
pub fn calculate_multiplier(overall_score: f64) -> f64 {
    let base_multiplier = 2.0;
    let max_bonus = 3.0;
    
    if overall_score >= 60.0 {
        let bonus = (overall_score - 60.0) / 40.0 * max_bonus;
        (base_multiplier + bonus).min(5.0)
    } else {
        // 评分低于60分时，倍增系数降低
        let penalty = (60.0 - overall_score) / 60.0;
        (base_multiplier - penalty).max(1.0)
    }
} 