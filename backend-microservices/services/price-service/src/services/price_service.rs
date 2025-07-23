use crate::models::*;
use crate::repositories::price_repository::PriceRepository;
use anyhow::Result;
use chrono::{Utc, Duration};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[derive(Clone)]
pub struct PriceService {
    repository: PriceRepository,
}

impl PriceService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repository: PriceRepository::new(db),
        }
    }

    pub async fn update_price(&self, request: PriceUpdateRequest) -> Result<PriceRecord> {
        let now = Utc::now();
        let record = PriceRecord {
            id: Uuid::new_v4(),
            product_id: request.product_id,
            price: request.price,
            currency: request.currency,
            market_source: request.market_source,
            recorded_at: now,
            created_at: now,
        };

        let created_record = self.repository.create(record).await?;

        // 检测价格异常
        self.detect_price_anomalies(request.product_id).await?;

        Ok(created_record)
    }

    pub async fn get_price_history(&self, product_id: Uuid, period: &str) -> Result<PriceHistory> {
        let records = self.repository.find_by_product_id(product_id).await?;
        
        Ok(PriceHistory {
            product_id,
            records: records.clone(),
            period: period.to_string(),
            total_records: records.len(),
        })
    }

    pub async fn analyze_price_trends(&self, product_id: Uuid) -> Result<PriceAnalysis> {
        let records = self.repository.find_by_product_id(product_id).await?;
        
        if records.is_empty() {
            return Err(anyhow::anyhow!("No price records found for product"));
        }

        let current_price = records.last().unwrap().price;
        
        // 计算24小时价格变化
        let price_change_24h = self.calculate_price_change(&records, 24)?;
        
        // 计算7天价格变化
        let price_change_7d = self.calculate_price_change(&records, 24 * 7)?;

        // 计算波动性得分
        let volatility_score = self.calculate_volatility(&records)?;

        // 评估操纵风险
        let manipulation_risk = self.assess_manipulation_risk(&records)?;

        // 确定趋势方向
        let trend_direction = self.determine_trend_direction(&records)?;

        Ok(PriceAnalysis {
            product_id,
            current_price,
            price_change_24h,
            price_change_7d,
            volatility_score,
            manipulation_risk,
            trend_direction,
            analyzed_at: Utc::now(),
        })
    }

    pub async fn detect_price_anomalies(&self, product_id: Uuid) -> Result<Vec<PriceAlert>> {
        let records = self.repository.find_by_product_id(product_id).await?;
        let mut alerts = Vec::new();

        if records.len() < 2 {
            return Ok(alerts);
        }

        let current_price = records.last().unwrap().price;
        let previous_price = records[records.len() - 2].price;
        let price_change_percent = ((current_price - previous_price) / previous_price) * 100.0;

        // 价格剧烈上涨检测
        if price_change_percent > 50.0 {
            alerts.push(PriceAlert {
                id: Uuid::new_v4(),
                product_id,
                alert_type: AlertType::PriceSpike,
                threshold: 50.0,
                current_value: price_change_percent,
                severity: AlertSeverity::Critical,
                message: format!("价格在短时间内上涨了{:.2}%", price_change_percent),
                triggered_at: Utc::now(),
                is_resolved: false,
            });
        }

        // 价格剧烈下跌检测
        if price_change_percent < -50.0 {
            alerts.push(PriceAlert {
                id: Uuid::new_v4(),
                product_id,
                alert_type: AlertType::PriceDrop,
                threshold: -50.0,
                current_value: price_change_percent,
                severity: AlertSeverity::Critical,
                message: format!("价格在短时间内下跌了{:.2}%", price_change_percent.abs()),
                triggered_at: Utc::now(),
                is_resolved: false,
            });
        }

        // 高波动性检测
        let volatility = self.calculate_volatility(&records)?;
        if volatility > 0.8 {
            alerts.push(PriceAlert {
                id: Uuid::new_v4(),
                product_id,
                alert_type: AlertType::VolatilityHigh,
                threshold: 0.8,
                current_value: volatility,
                severity: AlertSeverity::Warning,
                message: format!("价格波动性过高，波动性得分: {:.2}", volatility),
                triggered_at: Utc::now(),
                is_resolved: false,
            });
        }

        Ok(alerts)
    }

    pub async fn get_market_trends(&self, period: &str) -> Result<Vec<MarketTrend>> {
        // 在实际实现中，这里会分析多个产品的市场趋势
        // 现在返回模拟数据
        Ok(vec![
            MarketTrend {
                period: period.to_string(),
                start_date: Utc::now() - Duration::days(30),
                end_date: Utc::now(),
                average_price: 100.0,
                min_price: 80.0,
                max_price: 120.0,
                price_change_percent: 5.0,
                volume_trend: "增长".to_string(),
            }
        ])
    }

    fn calculate_price_change(&self, records: &[PriceRecord], hours_back: i64) -> Result<f64> {
        let cutoff_time = Utc::now() - Duration::hours(hours_back);
        let recent_records: Vec<&PriceRecord> = records.iter()
            .filter(|r| r.recorded_at >= cutoff_time)
            .collect();

        if recent_records.len() < 2 {
            return Ok(0.0);
        }

        let oldest_price = recent_records.first().unwrap().price;
        let newest_price = recent_records.last().unwrap().price;
        
        Ok(((newest_price - oldest_price) / oldest_price) * 100.0)
    }

    fn calculate_volatility(&self, records: &[PriceRecord]) -> Result<f64> {
        if records.len() < 2 {
            return Ok(0.0);
        }

        let prices: Vec<f64> = records.iter().map(|r| r.price).collect();
        let mean = prices.iter().sum::<f64>() / prices.len() as f64;
        
        let variance = prices.iter()
            .map(|p| (p - mean).powi(2))
            .sum::<f64>() / prices.len() as f64;
        
        let std_dev = variance.sqrt();
        
        // 归一化波动性得分 (0-1)
        Ok((std_dev / mean).min(1.0))
    }

    fn assess_manipulation_risk(&self, records: &[PriceRecord]) -> Result<ManipulationRisk> {
        let volatility = self.calculate_volatility(records)?;
        
        if volatility > 0.5 {
            Ok(ManipulationRisk::High)
        } else if volatility > 0.3 {
            Ok(ManipulationRisk::Medium)
        } else {
            Ok(ManipulationRisk::Low)
        }
    }

    fn determine_trend_direction(&self, records: &[PriceRecord]) -> Result<TrendDirection> {
        if records.len() < 3 {
            return Ok(TrendDirection::Stable);
        }

        let recent_prices: Vec<f64> = records.iter()
            .rev()
            .take(10)
            .map(|r| r.price)
            .collect();

        let trend_score = self.calculate_trend_score(&recent_prices);
        
        if trend_score > 0.1 {
            Ok(TrendDirection::Upward)
        } else if trend_score < -0.1 {
            Ok(TrendDirection::Downward)
        } else {
            let volatility = self.calculate_volatility(records)?;
            if volatility > 0.4 {
                Ok(TrendDirection::Volatile)
            } else {
                Ok(TrendDirection::Stable)
            }
        }
    }

    fn calculate_trend_score(&self, prices: &[f64]) -> f64 {
        if prices.len() < 2 {
            return 0.0;
        }

        let first_price = prices.first().unwrap();
        let last_price = prices.last().unwrap();
        
        (last_price - first_price) / first_price
    }
} 