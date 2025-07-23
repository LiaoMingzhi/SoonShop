// 事件系统核心模块
// 文件路径: /d:/workspace/Solana/SoonShop/backend-microservices/shared/events/src/lib.rs

use async_trait::async_trait;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow;

pub mod publisher;
pub mod subscriber;
pub mod handlers;
pub mod retry;
pub mod enhanced_publisher;
pub mod metrics;
pub mod storage;
pub mod event_types;
pub mod config;
pub mod error;

// 重导出
pub use publisher::*;
pub use subscriber::*;
pub use handlers::*;
pub use retry::*;
pub use enhanced_publisher::*;
pub use metrics::*;
pub use storage::*;
pub use event_types::*;
pub use config::*;
pub use error::*;

/// 事件发布者特征（object-safe 版本）
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish_json(&self, routing_key: &str, event_data: &serde_json::Value) -> anyhow::Result<()>;
}

/// 事件发布者扩展特征（带泛型的版本）
#[async_trait]
pub trait EventPublisherExt: EventPublisher {
    async fn publish<T>(&self, routing_key: &str, event: &Event<T>) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync,
    {
        let event_data = serde_json::to_value(event)?;
        self.publish_json(routing_key, &event_data).await
    }
}

/// 事件订阅者特征（object-safe 版本）
#[async_trait]
pub trait EventSubscriber: Send + Sync {
    async fn subscribe_json(&self, routing_key: &str, handler: Box<dyn EventJsonHandler>) -> anyhow::Result<()>;
}

/// JSON 事件处理器特征
#[async_trait]
pub trait EventJsonHandler: Send + Sync {
    async fn handle_json(&self, data: &serde_json::Value) -> anyhow::Result<()>;
}

/// 事件处理器特征
#[async_trait]
pub trait EventHandler<T>: Send + Sync {
    async fn handle(&self, data: &T) -> anyhow::Result<()>;
}

/// 基础事件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event<T> {
    /// 事件唯一标识
    pub id: Uuid,
    /// 事件类型
    pub event_type: String,
    /// 事件数据
    pub data: T,
    /// 事件时间戳
    pub timestamp: DateTime<Utc>,
    /// 事件来源
    pub source: String,
    /// 关联ID（用于事件链路追踪）
    pub correlation_id: Uuid,
    /// 事件版本
    pub version: String,
    /// 事件元数据
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl<T> Event<T> {
    /// 创建新事件
    pub fn new(event_type: String, data: T, source: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            data,
            timestamp: Utc::now(),
            source,
            correlation_id: Uuid::new_v4(),
            version: "1.0".to_string(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// 设置关联ID
    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = correlation_id;
        self
    }

    /// 设置版本
    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// 事件工厂
pub struct EventFactory {
    default_source: String,
}

impl EventFactory {
    /// 创建事件工厂
    pub fn new(default_source: String) -> Self {
        Self { default_source }
    }

    /// 创建事件
    pub fn create_event<T>(&self, event_type: String, data: T) -> Event<T> {
        Event::new(event_type, data, self.default_source.clone())
    }

    /// 创建带关联ID的事件
    pub fn create_correlated_event<T>(
        &self,
        event_type: String,
        data: T,
        correlation_id: Uuid,
    ) -> Event<T> {
        Event::new(event_type, data, self.default_source.clone())
            .with_correlation_id(correlation_id)
    }
}

/// 事件系统配置
#[derive(Debug, Clone)]
pub struct EventSystemConfig {
    /// RabbitMQ连接URL
    pub rabbitmq_url: String,
    /// 主交换机名称
    pub exchange_name: String,
    /// 队列前缀
    pub queue_prefix: String,
    /// 重试配置
    pub retry_config: RetryConfig,
    /// 是否启用指标收集
    pub enable_metrics: bool,
    /// 是否启用错误存储
    pub enable_error_storage: bool,
    /// 默认事件来源
    pub default_source: String,
}

impl Default for EventSystemConfig {
    fn default() -> Self {
        Self {
            rabbitmq_url: "amqp://localhost:5672".to_string(),
            exchange_name: "soonshop.events".to_string(),
            queue_prefix: "soonshop".to_string(),
            retry_config: RetryConfig::default(),
            enable_metrics: true,
            enable_error_storage: true,
            default_source: "soonshop".to_string(),
        }
    }
}

/// 事件系统管理器
pub struct EventSystem {
    config: EventSystemConfig,
    publisher: Option<std::sync::Arc<dyn EventPublisher>>,
    subscriber: Option<std::sync::Arc<dyn EventSubscriber>>,
    metrics: Option<std::sync::Arc<dyn EventMetrics>>,
    error_storage: Option<std::sync::Arc<dyn ErrorStorage>>,
    event_factory: EventFactory,
}

impl EventSystem {
    /// 创建新的事件系统
    pub fn new(config: EventSystemConfig) -> Self {
        let event_factory = EventFactory::new(config.default_source.clone());
        
        Self {
            config,
            publisher: None,
            subscriber: None,
            metrics: None,
            error_storage: None,
            event_factory,
        }
    }

    /// 初始化事件系统
    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        // 初始化指标收集器
        if self.config.enable_metrics {
            let metrics = std::sync::Arc::new(InMemoryEventMetrics::new());
            self.metrics = Some(metrics);
        }

        // 初始化错误存储
        if self.config.enable_error_storage {
            let error_storage = std::sync::Arc::new(InMemoryStorage::new());
            self.error_storage = Some(error_storage);
        }

        // 初始化增强发布器
        let publisher = std::sync::Arc::new(
            EnhancedRabbitMQEventPublisher::new(
                &self.config.rabbitmq_url,
                &self.config.exchange_name,
                self.config.retry_config.clone(),
                self.metrics.clone(),
                self.error_storage.clone(),
            ).await?
        );
        self.publisher = Some(publisher);

        // 初始化订阅器
        let subscriber = std::sync::Arc::new(
            RabbitMQEventSubscriber::new(
                &self.config.rabbitmq_url,
                &self.config.exchange_name,
                &self.config.queue_prefix,
            ).await?
        );
        self.subscriber = Some(subscriber);

        Ok(())
    }

    /// 获取事件发布器
    pub fn publisher(&self) -> Option<std::sync::Arc<dyn EventPublisher>> {
        self.publisher.clone()
    }

    /// 获取事件订阅器
    pub fn subscriber(&self) -> Option<std::sync::Arc<dyn EventSubscriber>> {
        self.subscriber.clone()
    }

    /// 获取指标收集器
    pub fn metrics(&self) -> Option<std::sync::Arc<dyn EventMetrics>> {
        self.metrics.clone()
    }

    /// 获取错误存储
    pub fn error_storage(&self) -> Option<std::sync::Arc<dyn ErrorStorage>> {
        self.error_storage.clone()
    }

    /// 获取事件工厂
    pub fn event_factory(&self) -> &EventFactory {
        &self.event_factory
    }

    /// 发布事件
    pub async fn publish<T>(&self, event_type: String, routing_key: &str, data: T) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync,
    {
        if let Some(publisher) = &self.publisher {
            let event = self.event_factory.create_event(event_type, data);
            let event_data = serde_json::to_value(&event)?;
            publisher.publish_json(routing_key, &event_data).await
        } else {
            Err(anyhow::anyhow!("Publisher not initialized"))
        }
    }

    /// 订阅事件（简化版本）
    pub async fn subscribe_json<H>(&self, routing_key: &str, handler: H) -> anyhow::Result<()>
    where
        H: EventJsonHandler + Send + Sync + 'static,
    {
        if let Some(subscriber) = &self.subscriber {
            subscriber.subscribe_json(routing_key, Box::new(handler)).await
        } else {
            Err(anyhow::anyhow!("Subscriber not initialized"))
        }
    }

    /// 获取事件系统健康状态
    pub async fn health_check(&self) -> anyhow::Result<EventSystemHealth> {
        let mut health = EventSystemHealth {
            publisher_available: self.publisher.is_some(),
            subscriber_available: self.subscriber.is_some(),
            metrics_available: self.metrics.is_some(),
            error_storage_available: self.error_storage.is_some(),
            rabbitmq_connected: false,
            last_check_time: Utc::now(),
            error_count: 0,
            metrics_summary: None,
        };

        // 检查RabbitMQ连接状态
        // 这里可以添加实际的连接检查逻辑
        health.rabbitmq_connected = true; // 简化实现

        // 获取指标摘要
        if let Some(metrics) = &self.metrics {
            let stats = metrics.get_stats(None).await;
            health.metrics_summary = Some(format!(
                "Success: {}, Failed: {}, Retries: {}",
                stats.successful_publishes,
                stats.failed_publishes,
                stats.retry_attempts
            ));
        }

        Ok(health)
    }
}

/// 事件系统健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSystemHealth {
    pub publisher_available: bool,
    pub subscriber_available: bool,
    pub metrics_available: bool,
    pub error_storage_available: bool,
    pub rabbitmq_connected: bool,
    pub last_check_time: DateTime<Utc>,
    pub error_count: u64,
    pub metrics_summary: Option<String>,
}

