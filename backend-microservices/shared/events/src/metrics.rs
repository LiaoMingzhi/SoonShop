// 事件系统指标收集器
// 文件路径: /d:/workspace/Solana/SoonShop/backend-microservices/shared/events/src/metrics.rs

use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::retry::ErrorType;

/// 事件指标数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetricData {
    /// 指标名称
    pub name: String,
    /// 指标值
    pub value: f64,
    /// 标签
    pub labels: HashMap<String, String>,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 指标类型
    pub metric_type: MetricType,
}

/// 指标类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    /// 计数器（累计值）
    Counter,
    /// 测量值（瞬时值）
    Gauge,
    /// 直方图
    Histogram,
    /// 摘要
    Summary,
}

/// 事件统计信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventStats {
    /// 成功发布次数
    pub successful_publishes: u64,
    /// 失败发布次数
    pub failed_publishes: u64,
    /// 重试次数
    pub retry_attempts: u64,
    /// 死信队列消息数
    pub dead_letter_messages: u64,
    /// 平均处理时间（毫秒）
    pub avg_processing_time: f64,
    /// 错误类型统计
    pub error_type_counts: HashMap<ErrorType, u64>,
    /// 最后更新时间
    pub last_updated: DateTime<Utc>,
}

/// 事件指标收集器特征
#[async_trait]
pub trait EventMetrics: Send + Sync {
    /// 记录成功发布事件
    async fn record_successful_publish(&self, exchange: &str, routing_key: &str);
    
    /// 记录失败发布事件
    async fn record_failed_publish(&self, exchange: &str, routing_key: &str, error_type: &ErrorType);
    
    /// 记录重试尝试
    async fn record_retry_attempt(&self, exchange: &str, routing_key: &str, attempt: u32);
    
    /// 记录死信队列事件
    async fn record_dead_letter_event(&self, exchange: &str, routing_key: &str);
    
    /// 记录处理时间
    async fn record_processing_time(&self, exchange: &str, routing_key: &str, duration_ms: f64);
    
    /// 获取指标数据
    async fn get_metrics(&self) -> Vec<EventMetricData>;
    
    /// 获取统计信息
    async fn get_stats(&self, exchange: Option<&str>) -> EventStats;
    
    /// 重置指标
    async fn reset_metrics(&self);
}

/// 内存事件指标收集器
pub struct InMemoryEventMetrics {
    /// 指标数据存储
    metrics: Arc<RwLock<HashMap<String, EventMetricData>>>,
    /// 统计数据存储
    stats: Arc<RwLock<HashMap<String, EventStats>>>,
}

impl InMemoryEventMetrics {
    /// 创建新的内存指标收集器
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 生成指标键
    fn generate_metric_key(&self, exchange: &str, routing_key: &str, metric_name: &str) -> String {
        format!("{}:{}:{}", exchange, routing_key, metric_name)
    }

    /// 更新计数器指标
    async fn increment_counter(&self, key: String, labels: HashMap<String, String>) {
        let mut metrics = self.metrics.write().await;
        
        match metrics.get_mut(&key) {
            Some(metric) => {
                metric.value += 1.0;
                metric.timestamp = Utc::now();
            }
            None => {
                metrics.insert(
                    key.clone(),
                    EventMetricData {
                        name: key,
                        value: 1.0,
                        labels,
                        timestamp: Utc::now(),
                        metric_type: MetricType::Counter,
                    },
                );
            }
        }
    }

    /// 更新测量值指标
    async fn set_gauge(&self, key: String, value: f64, labels: HashMap<String, String>) {
        let mut metrics = self.metrics.write().await;
        
        metrics.insert(
            key.clone(),
            EventMetricData {
                name: key,
                value,
                labels,
                timestamp: Utc::now(),
                metric_type: MetricType::Gauge,
            },
        );
    }

    /// 获取或创建统计信息
    async fn get_or_create_stats(&self, exchange: &str) -> EventStats {
        let mut stats = self.stats.write().await;
        stats.entry(exchange.to_string())
            .or_insert_with(EventStats::default)
            .clone()
    }

    /// 更新统计信息
    async fn update_stats<F>(&self, exchange: &str, update_fn: F)
    where
        F: FnOnce(&mut EventStats),
    {
        let mut stats = self.stats.write().await;
        let entry = stats.entry(exchange.to_string()).or_insert_with(EventStats::default);
        update_fn(entry);
        entry.last_updated = Utc::now();
    }
}

#[async_trait]
impl EventMetrics for InMemoryEventMetrics {
    async fn record_successful_publish(&self, exchange: &str, routing_key: &str) {
        // 更新计数器指标
        let key = self.generate_metric_key(exchange, routing_key, "successful_publishes");
        let labels = HashMap::from([
            ("exchange".to_string(), exchange.to_string()),
            ("routing_key".to_string(), routing_key.to_string()),
        ]);
        
        self.increment_counter(key, labels).await;
        
        // 更新统计信息
        self.update_stats(exchange, |stats| {
            stats.successful_publishes += 1;
        }).await;
    }

