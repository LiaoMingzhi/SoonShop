use crate::config::MonitoringConfig;
use crate::error::MonitoringError;
use prometheus::{
    Counter, CounterVec, Gauge, GaugeVec, Histogram, HistogramOpts, HistogramVec, IntCounter,
    IntCounterVec, IntGauge, IntGaugeVec, Opts, Registry,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Prometheus指标收集器
pub struct PrometheusMetrics {
    registry: Registry,
    config: MonitoringConfig,
    
    // HTTP 指标
    http_requests_total: IntCounterVec,
    http_request_duration: HistogramVec,
    http_response_size: HistogramVec,
    
    // 数据库指标
    db_connections_active: IntGauge,
    db_connections_idle: IntGauge,
    db_query_duration: HistogramVec,
    db_queries_total: IntCounterVec,
    
    // 消息队列指标
    mq_messages_published: IntCounterVec,
    mq_messages_consumed: IntCounterVec,
    mq_message_processing_duration: HistogramVec,
    mq_connection_status: IntGauge,
    
    // 业务指标
    business_operations_total: IntCounterVec,
    business_operation_duration: HistogramVec,
    business_errors_total: IntCounterVec,
    
    // 系统指标
    system_uptime: Gauge,
    system_memory_usage: Gauge,
    system_cpu_usage: Gauge,
    system_disk_usage: GaugeVec,
    
    // 缓存指标
    cache_hits_total: IntCounter,
    cache_misses_total: IntCounter,
    cache_operations_duration: Histogram,
    
    // 自定义指标
    custom_metrics: Arc<RwLock<HashMap<String, Box<dyn CustomMetric + Send + Sync>>>>,
}

impl PrometheusMetrics {
    /// 创建新的Prometheus指标收集器
    pub fn new(config: MonitoringConfig) -> Result<Self, MonitoringError> {
        let registry = Registry::new();
        
        // HTTP 指标
        let http_requests_total = IntCounterVec::new(
            Opts::new("http_requests_total", "Total number of HTTP requests")
                .namespace(&config.namespace)
                .subsystem("http"),
            &["method", "path", "status"],
        )?;
        
        let http_request_duration = HistogramVec::new(
            HistogramOpts::new("http_request_duration_seconds", "HTTP request duration")
                .namespace(&config.namespace)
                .subsystem("http")
                .buckets(vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
            &["method", "path"],
        )?;
        
        let http_response_size = HistogramVec::new(
            HistogramOpts::new("http_response_size_bytes", "HTTP response size")
                .namespace(&config.namespace)
                .subsystem("http")
                .buckets(vec![64.0, 256.0, 1024.0, 4096.0, 16384.0, 65536.0, 262144.0, 1048576.0]),
            &["method", "path"],
        )?;
        
        // 数据库指标
        let db_connections_active = IntGauge::new(
            "db_connections_active",
            "Number of active database connections",
        )?;
        
        let db_connections_idle = IntGauge::new(
            "db_connections_idle",
            "Number of idle database connections",
        )?;
        
        let db_query_duration = HistogramVec::new(
            HistogramOpts::new("db_query_duration_seconds", "Database query duration")
                .namespace(&config.namespace)
                .subsystem("db")
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]),
            &["operation", "table"],
        )?;
        
        let db_queries_total = IntCounterVec::new(
            Opts::new("db_queries_total", "Total number of database queries")
                .namespace(&config.namespace)
                .subsystem("db"),
            &["operation", "table", "result"],
        )?;
        
        // 消息队列指标
        let mq_messages_published = IntCounterVec::new(
            Opts::new("mq_messages_published_total", "Total number of messages published")
                .namespace(&config.namespace)
                .subsystem("mq"),
            &["exchange", "routing_key"],
        )?;
        
        let mq_messages_consumed = IntCounterVec::new(
            Opts::new("mq_messages_consumed_total", "Total number of messages consumed")
                .namespace(&config.namespace)
                .subsystem("mq"),
            &["queue", "result"],
        )?;
        
        let mq_message_processing_duration = HistogramVec::new(
            HistogramOpts::new("mq_message_processing_duration_seconds", "Message processing duration")
                .namespace(&config.namespace)
                .subsystem("mq")
                .buckets(vec![0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
            &["queue", "handler"],
        )?;
        
        let mq_connection_status = IntGauge::new(
            "mq_connection_status",
            "Message queue connection status (1 = connected, 0 = disconnected)",
        )?;
        
        // 业务指标
        let business_operations_total = IntCounterVec::new(
            Opts::new("business_operations_total", "Total number of business operations")
                .namespace(&config.namespace)
                .subsystem("business"),
            &["operation", "service", "result"],
        )?;
        
        let business_operation_duration = HistogramVec::new(
            HistogramOpts::new("business_operation_duration_seconds", "Business operation duration")
                .namespace(&config.namespace)
                .subsystem("business")
                .buckets(vec![0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
            &["operation", "service"],
        )?;
        
        let business_errors_total = IntCounterVec::new(
            Opts::new("business_errors_total", "Total number of business errors")
                .namespace(&config.namespace)
                .subsystem("business"),
            &["operation", "service", "error_type"],
        )?;
        
        // 系统指标
        let system_uptime = Gauge::new(
            "system_uptime_seconds",
            "System uptime in seconds",
        )?;
        
        let system_memory_usage = Gauge::new(
            "system_memory_usage_bytes",
            "System memory usage in bytes",
        )?;
        
        let system_cpu_usage = Gauge::new(
            "system_cpu_usage_percent",
            "System CPU usage percentage",
        )?;
        
        let system_disk_usage = GaugeVec::new(
            Opts::new("system_disk_usage_bytes", "System disk usage in bytes")
                .namespace(&config.namespace)
                .subsystem("system"),
            &["disk", "type"],
        )?;
        
        // 缓存指标
        let cache_hits_total = IntCounter::new(
            "cache_hits_total",
            "Total number of cache hits",
        )?;
        
        let cache_misses_total = IntCounter::new(
            "cache_misses_total",
            "Total number of cache misses",
        )?;
        
        let cache_operations_duration = Histogram::with_opts(
            HistogramOpts::new("cache_operations_duration_seconds", "Cache operation duration")
                .namespace(&config.namespace)
                .subsystem("cache")
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5]),
        )?;
        
        // 注册所有指标
        registry.register(Box::new(http_requests_total.clone()))?;
        registry.register(Box::new(http_request_duration.clone()))?;
        registry.register(Box::new(http_response_size.clone()))?;
        registry.register(Box::new(db_connections_active.clone()))?;
        registry.register(Box::new(db_connections_idle.clone()))?;
        registry.register(Box::new(db_query_duration.clone()))?;
        registry.register(Box::new(db_queries_total.clone()))?;
        registry.register(Box::new(mq_messages_published.clone()))?;
        registry.register(Box::new(mq_messages_consumed.clone()))?;
        registry.register(Box::new(mq_message_processing_duration.clone()))?;
        registry.register(Box::new(mq_connection_status.clone()))?;
        registry.register(Box::new(business_operations_total.clone()))?;
        registry.register(Box::new(business_operation_duration.clone()))?;
        registry.register(Box::new(business_errors_total.clone()))?;
        registry.register(Box::new(system_uptime.clone()))?;
        registry.register(Box::new(system_memory_usage.clone()))?;
        registry.register(Box::new(system_cpu_usage.clone()))?;
        registry.register(Box::new(system_disk_usage.clone()))?;
        registry.register(Box::new(cache_hits_total.clone()))?;
        registry.register(Box::new(cache_misses_total.clone()))?;
        registry.register(Box::new(cache_operations_duration.clone()))?;
        
        info!("Prometheus指标收集器已初始化");
        
        Ok(Self {
            registry,
            config,
            http_requests_total,
            http_request_duration,
            http_response_size,
            db_connections_active,
            db_connections_idle,
            db_query_duration,
            db_queries_total,
            mq_messages_published,
            mq_messages_consumed,
            mq_message_processing_duration,
            mq_connection_status,
            business_operations_total,
            business_operation_duration,
            business_errors_total,
            system_uptime,
            system_memory_usage,
            system_cpu_usage,
            system_disk_usage,
            cache_hits_total,
            cache_misses_total,
            cache_operations_duration,
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// 记录HTTP请求指标
    pub fn record_http_request(&self, method: &str, path: &str, status: u16, duration: f64, response_size: f64) {
        let status_str = status.to_string();
        self.http_requests_total
            .with_label_values(&[method, path, &status_str])
            .inc();
        
        self.http_request_duration
            .with_label_values(&[method, path])
            .observe(duration);
        
        self.http_response_size
            .with_label_values(&[method, path])
            .observe(response_size);
    }
    
    /// 记录数据库查询指标
    pub fn record_db_query(&self, operation: &str, table: &str, duration: f64, success: bool) {
        let result = if success { "success" } else { "error" };
        
        self.db_queries_total
            .with_label_values(&[operation, table, result])
            .inc();
        
        self.db_query_duration
            .with_label_values(&[operation, table])
            .observe(duration);
    }
    
    /// 更新数据库连接状态
    pub fn update_db_connections(&self, active: i64, idle: i64) {
        self.db_connections_active.set(active);
        self.db_connections_idle.set(idle);
    }
    
    /// 记录消息队列指标
    pub fn record_mq_message_published(&self, exchange: &str, routing_key: &str) {
        self.mq_messages_published
            .with_label_values(&[exchange, routing_key])
            .inc();
    }
    
    pub fn record_mq_message_consumed(&self, queue: &str, success: bool, duration: f64, handler: &str) {
        let result = if success { "success" } else { "error" };
        
        self.mq_messages_consumed
            .with_label_values(&[queue, result])
            .inc();
        
        self.mq_message_processing_duration
            .with_label_values(&[queue, handler])
            .observe(duration);
    }
    
    /// 更新消息队列连接状态
    pub fn update_mq_connection_status(&self, connected: bool) {
        self.mq_connection_status.set(if connected { 1 } else { 0 });
    }
    
    /// 记录业务操作指标
    pub fn record_business_operation(&self, operation: &str, service: &str, duration: f64, success: bool) {
        let result = if success { "success" } else { "error" };
        
        self.business_operations_total
            .with_label_values(&[operation, service, result])
            .inc();
        
        self.business_operation_duration
            .with_label_values(&[operation, service])
            .observe(duration);
    }
    
    /// 记录业务错误
    pub fn record_business_error(&self, operation: &str, service: &str, error_type: &str) {
        self.business_errors_total
            .with_label_values(&[operation, service, error_type])
            .inc();
    }
    
    /// 更新系统指标
    pub fn update_system_metrics(&self, uptime: f64, memory_usage: f64, cpu_usage: f64) {
        self.system_uptime.set(uptime);
        self.system_memory_usage.set(memory_usage);
        self.system_cpu_usage.set(cpu_usage);
    }
    
    /// 更新磁盘使用情况
    pub fn update_disk_usage(&self, disk: &str, usage_type: &str, bytes: f64) {
        self.system_disk_usage
            .with_label_values(&[disk, usage_type])
            .set(bytes);
    }
    
    /// 记录缓存操作
    pub fn record_cache_hit(&self, duration: f64) {
        self.cache_hits_total.inc();
        self.cache_operations_duration.observe(duration);
    }
    
    pub fn record_cache_miss(&self, duration: f64) {
        self.cache_misses_total.inc();
        self.cache_operations_duration.observe(duration);
    }
    
    /// 注册自定义指标
    pub async fn register_custom_metric(&self, name: &str, metric: Box<dyn CustomMetric + Send + Sync>) -> Result<(), MonitoringError> {
        let mut custom_metrics = self.custom_metrics.write().await;
        custom_metrics.insert(name.to_string(), metric);
        Ok(())
    }
    
    /// 获取指标数据
    pub fn gather(&self) -> Vec<prometheus::proto::MetricFamily> {
        self.registry.gather()
    }
    
    /// 获取指标文本格式
    pub fn export_text(&self) -> String {
        use prometheus::TextEncoder;
        let encoder = TextEncoder::new();
        let metric_families = self.gather();
        encoder.encode_to_string(&metric_families).unwrap_or_default()
    }
    
    /// 获取指标统计
    pub async fn get_stats(&self) -> MetricsStats {
        let custom_metrics_count = self.custom_metrics.read().await.len();
        
        MetricsStats {
            total_metrics: self.registry.gather().len(),
            custom_metrics: custom_metrics_count,
            registry_size: self.registry.gather().iter().map(|mf| mf.get_metric().len()).sum(),
            last_updated: Utc::now(),
        }
    }
}

/// 自定义指标trait
pub trait CustomMetric {
    fn name(&self) -> &str;
    fn help(&self) -> &str;
    fn metric_type(&self) -> &str;
    fn value(&self) -> f64;
}

/// 指标统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsStats {
    pub total_metrics: usize,
    pub custom_metrics: usize,
    pub registry_size: usize,
    pub last_updated: DateTime<Utc>,
}

/// 计时器辅助工具
pub struct Timer {
    start_time: std::time::Instant,
}

impl Timer {
    pub fn start() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }
    
    pub fn elapsed(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
    
    pub fn elapsed_ms(&self) -> f64 {
        self.start_time.elapsed().as_millis() as f64
    }
}

/// 指标中间件
pub struct MetricsMiddleware {
    metrics: Arc<PrometheusMetrics>,
}

impl MetricsMiddleware {
    pub fn new(metrics: Arc<PrometheusMetrics>) -> Self {
        Self { metrics }
    }
    
    pub fn wrap_http_handler<F>(&self, handler: F) -> impl Fn() -> ()
    where
        F: Fn() -> (),
    {
        let metrics = self.metrics.clone();
        move || {
            let timer = Timer::start();
            handler();
            let duration = timer.elapsed();
            // 这里可以添加更多的指标记录逻辑
        }
    }
}

/// 指标导出器
pub struct MetricsExporter {
    metrics: Arc<PrometheusMetrics>,
    config: MonitoringConfig,
}

impl MetricsExporter {
    pub fn new(metrics: Arc<PrometheusMetrics>, config: MonitoringConfig) -> Self {
        Self { metrics, config }
    }
    
    /// 启动指标导出服务
    pub async fn start_server(&self) -> Result<(), MonitoringError> {
        use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
        
        let metrics = self.metrics.clone();
        let bind_addr = format!("{}:{}", self.config.host, self.config.port);
        
        info!("启动指标导出服务: {}", bind_addr);
        
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(metrics.clone()))
                .route("/metrics", web::get().to(metrics_handler))
                .route("/health", web::get().to(health_handler))
        })
        .bind(&bind_addr)
        .map_err(|e| MonitoringError::ServerError(e.to_string()))?
        .run()
        .await
        .map_err(|e| MonitoringError::ServerError(e.to_string()))?;
        
        Ok(())
    }
    
    /// 定期导出指标
    pub async fn export_periodically(&self, interval_secs: u64) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));
        
        loop {
            interval.tick().await;
            
            let metrics_text = self.metrics.export_text();
            if let Err(e) = self.export_to_file(&metrics_text).await {
                error!("导出指标到文件失败: {}", e);
            }
        }
    }
    
    async fn export_to_file(&self, metrics: &str) -> Result<(), MonitoringError> {
        use tokio::fs::OpenOptions;
        use tokio::io::AsyncWriteExt;
        
        let filename = format!("metrics_{}.txt", Utc::now().format("%Y%m%d_%H%M%S"));
        let filepath = format!("{}/{}", self.config.export_path, filename);
        
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&filepath)
            .await
            .map_err(|e| MonitoringError::FileError(e.to_string()))?;
        
        file.write_all(metrics.as_bytes())
            .await
            .map_err(|e| MonitoringError::FileError(e.to_string()))?;
        
        Ok(())
    }
}

// HTTP处理器
async fn metrics_handler(metrics: web::Data<Arc<PrometheusMetrics>>) -> ActixResult<HttpResponse> {
    let metrics_text = metrics.export_text();
    Ok(HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics_text))
}

async fn health_handler() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now()
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MonitoringConfig;
    
    #[test]
    fn test_prometheus_metrics_creation() {
        let config = MonitoringConfig {
            namespace: "test".to_string(),
            service_name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 9090,
            export_path: "/tmp".to_string(),
            scrape_interval: 15,
        };
        
        let metrics = PrometheusMetrics::new(config);
        assert!(metrics.is_ok());
    }
    
    #[test]
    fn test_timer() {
        let timer = Timer::start();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let elapsed = timer.elapsed();
        assert!(elapsed >= 0.01);
    }
    
    #[tokio::test]
    async fn test_metrics_stats() {
        let config = MonitoringConfig {
            namespace: "test".to_string(),
            service_name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 9090,
            export_path: "/tmp".to_string(),
            scrape_interval: 15,
        };
        
        let metrics = PrometheusMetrics::new(config).unwrap();
        let stats = metrics.get_stats().await;
        assert!(stats.total_metrics > 0);
    }
} 