impl EventSystemHealth {
    /// 检查系统是否健康
    pub fn is_healthy(&self) -> bool {
        self.publisher_available
            && self.subscriber_available
            && self.rabbitmq_connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestEventData {
        message: String,
        value: i32,
    }

    #[tokio::test]
    async fn test_event_creation() {
        let factory = EventFactory::new("test-service".to_string());
        let data = TestEventData {
            message: "Hello, World!".to_string(),
            value: 42,
        };

        let event = factory.create_event("test.event".to_string(), data);
        
        assert_eq!(event.event_type, "test.event");
        assert_eq!(event.source, "test-service");
        assert_eq!(event.data.message, "Hello, World!");
        assert_eq!(event.data.value, 42);
        assert_eq!(event.version, "1.0");
    }

    #[tokio::test]
    async fn test_event_system_configuration() {
        let config = EventSystemConfig {
            rabbitmq_url: "amqp://test:5672".to_string(),
            exchange_name: "test.exchange".to_string(),
            queue_prefix: "test".to_string(),
            retry_config: RetryConfig::no_retry(),
            enable_metrics: true,
            enable_error_storage: false,
            default_source: "test-service".to_string(),
        };

        let event_system = EventSystem::new(config);
        assert_eq!(event_system.config.exchange_name, "test.exchange");
        assert_eq!(event_system.config.default_source, "test-service");
    }

    #[tokio::test] 
    async fn test_event_metadata() {
        let factory = EventFactory::new("test".to_string());
        let data = TestEventData {
            message: "test".to_string(),
            value: 1,
        };

        let event = factory
            .create_event("test.event".to_string(), data)
            .with_metadata("key1".to_string(), serde_json::json!("value1"))
            .with_metadata("key2".to_string(), serde_json::json!(123));

        assert_eq!(event.metadata.len(), 2);
        assert_eq!(event.metadata.get("key1").unwrap(), &serde_json::json!("value1"));
        assert_eq!(event.metadata.get("key2").unwrap(), &serde_json::json!(123));
    }
} 