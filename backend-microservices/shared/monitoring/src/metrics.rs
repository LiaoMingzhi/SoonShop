use prometheus::{
    Counter, Histogram, Gauge, Registry, Encoder, TextEncoder,
    HistogramOpts, Opts, CounterVec, HistogramVec, GaugeVec,
};
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use once_cell::sync::Lazy;

/// 系统指标收集器
pub struct MetricsCollector {
    registry: Registry,
    
    // HTTP请求指标
    pub http_requests_total: CounterVec,
    pub http_request_duration: HistogramVec,
    pub http_requests_in_flight: GaugeVec,
    
    // 数据库指标
    pub db_connections_active: Gauge,
    pub db_connections_total: Counter,
    pub db_query_duration: HistogramVec,
    pub db_errors_total: CounterVec,
    
    // 消息队列指标
    pub messages_published_total: CounterVec,
    pub messages_consumed_total: CounterVec,
    pub message_processing_duration: HistogramVec,
    pub message_queue_size: GaugeVec,
    
    // 事件指标
    pub events_published_total: CounterVec,
    pub events_processed_total: CounterVec,
    pub event_processing_duration: HistogramVec,
    pub event_failures_total: CounterVec,
    
    // 缓存指标
    pub cache_hits_total: CounterVec,
    pub cache_misses_total: CounterVec,
    pub cache_operations_duration: HistogramVec,
    pub cache_size: GaugeVec,
    
    // 系统指标
    pub cpu_usage: Gauge,
    pub memory_usage: Gauge,
    pub disk_usage: GaugeVec,
    pub network_io: CounterVec,
    
    // 业务指标
    pub active_users: Gauge,
    pub orders_total: CounterVec,
    pub payments_total: CounterVec,
    pub revenue_total: CounterVec,
}