    async fn record_failed_publish(&self, exchange: &str, routing_key: &str, error_type: &ErrorType) {
        // 更新计数器指标
        let key = self.generate_metric_key(exchange, routing_key, "failed_publishes");
        let labels = HashMap::from([
            ("exchange".to_string(), exchange.to_string()),
            ("routing_key".to_string(), routing_key.to_string()),
            ("error_type".to_string(), format!("{:?}", error_type)),
        ]);
        
        self.increment_counter(key, labels).await;
        
        // 更新统计信息
        self.update_stats(exchange, |stats| {
            stats.failed_publishes += 1;
            *stats.error_type_counts.entry(error_type.clone()).or_insert(0) += 1;
        }).await;
    }

    async fn record_retry_attempt(&self, exchange: &str, routing_key: &str, attempt: u32) {
        // 更新计数器指标
        let key = self.generate_metric_key(exchange, routing_key, "retry_attempts");
        let labels = HashMap::from([
            ("exchange".to_string(), exchange.to_string()),
            ("routing_key".to_string(), routing_key.to_string()),
            ("attempt".to_string(), attempt.to_string()),
        ]);
        
        self.increment_counter(key, labels).await;
        
        // 更新统计信息
        self.update_stats(exchange, |stats| {
            stats.retry_attempts += 1;
        }).await;
    }

    async fn record_dead_letter_event(&self, exchange: &str, routing_key: &str) {
        // 更新计数器指标
        let key = self.generate_metric_key(exchange, routing_key, "dead_letter_messages");
        let labels = HashMap::from([
            ("exchange".to_string(), exchange.to_string()),
            ("routing_key".to_string(), routing_key.to_string()),
        ]);
        
        self.increment_counter(key, labels).await;
        
        // 更新统计信息
        self.update_stats(exchange, |stats| {
            stats.dead_letter_messages += 1;
        }).await;
    }

    async fn record_processing_time(&self, exchange: &str, routing_key: &str, duration_ms: f64) {
        // 更新测量值指标
        let key = self.generate_metric_key(exchange, routing_key, "processing_time");
        let labels = HashMap::from([
            ("exchange".to_string(), exchange.to_string()),
            ("routing_key".to_string(), routing_key.to_string()),
        ]);
        
        self.set_gauge(key, duration_ms, labels).await;
        
        // 更新统计信息（简单平均）
        self.update_stats(exchange, |stats| {
            let total_events = stats.successful_publishes + stats.failed_publishes;
            if total_events > 0 {
                stats.avg_processing_time = 
                    (stats.avg_processing_time * (total_events - 1) as f64 + duration_ms) / total_events as f64;
            } else {
                stats.avg_processing_time = duration_ms;
            }
        }).await;
    }

    async fn get_metrics(&self) -> Vec<EventMetricData> {
        let metrics = self.metrics.read().await;
        metrics.values().cloned().collect()
    }

    async fn get_stats(&self, exchange: Option<&str>) -> EventStats {
        let stats = self.stats.read().await;
        
        match exchange {
            Some(exchange_name) => {
                stats.get(exchange_name).cloned().unwrap_or_default()
            }
            None => {
                // 聚合所有交换机的统计
                let mut aggregated = EventStats::default();
                for stat in stats.values() {
                    aggregated.successful_publishes += stat.successful_publishes;
                    aggregated.failed_publishes += stat.failed_publishes;
                    aggregated.retry_attempts += stat.retry_attempts;
                    aggregated.dead_letter_messages += stat.dead_letter_messages;
                    
                    // 合并错误类型统计
                    for (error_type, count) in &stat.error_type_counts {
                        *aggregated.error_type_counts.entry(error_type.clone()).or_insert(0) += count;
                    }
                    
                    // 更新最后更新时间
                    if stat.last_updated > aggregated.last_updated {
                        aggregated.last_updated = stat.last_updated;
                    }
                }
                
                // 计算平均处理时间
                let total_stats: Vec<_> = stats.values().collect();
                if !total_stats.is_empty() {
                    aggregated.avg_processing_time = total_stats.iter()
                        .map(|s| s.avg_processing_time)
                        .sum::<f64>() / total_stats.len() as f64;
                }
                
                aggregated
            }
        }
    }

    async fn reset_metrics(&self) {
        let mut metrics = self.metrics.write().await;
        let mut stats = self.stats.write().await;
        
        metrics.clear();
        stats.clear();
    }
}

/// Prometheus指标收集器
#[cfg(feature = "prometheus")]
pub struct PrometheusEventMetrics {
    /// 成功发布计数器
    successful_publishes: prometheus::CounterVec,
    /// 失败发布计数器
    failed_publishes: prometheus::CounterVec,
    /// 重试计数器
    retry_attempts: prometheus::CounterVec,
    /// 死信队列计数器
    dead_letter_messages: prometheus::CounterVec,
    /// 处理时间直方图
    processing_time: prometheus::HistogramVec,
}

