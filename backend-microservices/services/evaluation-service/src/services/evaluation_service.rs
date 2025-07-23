use crate::models::{Evaluation, EvaluationCreateRequest, EvaluationReport, MultiplierUpdateRequest, calculate_multiplier, WEIGHTS};
use crate::repositories::evaluation_repository::EvaluationRepository;
use anyhow::Result;
use chrono::Utc;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[derive(Clone)]
pub struct EvaluationService {
    repository: EvaluationRepository,
}

impl EvaluationService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repository: EvaluationRepository::new(db),
        }
    }

    pub async fn submit_evaluation(&self, request: EvaluationCreateRequest) -> Result<Evaluation> {
        let now = Utc::now();
        
        // 计算综合评分
        let overall_score = self.calculate_overall_score(&request);
        
        // 计算倍增系数
        let calculated_multiplier = calculate_multiplier(overall_score);
        
        let evaluation = Evaluation {
            id: Uuid::new_v4(),
            enterprise_id: request.enterprise_id,
            evaluator_id: request.evaluator_id,
            production_capacity_score: request.production_capacity_score,
            product_quality_score: request.product_quality_score,
            service_level_score: request.service_level_score,
            social_responsibility_score: request.social_responsibility_score,
            innovation_score: request.innovation_score,
            overall_score,
            calculated_multiplier,
            comments: request.comments,
            created_at: now,
            updated_at: now,
        };

        let created_evaluation = self.repository.create(evaluation).await?;

        // 更新企业的倍增系数
        self.update_enterprise_multiplier(request.enterprise_id).await?;

        Ok(created_evaluation)
    }

    pub async fn get_evaluations_by_enterprise(&self, enterprise_id: Uuid) -> Result<Vec<Evaluation>> {
        self.repository.find_by_enterprise_id(enterprise_id).await
    }

    pub async fn get_evaluation(&self, id: Uuid) -> Result<Option<Evaluation>> {
        self.repository.find_by_id(id).await
    }

    pub async fn generate_evaluation_report(&self, enterprise_id: Uuid) -> Result<EvaluationReport> {
        let evaluations = self.repository.find_by_enterprise_id(enterprise_id).await?;
        
        if evaluations.is_empty() {
            return Err(anyhow::anyhow!("No evaluations found for enterprise"));
        }

        let total_evaluations = evaluations.len() as u32;
        let average_overall_score = evaluations.iter()
            .map(|e| e.overall_score)
            .sum::<f64>() / total_evaluations as f64;

        let current_multiplier = calculate_multiplier(average_overall_score);

        // 计算各项评分的平均值
        let production_capacity_avg = evaluations.iter()
            .map(|e| e.production_capacity_score as f64)
            .sum::<f64>() / total_evaluations as f64;
        
        let product_quality_avg = evaluations.iter()
            .map(|e| e.product_quality_score as f64)
            .sum::<f64>() / total_evaluations as f64;
        
        let service_level_avg = evaluations.iter()
            .map(|e| e.service_level_score as f64)
            .sum::<f64>() / total_evaluations as f64;
        
        let social_responsibility_avg = evaluations.iter()
            .map(|e| e.social_responsibility_score as f64)
            .sum::<f64>() / total_evaluations as f64;
        
        let innovation_avg = evaluations.iter()
            .map(|e| e.innovation_score as f64)
            .sum::<f64>() / total_evaluations as f64;

        let score_breakdown = crate::models::ScoreBreakdown {
            production_capacity_avg,
            product_quality_avg,
            service_level_avg,
            social_responsibility_avg,
            innovation_avg,
        };

        // 趋势分析
        let trend_analysis = self.analyze_score_trend(&evaluations);

        // 生成建议
        let recommendations = self.generate_recommendations(&score_breakdown);

        Ok(EvaluationReport {
            enterprise_id,
            enterprise_name: "未知企业".to_string(), // 在实际实现中需要从企业服务获取
            total_evaluations,
            average_overall_score,
            current_multiplier,
            score_breakdown,
            trend_analysis,
            recommendations,
            generated_at: Utc::now(),
        })
    }

    pub async fn update_enterprise_multiplier(&self, enterprise_id: Uuid) -> Result<()> {
        let evaluations = self.repository.find_by_enterprise_id(enterprise_id).await?;
        
        if evaluations.is_empty() {
            return Ok(());
        }

        let average_score = evaluations.iter()
            .map(|e| e.overall_score)
            .sum::<f64>() / evaluations.len() as f64;

        let new_multiplier = calculate_multiplier(average_score);

        // 在实际实现中，这里需要调用企业服务的API来更新倍增系数
        // 示例：enterprise_service.update_multiplier(enterprise_id, new_multiplier).await?;
        
        Ok(())
    }

    fn calculate_overall_score(&self, request: &EvaluationCreateRequest) -> f64 {
        let scores = [
            request.production_capacity_score as f64,
            request.product_quality_score as f64,
            request.service_level_score as f64,
            request.social_responsibility_score as f64,
            request.innovation_score as f64,
        ];

        scores.iter()
            .zip(WEIGHTS.iter())
            .map(|(score, weight)| score * weight)
            .sum()
    }

    fn analyze_score_trend(&self, evaluations: &[Evaluation]) -> crate::models::TrendAnalysis {
        if evaluations.len() < 2 {
            return crate::models::TrendAnalysis {
                score_trend: "stable".to_string(),
                trend_percentage: 0.0,
                period_days: 0,
            };
        }

        let mut sorted_evaluations = evaluations.to_vec();
        sorted_evaluations.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        let first_score = sorted_evaluations.first().unwrap().overall_score;
        let last_score = sorted_evaluations.last().unwrap().overall_score;
        
        let trend_percentage = ((last_score - first_score) / first_score) * 100.0;
        
        let score_trend = if trend_percentage > 5.0 {
            "improving"
        } else if trend_percentage < -5.0 {
            "declining"
        } else {
            "stable"
        };

        let period_days = (sorted_evaluations.last().unwrap().created_at - sorted_evaluations.first().unwrap().created_at).num_days() as u32;

        crate::models::TrendAnalysis {
            score_trend: score_trend.to_string(),
            trend_percentage,
            period_days,
        }
    }

    fn generate_recommendations(&self, breakdown: &crate::models::ScoreBreakdown) -> Vec<String> {
        let mut recommendations = Vec::new();

        if breakdown.production_capacity_avg < 70.0 {
            recommendations.push("建议改善生产能力，提高生产效率和产能".to_string());
        }

        if breakdown.product_quality_avg < 70.0 {
            recommendations.push("建议加强产品质量管理，提升产品品质".to_string());
        }

        if breakdown.service_level_avg < 70.0 {
            recommendations.push("建议提升服务水平，改善客户体验".to_string());
        }

        if breakdown.social_responsibility_avg < 70.0 {
            recommendations.push("建议加强社会责任履行，提升企业形象".to_string());
        }

        if breakdown.innovation_avg < 70.0 {
            recommendations.push("建议增强创新能力，推进技术和管理创新".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("企业各项指标表现良好，继续保持现有水平".to_string());
        }

        recommendations
    }
} 