impl MetricsCollector {
    pub fn new(service_name: &str) -> Result<Self> {
        let registry = Registry::new();
        
        // HTTP请求指标
        let http_requests_total = CounterVec::new(
            Opts::new("http_requests_total", "Total HTTP requests")
                .namespace(service_name),
            &["method", "endpoint", "status"],
        )?;
        
        let http_request_duration = HistogramVec::new(
            HistogramOpts::new("http_request_duration_seconds", "HTTP request duration")
                .namespace(service_name)
                .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0]),
            &["method", "endpoint"],
        )?;
        
        let http_requests_in_flight = GaugeVec::new(
            Opts::new("http_requests_in_flight", "HTTP requests currently being processed")
                .namespace(service_name),
            &["method", "endpoint"],
        )?;
        
        // 数据库指标
        let db_connections_active = Gauge::new(
            "db_connections_active",
            "Number of active database connections",
        )?;
        
        let db_connections_total = Counter::new(
            "db_connections_total",
            "Total database connections created",
        )?;
        
        let db_query_duration = HistogramVec::new(
            HistogramOpts::new("db_query_duration_seconds", "Database query duration")
                .namespace(service_name)
                .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0]),
            &["query_type", "table"],
        )?;
        
        let db_errors_total = CounterVec::new(
            Opts::new("db_errors_total", "Total database errors")
                .namespace(service_name),
            &["error_type", "operation"],
        )?;
        
        // 消息队列指标
        let messages_published_total = CounterVec::new(
            Opts::new("messages_published_total", "Total messages published")
                .namespace(service_name),
            &["queue", "topic"],
        )?;
        
        let messages_consumed_total = CounterVec::new(
            Opts::new("messages_consumed_total", "Total messages consumed")
                .namespace(service_name),
            &["queue", "topic"],
        )?;
        
        let message_processing_duration = HistogramVec::new(
            HistogramOpts::new("message_processing_duration_seconds", "Message processing duration")
                .namespace(service_name)
                .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0]),
            &["queue", "handler"],
        )?;
        
        let message_queue_size = GaugeVec::new(
            Opts::new("message_queue_size", "Current message queue size")
                .namespace(service_name),
            &["queue"],
        )?;
        
        // 事件指标
        let events_published_total = CounterVec::new(
            Opts::new("events_published_total", "Total events published")
                .namespace(service_name),
            &["event_type", "aggregate_type"],
        )?;
        
        let events_processed_total = CounterVec::new(
            Opts::new("events_processed_total", "Total events processed")
                .namespace(service_name),
            &["event_type", "handler"],
        )?;
        
        let event_processing_duration = HistogramVec::new(
            HistogramOpts::new("event_processing_duration_seconds", "Event processing duration")
                .namespace(service_name)
                .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0]),
            &["event_type", "handler"],
        )?;
        
        let event_failures_total = CounterVec::new(
            Opts::new("event_failures_total", "Total event processing failures")
                .namespace(service_name),
            &["event_type", "handler", "error_type"],
        )?;
        
        // 缓存指标
        let cache_hits_total = CounterVec::new(
            Opts::new("cache_hits_total", "Total cache hits")
                .namespace(service_name),
            &["cache_name"],
        )?;
        
        let cache_misses_total = CounterVec::new(
            Opts::new("cache_misses_total", "Total cache misses")
                .namespace(service_name),
            &["cache_name"],
        )?;
        
        let cache_operations_duration = HistogramVec::new(
            HistogramOpts::new("cache_operations_duration_seconds", "Cache operation duration")
                .namespace(service_name)
                .buckets(vec![0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1]),
            &["cache_name", "operation"],
        )?;
        
        let cache_size = GaugeVec::new(
            Opts::new("cache_size", "Current cache size")
                .namespace(service_name),
            &["cache_name"],
        )?;
        
        // 系统指标
        let cpu_usage = Gauge::new(
            "cpu_usage_percent",
            "Current CPU usage percentage",
        )?;
        
        let memory_usage = Gauge::new(
            "memory_usage_bytes",
            "Current memory usage in bytes",
        )?;
        
        let disk_usage = GaugeVec::new(
            Opts::new("disk_usage_bytes", "Current disk usage in bytes")
                .namespace(service_name),
            &["mount_point"],
        )?;
        
        let network_io = CounterVec::new(
            Opts::new("network_io_bytes_total", "Total network I/O in bytes")
                .namespace(service_name),
            &["direction", "interface"],
        )?;
        
        // 业务指标
        let active_users = Gauge::new(
            "active_users",
            "Number of currently active users",
        )?;
        
        let orders_total = CounterVec::new(
            Opts::new("orders_total", "Total orders")
                .namespace(service_name),
            &["status", "payment_method"],
        )?;
        
        let payments_total = CounterVec::new(
            Opts::new("payments_total", "Total payments")
                .namespace(service_name),
            &["method", "status", "currency"],
        )?;
        
        let revenue_total = CounterVec::new(
            Opts::new("revenue_total", "Total revenue")
                .namespace(service_name),
            &["currency", "source"],
        )?;
        
        // 注册所有指标
        registry.register(Box::new(http_requests_total.clone()))?;
        registry.register(Box::new(http_request_duration.clone()))?;
        registry.register(Box::new(http_requests_in_flight.clone()))?;
        registry.register(Box::new(db_connections_active.clone()))?;
        registry.register(Box::new(db_connections_total.clone()))?;
        registry.register(Box::new(db_query_duration.clone()))?;
        registry.register(Box::new(db_errors_total.clone()))?;
        registry.register(Box::new(messages_published_total.clone()))?;
        registry.register(Box::new(messages_consumed_total.clone()))?;
        registry.register(Box::new(message_processing_duration.clone()))?;
        registry.register(Box::new(message_queue_size.clone()))?;
        registry.register(Box::new(events_published_total.clone()))?;
        registry.register(Box::new(events_processed_total.clone()))?;
        registry.register(Box::new(event_processing_duration.clone()))?;
        registry.register(Box::new(event_failures_total.clone()))?;
        registry.register(Box::new(cache_hits_total.clone()))?;
        registry.register(Box::new(cache_misses_total.clone()))?;
        registry.register(Box::new(cache_operations_duration.clone()))?;
        registry.register(Box::new(cache_size.clone()))?;
        registry.register(Box::new(cpu_usage.clone()))?;
        registry.register(Box::new(memory_usage.clone()))?;
        registry.register(Box::new(disk_usage.clone()))?;
        registry.register(Box::new(network_io.clone()))?;
        registry.register(Box::new(active_users.clone()))?;
        registry.register(Box::new(orders_total.clone()))?;
        registry.register(Box::new(payments_total.clone()))?;
        registry.register(Box::new(revenue_total.clone()))?;
        
        Ok(Self {
            registry,
            http_requests_total,
            http_request_duration,
            http_requests_in_flight,
            db_connections_active,
            db_connections_total,
            db_query_duration,
            db_errors_total,
            messages_published_total,
            messages_consumed_total,
            message_processing_duration,
            message_queue_size,
            events_published_total,
            events_processed_total,
            event_processing_duration,
            event_failures_total,
            cache_hits_total,
            cache_misses_total,
            cache_operations_duration,
            cache_size,
            cpu_usage,
            memory_usage,
            disk_usage,
            network_io,
            active_users,
            orders_total,
            payments_total,
            revenue_total,
        })
    }
    
    /// 获取所有指标的Prometheus格式输出
    pub fn gather(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
    
    /// 更新系统指标
    pub fn update_system_metrics(&self) -> Result<()> {
        use sysinfo::System;
        
        let mut system = System::new_all();
        system.refresh_all();
        
        // CPU使用率 - 新版本API
        let cpu_usage = system.global_cpu_info().cpu_usage() as f64;
        self.cpu_usage.set(cpu_usage);
        
        // 内存使用量
        let memory_usage = system.used_memory() as f64;
        self.memory_usage.set(memory_usage);
        
        // 注意：新版本的sysinfo可能需要不同的API来获取磁盘和网络信息
        // 暂时注释掉这些部分以避免编译错误
        /*
        // 磁盘使用量
        for disk in system.disks() {
            let mount_point = disk.mount_point().to_string_lossy();
            let used_space = disk.total_space() - disk.available_space();
            self.disk_usage
                .with_label_values(&[&mount_point])
                .set(used_space as f64);
        }
        
        // 网络I/O（累计值）
        for (interface_name, network) in system.networks() {
            self.network_io
                .with_label_values(&["received", interface_name])
                .inc_by(network.received() as f64);
            self.network_io
                .with_label_values(&["transmitted", interface_name])
                .inc_by(network.transmitted() as f64);
        }
        */
        
        Ok(())
    }
    
    /// 记录HTTP请求
    pub fn record_http_request(
        &self,
        method: &str,
        endpoint: &str,
        status: &str,
        duration: f64,
    ) {
        self.http_requests_total
            .with_label_values(&[method, endpoint, status])
            .inc();
        
        self.http_request_duration
            .with_label_values(&[method, endpoint])
            .observe(duration);
    }
    
    /// 记录数据库查询
    pub fn record_db_query(&self, query_type: &str, table: &str, duration: f64) {
        self.db_query_duration
            .with_label_values(&[query_type, table])
            .observe(duration);
    }
    
    /// 记录数据库错误
    pub fn record_db_error(&self, error_type: &str, operation: &str) {
        self.db_errors_total
            .with_label_values(&[error_type, operation])
            .inc();
    }
    
    /// 记录事件发布
    pub fn record_event_published(&self, event_type: &str, aggregate_type: &str) {
        self.events_published_total
            .with_label_values(&[event_type, aggregate_type])
            .inc();
    }
    
    /// 记录事件处理
    pub fn record_event_processed(&self, event_type: &str, handler: &str, duration: f64) {
        self.events_processed_total
            .with_label_values(&[event_type, handler])
            .inc();
        
        self.event_processing_duration
            .with_label_values(&[event_type, handler])
            .observe(duration);
    }
    
    /// 记录事件处理失败
    pub fn record_event_failure(&self, event_type: &str, handler: &str, error_type: &str) {
        self.event_failures_total
            .with_label_values(&[event_type, handler, error_type])
            .inc();
    }
    
    /// 记录缓存命中
    pub fn record_cache_hit(&self, cache_name: &str) {
        self.cache_hits_total
            .with_label_values(&[cache_name])
            .inc();
    }
    
    /// 记录缓存未命中
    pub fn record_cache_miss(&self, cache_name: &str) {
        self.cache_misses_total
            .with_label_values(&[cache_name])
            .inc();
    }
    
    /// 记录订单
    pub fn record_order(&self, status: &str, payment_method: &str) {
        self.orders_total
            .with_label_values(&[status, payment_method])
            .inc();
    }
    
    /// 记录支付
    pub fn record_payment(&self, method: &str, status: &str, currency: &str) {
        self.payments_total
            .with_label_values(&[method, status, currency])
            .inc();
    }
    
    /// 记录收入
    pub fn record_revenue(&self, currency: &str, source: &str, amount: f64) {
        self.revenue_total
            .with_label_values(&[currency, source])
            .inc_by(amount);
    }
    
    /// 设置活跃用户数
    pub fn set_active_users(&self, count: f64) {
        self.active_users.set(count);
    }
}

/// 全局指标收集器
pub static METRICS: Lazy<Arc<MetricsCollector>> = Lazy::new(|| {
    Arc::new(
        MetricsCollector::new("soonshop")
            .expect("Failed to create metrics collector")
    )
});

/// 便捷宏用于记录指标
#[macro_export]
macro_rules! record_http_request {
    ($method:expr, $endpoint:expr, $status:expr, $duration:expr) => {
        $crate::METRICS.record_http_request($method, $endpoint, $status, $duration);
    };
}

#[macro_export]
macro_rules! record_db_query {
    ($query_type:expr, $table:expr, $duration:expr) => {
        $crate::METRICS.record_db_query($query_type, $table, $duration);
    };
}

#[macro_export]
macro_rules! record_event_published {
    ($event_type:expr, $aggregate_type:expr) => {
        $crate::METRICS.record_event_published($event_type, $aggregate_type);
    };
}

#[macro_export]
macro_rules! record_event_processed {
    ($event_type:expr, $handler:expr, $duration:expr) => {
        $crate::METRICS.record_event_processed($event_type, $handler, $duration);
    };
} 