#[cfg(feature = "prometheus")]
impl PrometheusEventMetrics {
    /// 创建新的Prometheus指标收集器
    pub fn new(namespace: &str) -> anyhow::Result<Self> {
        use prometheus::{CounterVec, HistogramVec, Opts, HistogramOpts};
        
        let successful_publishes = CounterVec::new(
            Opts::new(
                format!("{}_successful_publishes_total", namespace),
                "Total number of successful event publishes"
            ),
            &["exchange", "routing_key"]
        )?;
        
        let failed_publishes = CounterVec::new(
            Opts::new(
                format!("{}_failed_publishes_total", namespace),
                "Total number of failed event publishes"
            ),
            &["exchange", "routing_key", "error_type"]
        )?;
        
        let retry_attempts = CounterVec::new(
            Opts::new(
                format!("{}_retry_attempts_total", namespace),
                "Total number of retry attempts"
            ),
            &["exchange", "routing_key", "attempt"]
        )?;
        
        let dead_letter_messages = CounterVec::new(
            Opts::new(
                format!("{}_dead_letter_messages_total", namespace),
                "Total number of dead letter messages"
            ),
            &["exchange", "routing_key"]
        )?;
        
        let processing_time = HistogramVec::new(
            HistogramOpts::new(
                format!("{}_processing_time_seconds", namespace),
                "Event processing time in seconds"
            ),
            &["exchange", "routing_key"]
        )?;
        
        // 注册指标
        prometheus::register(Box::new(successful_publishes.clone()))?;
        prometheus::register(Box::new(failed_publishes.clone()))?;
        prometheus::register(Box::new(retry_attempts.clone()))?;
        prometheus::register(Box::new(dead_letter_messages.clone()))?;
        prometheus::register(Box::new(processing_time.clone()))?;
        
        Ok(Self {
            successful_publishes,
            failed_publishes,
            retry_attempts,
            dead_letter_messages,
            processing_time,
        })
    }
}

#[cfg(feature = "prometheus")]
#[async_trait]
impl EventMetrics for PrometheusEventMetrics {
    async fn record_successful_publish(&self, exchange: &str, routing_key: &str) {
        self.successful_publishes
            .with_label_values(&[exchange, routing_key])
            .inc();
    }

    async fn record_failed_publish(&self, exchange: &str, routing_key: &str, error_type: &ErrorType) {
        self.failed_publishes
            .with_label_values(&[exchange, routing_key, &format!("{:?}", error_type)])
            .inc();
    }

    async fn record_retry_attempt(&self, exchange: &str, routing_key: &str, attempt: u32) {
        self.retry_attempts
            .with_label_values(&[exchange, routing_key, &attempt.to_string()])
            .inc();
    }

    async fn record_dead_letter_event(&self, exchange: &str, routing_key: &str) {
        self.dead_letter_messages
            .with_label_values(&[exchange, routing_key])
            .inc();
    }

    async fn record_processing_time(&self, exchange: &str, routing_key: &str, duration_ms: f64) {
        self.processing_time
            .with_label_values(&[exchange, routing_key])
            .observe(duration_ms / 1000.0); // 转换为秒
    }

    async fn get_metrics(&self) -> Vec<EventMetricData> {
        // Prometheus指标通过/metrics端点暴露，这里返回空向量
        Vec::new()
    }

    async fn get_stats(&self, _exchange: Option<&str>) -> EventStats {
        // Prometheus指标通过查询API获取，这里返回默认值
        EventStats::default()
    }

    async fn reset_metrics(&self) {
        // Prometheus指标通常不需要重置
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_in_memory_metrics() {
        let metrics = InMemoryEventMetrics::new();
        
        // 记录一些事件
        metrics.record_successful_publish("test_exchange", "test.routing.key").await;
        metrics.record_failed_publish("test_exchange", "test.routing.key", &ErrorType::Network).await;
        metrics.record_retry_attempt("test_exchange", "test.routing.key", 1).await;
        
        // 获取统计信息
        let stats = metrics.get_stats(Some("test_exchange")).await;
        assert_eq!(stats.successful_publishes, 1);
        assert_eq!(stats.failed_publishes, 1);
        assert_eq!(stats.retry_attempts, 1);
        
        // 获取指标数据
        let metric_data = metrics.get_metrics().await;
        assert!(!metric_data.is_empty());
    }

    #[tokio::test]
    async fn test_aggregated_stats() {
        let metrics = InMemoryEventMetrics::new();
        
        // 记录多个交换机的事件
        metrics.record_successful_publish("exchange1", "key1").await;
        metrics.record_successful_publish("exchange2", "key2").await;
        metrics.record_failed_publish("exchange1", "key1", &ErrorType::Network).await;
        
        // 获取聚合统计
        let aggregated_stats = metrics.get_stats(None).await;
        assert_eq!(aggregated_stats.successful_publishes, 2);
        assert_eq!(aggregated_stats.failed_publishes, 1);
        
        // 获取单个交换机统计
        let exchange1_stats = metrics.get_stats(Some("exchange1")).await;
        assert_eq!(exchange1_stats.successful_publishes, 1);
        assert_eq!(exchange1_stats.failed_publishes, 1);
    }
} 