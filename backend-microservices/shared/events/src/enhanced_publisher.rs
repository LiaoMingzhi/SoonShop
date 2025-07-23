// 增强的事件发布器 - 支持重试和错误处理
// 文件路径: /d:/workspace/Solana/SoonShop/backend-microservices/shared/events/src/enhanced_publisher.rs

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use lapin::{Connection, ConnectionProperties, Channel, BasicProperties, ExchangeKind};
use lapin::options::{ExchangeDeclareOptions, BasicPublishOptions, QueueDeclareOptions, QueueBindOptions};
use lapin::types::FieldTable;

use crate::{Event, EventPublisher};
use crate::retry::{RetryConfig, RetryState, ErrorInfo, ErrorType};
use crate::metrics::EventMetrics;
use crate::storage::ErrorStorage;
use crate::event_types::*;
use crate::config::EventConfig;
use crate::error::EventError;
use lapin::publisher_confirm::Confirmation;

/// 增强的事件发布器状态
#[derive(Debug)]
struct PublisherState {
    /// 重试状态映射 (事件ID -> 重试状态)
    retry_states: HashMap<Uuid, RetryState>,
    /// 连接状态
    connected: bool,
    /// 最后连接时间
    last_connected_at: Option<chrono::DateTime<Utc>>,
}

/// 增强的RabbitMQ事件发布器
pub struct EnhancedRabbitMQEventPublisher {
    /// 主要频道
    channel: Channel,
    /// 交换机名称
    exchange: String,
    /// 重试配置
    retry_config: RetryConfig,
    /// 发布器状态
    state: Arc<RwLock<PublisherState>>,
    /// 指标收集器
    metrics: Option<Arc<dyn EventMetrics>>,
    /// 错误存储
    error_storage: Option<Arc<dyn ErrorStorage>>,
    /// RabbitMQ连接URL
    rabbitmq_url: String,
}

impl EnhancedRabbitMQEventPublisher {
    /// 创建新的增强事件发布器
    pub async fn new(
        rabbitmq_url: &str,
        exchange: &str,
        retry_config: RetryConfig,
        metrics: Option<Arc<dyn EventMetrics>>,
        error_storage: Option<Arc<dyn ErrorStorage>>,
    ) -> anyhow::Result<Self> {
        let conn = Connection::connect(rabbitmq_url, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        
        // 声明主交换机
        channel.exchange_declare(
            exchange,
            ExchangeKind::Topic,
            ExchangeDeclareOptions::default(),
            Default::default(),
        ).await?;

        // 如果启用死信队列，声明死信交换机和队列
        if retry_config.enable_dead_letter_queue {
            if let Some(dlq_name) = &retry_config.dead_letter_queue {
                let dlx_name = format!("{}.dlx", exchange);
                
                // 声明死信交换机
                channel.exchange_declare(
                    &dlx_name,
                    ExchangeKind::Topic,
                    ExchangeDeclareOptions::default(),
                    Default::default(),
                ).await?;

                // 声明死信队列
                let mut dlq_args = FieldTable::default();
                dlq_args.insert("x-message-ttl".into(), 86400000.into()); // 24小时TTL
                
                channel.queue_declare(
                    dlq_name,
                    QueueDeclareOptions::default(),
                    dlq_args,
                ).await?;

                // 绑定死信队列到死信交换机
                channel.queue_bind(
                    dlq_name,
                    &dlx_name,
                    "#", // 捕获所有消息
                    QueueBindOptions::default(),
                    FieldTable::default(),
                ).await?;

                tracing::info!("Dead letter queue '{}' configured", dlq_name);
            }
        }

        let state = PublisherState {
            retry_states: HashMap::new(),
            connected: true,
            last_connected_at: Some(Utc::now()),
        };

        Ok(Self {
            channel,
            exchange: exchange.to_string(),
            retry_config,
            state: Arc::new(RwLock::new(state)),
            metrics,
            error_storage,
            rabbitmq_url: rabbitmq_url.to_string(),
        })
    }

    /// 发布事件到死信队列
    async fn publish_to_dead_letter_queue<T>(
        &self,
        routing_key: &str,
        event: &Event<T>,
        retry_state: &RetryState,
    ) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync,
    {
        if !self.retry_config.enable_dead_letter_queue {
            return Ok(());
        }

        let dlx_name = format!("{}.dlx", self.exchange);
        
        // 创建死信事件包装器
        let dead_letter_event = serde_json::json!({
            "original_event": event,
            "retry_state": retry_state,
            "dead_letter_reason": "retry_exhausted",
            "dead_letter_time": Utc::now(),
            "original_routing_key": routing_key
        });

        let payload = serde_json::to_vec(&dead_letter_event)?;
        
        // 设置死信消息属性
        let mut headers = std::collections::BTreeMap::new();
        headers.insert("x-dead-letter-reason".into(), lapin::types::LongString::from("retry-exhausted").into());
        headers.insert("x-original-routing-key".into(), lapin::types::LongString::from(routing_key).into());
        headers.insert("x-dead-letter-time".into(), Utc::now().timestamp().into());
        
        let mut properties = BasicProperties::default();
        properties = properties.with_headers(FieldTable::from(headers));

        self.channel.basic_publish(
            &dlx_name,
            routing_key,
            BasicPublishOptions::default(),
            &payload,
            properties,
        ).await?;

        tracing::warn!(
            "Event {} published to dead letter queue after {} failed attempts",
            event.id,
            retry_state.total_failures()
        );

        // 记录死信队列指标
        if let Some(metrics) = &self.metrics {
            metrics.record_dead_letter_event(&self.exchange, routing_key).await;
        }

        Ok(())
    }

    /// 重新连接到RabbitMQ
    async fn reconnect(&self) -> anyhow::Result<Channel> {
        tracing::info!("Attempting to reconnect to RabbitMQ...");
        
        let conn = Connection::connect(&self.rabbitmq_url, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        
        // 重新声明交换机
        channel.exchange_declare(
            &self.exchange,
            ExchangeKind::Topic,
            ExchangeDeclareOptions::default(),
            Default::default(),
        ).await?;

        // 更新连接状态
        {
            let mut state = self.state.write().await;
            state.connected = true;
            state.last_connected_at = Some(Utc::now());
        }

        tracing::info!("Successfully reconnected to RabbitMQ");
        Ok(channel)
    }

    /// 分类错误类型
    fn classify_error(&self, error: &anyhow::Error) -> ErrorType {
        let error_msg = error.to_string().to_lowercase();
        
        if error_msg.contains("connection") || error_msg.contains("network") {
            ErrorType::Network
        } else if error_msg.contains("serialization") || error_msg.contains("json") {
            ErrorType::Serialization
        } else if error_msg.contains("channel") || error_msg.contains("queue") {
            ErrorType::System
        } else {
            ErrorType::Unknown
        }
    }

    /// 持久化错误信息
    async fn persist_error(&self, event_id: Uuid, error_info: &ErrorInfo) -> anyhow::Result<()> {
        if let Some(storage) = &self.error_storage {
            storage.store_error(event_id, error_info).await?;
        }
        Ok(())
    }

    /// 带重试的发布实现
    async fn publish_with_retry<T>(
        &self,
        routing_key: &str,
        event: &Event<T>,
    ) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync,
    {
        let event_id = event.id;
        
        // 获取或创建重试状态
        let mut retry_state = {
            let mut state = self.state.write().await;
            state.retry_states.remove(&event_id).unwrap_or_else(RetryState::new)
        };

        loop {
            // 尝试发布事件
            let result = self.try_publish(routing_key, event).await;
            
            match result {
                Ok(()) => {
                    // 发布成功，清理重试状态
                    {
                        let mut state = self.state.write().await;
                        state.retry_states.remove(&event_id);
                    }
                    
                    // 记录成功指标
                    if let Some(metrics) = &self.metrics {
                        metrics.record_successful_publish(&self.exchange, routing_key).await;
                    }
                    
                    tracing::info!(
                        "Event {} published successfully to exchange '{}' with routing key '{}'",
                        event_id, self.exchange, routing_key
                    );
                    
                    return Ok(());
                }
                Err(error) => {
                    // 分类错误并创建错误信息
                    let error_type = self.classify_error(&error);
                    let error_info = ErrorInfo::new(error_type.clone(), error.to_string())
                        .with_source("enhanced_publisher".to_string())
                        .with_details(format!("Exchange: {}, Routing Key: {}", self.exchange, routing_key));

                    // 记录错误
                    retry_state.record_error(error_info.clone(), &self.retry_config.strategy);
                    
                    // 持久化错误
                    if let Err(e) = self.persist_error(event_id, &error_info).await {
                        tracing::error!("Failed to persist error: {}", e);
                    }
                    
                    // 记录失败指标
                    if let Some(metrics) = &self.metrics {
                        metrics.record_failed_publish(&self.exchange, routing_key, &error_type).await;
                    }
                    
                    // 检查是否可以重试
                    if retry_state.can_retry(&self.retry_config.strategy) && error_type.is_retryable() {
                        if let Some(delay) = self.retry_config.strategy.calculate_delay(retry_state.attempt) {
                            tracing::warn!(
                                "Event {} publish failed (attempt {}), retrying in {:?}: {}",
                                event_id, retry_state.attempt, delay, error
                            );
                            
                            // 保存重试状态
                            {
                                let mut state = self.state.write().await;
                                state.retry_states.insert(event_id, retry_state.clone());
                            }
                            
                            // 等待重试延迟
                            sleep(delay).await;
                            continue;
                        }
                    }
                    
                    // 重试已用尽，发送到死信队列
                    if let Err(dlq_error) = self.publish_to_dead_letter_queue(routing_key, event, &retry_state).await {
                        tracing::error!("Failed to publish to dead letter queue: {}", dlq_error);
                    }
                    
                    tracing::error!(
                        "Event {} publish failed after {} attempts, giving up: {}",
                        event_id, retry_state.total_failures(), error
                    );
                    
                    return Err(error);
                }
            }
        }
    }

    /// 尝试发布事件（单次尝试）
    async fn try_publish<T>(&self, routing_key: &str, event: &Event<T>) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync,
    {
        let payload = serde_json::to_vec(event)?;
        
        // 设置消息属性
        let mut headers = std::collections::BTreeMap::new();
        headers.insert("event_type".into(), lapin::types::LongString::from(event.event_type.clone()).into());
        headers.insert("correlation_id".into(), lapin::types::LongString::from(event.correlation_id.to_string()).into());
        headers.insert("source".into(), lapin::types::LongString::from(event.source.clone()).into());
        
        let mut properties = BasicProperties::default()
            .with_message_id(event.id.to_string().into())
            .with_timestamp(event.timestamp.timestamp() as u64)
            .with_headers(FieldTable::from(headers));

        // 如果启用了持久化，设置消息为持久化
        if self.retry_config.enable_error_persistence {
            properties = properties.with_delivery_mode(2); // 持久化消息
        }

        // 使用超时发布消息
        let publish_future = self.channel.basic_publish(
            &self.exchange,
            routing_key,
            BasicPublishOptions::default(),
            &payload,
            properties,
        );

        // 设置5秒超时
        timeout(Duration::from_secs(5), publish_future).await??;
        
        Ok(())
    }

    /// 获取重试统计信息
    pub async fn get_retry_stats(&self) -> HashMap<Uuid, RetryState> {
        let state = self.state.read().await;
        state.retry_states.clone()
    }

    /// 清理过期的重试状态
    pub async fn cleanup_expired_retries(&self) {
        let mut state = self.state.write().await;
        let now = Utc::now();
        
        state.retry_states.retain(|_, retry_state| {
            // 保留24小时内的重试状态
            now.signed_duration_since(retry_state.first_attempt_at).num_hours() < 24
        });
    }

    /// 强制重试特定事件
    pub async fn force_retry<T>(&self, event_id: Uuid, routing_key: &str, event: &Event<T>) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync,
    {
        // 重置重试状态
        {
            let mut state = self.state.write().await;
            if let Some(retry_state) = state.retry_states.get_mut(&event_id) {
                retry_state.reset();
            }
        }
        
        // 重新尝试发布
        self.publish_with_retry(routing_key, event).await
    }
}

#[async_trait]
impl EventPublisher for EnhancedRabbitMQEventPublisher {
    async fn publish_json(&self, routing_key: &str, event_data: &serde_json::Value) -> anyhow::Result<()> {
        // 从 JSON 中反序列化事件
        let event: Event<serde_json::Value> = serde_json::from_value(event_data.clone())?;
        self.publish_with_retry(routing_key, &event).await
    }
}

/// 后台任务：定期清理过期的重试状态
pub async fn start_retry_cleanup_task(publisher: Arc<EnhancedRabbitMQEventPublisher>) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600)); // 1 hour
    
    loop {
        interval.tick().await;
        publisher.cleanup_expired_retries().await;
        tracing::debug!("Cleaned up expired retry states");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry::{RetryStrategy, RetryConfig};
    use std::time::Duration;

    #[test]
    fn test_error_classification() {
        let publisher = create_test_publisher().await;
        
        let network_error = anyhow::anyhow!("Connection failed");
        assert_eq!(publisher.classify_error(&network_error), ErrorType::Network);
        
        let serialization_error = anyhow::anyhow!("JSON parse error");
        assert_eq!(publisher.classify_error(&serialization_error), ErrorType::Serialization);
    }

    async fn create_test_publisher() -> EnhancedRabbitMQEventPublisher {
        let config = RetryConfig::simple_retry(3, Duration::from_secs(1));
        // 注意：这需要实际的RabbitMQ连接来创建，测试时可能需要模拟
        // EnhancedRabbitMQEventPublisher::new("amqp://test", "test_exchange", config, None, None).await.unwrap()
    }
}

/// 发布器指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherMetrics {
    pub events_published: u64,
    pub delayed_events_published: u64,
    pub batch_events_published: u64,
    pub transactional_events_published: u64,
    pub dead_letter_events_published: u64,
    pub last_publish_time: Option<DateTime<Utc>>,
    pub last_batch_size: usize,
    pub publish_errors: u64,
    pub created_at: DateTime<Utc>,
}

impl PublisherMetrics {
    fn new() -> Self {
        Self {
            events_published: 0,
            delayed_events_published: 0,
            batch_events_published: 0,
            transactional_events_published: 0,
            dead_letter_events_published: 0,
            last_publish_time: None,
            last_batch_size: 0,
            publish_errors: 0,
            created_at: Utc::now(),
        }
    }
}

/// 系统事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    ServiceStarted {
        service_name: String,
        version: String,
        timestamp: DateTime<Utc>,
    },
    ServiceStopped {
        service_name: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    HealthCheckFailed {
        service_name: String,
        error: String,
        timestamp: DateTime<Utc>,
    },
    MetricsReported {
        service_name: String,
        metrics: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
}

impl SystemEvent {
    fn event_type(&self) -> &'static str {
        match self {
            SystemEvent::ServiceStarted { .. } => "service.started",
            SystemEvent::ServiceStopped { .. } => "service.stopped",
            SystemEvent::HealthCheckFailed { .. } => "health.failed",
            SystemEvent::MetricsReported { .. } => "metrics.reported",
        }
    }
}

/// 业务事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessEvent {
    UserRegistered {
        user_id: Uuid,
        email: String,
        timestamp: DateTime<Utc>,
    },
    OrderCreated {
        order_id: Uuid,
        user_id: Uuid,
        total_amount: f64,
        timestamp: DateTime<Utc>,
    },
    PaymentProcessed {
        payment_id: Uuid,
        order_id: Uuid,
        amount: f64,
        timestamp: DateTime<Utc>,
    },
    ProductUpdated {
        product_id: Uuid,
        changes: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
}

impl BusinessEvent {
    fn event_type(&self) -> &'static str {
        match self {
            BusinessEvent::UserRegistered { .. } => "user.registered",
            BusinessEvent::OrderCreated { .. } => "order.created",
            BusinessEvent::PaymentProcessed { .. } => "payment.processed",
            BusinessEvent::ProductUpdated { .. } => "product.updated",
        }
    }
}

// 辅助函数

fn create_default_headers() -> FieldTable {
    let mut headers = FieldTable::default();
    headers.insert("publisher".into(), lapin::types::LongString::from("soonshop-enhanced-publisher").into());
    headers.insert("published_at".into(), lapin::types::LongString::from(Utc::now().to_rfc3339()).into());
    headers
}

fn create_delay_headers(delay_ms: u64) -> FieldTable {
    let mut headers = create_default_headers();
    headers.insert("x-delay".into(), delay_ms.into());
    headers
}

/// 事件发布器构建器
pub struct EventPublisherBuilder {
    config: EventConfig,
}

impl EventPublisherBuilder {
    pub fn new(service_name: &str) -> Self {
        Self {
            config: EventConfig {
                service_name: service_name.to_string(),
                rabbitmq_url: "amqp://localhost:5672".to_string(),
                exchange_name: "soonshop.events".to_string(),
                queue_prefix: "soonshop".to_string(),
                max_retries: 3,
                retry_delay_ms: 1000,
                enable_dead_letter_queue: true,
                dead_letter_queue: Some("soonshop.events.dlq".to_string()),
                enable_metrics: true,
                enable_error_storage: true,
                connection_timeout_secs: 30,
                publish_timeout_secs: 10,
            },
        }
    }

    pub fn with_rabbitmq_url(mut self, url: &str) -> Self {
        self.config.rabbitmq_url = url.to_string();
        self
    }

    pub fn with_exchange_name(mut self, name: &str) -> Self {
        self.config.exchange_name = name.to_string();
        self
    }

    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    pub fn with_retry_delay(mut self, delay_ms: u64) -> Self {
        self.config.retry_delay_ms = delay_ms;
        self
    }

    pub async fn build(self) -> Result<EnhancedRabbitMQEventPublisher, EventError> {
        let retry_config = RetryConfig {
            strategy: crate::retry::RetryStrategy::ExponentialBackoff {
                initial_delay: Duration::from_millis(self.config.retry_delay_ms),
                backoff_factor: 2.0,
                max_delay: Duration::from_millis(self.config.retry_delay_ms * 10),
                max_attempts: self.config.max_retries,
            },
            enable_dead_letter_queue: self.config.enable_dead_letter_queue,
            dead_letter_queue: self.config.dead_letter_queue.clone(),
            enable_error_persistence: self.config.enable_error_storage,
            error_storage_table: None,
            enable_metrics: self.config.enable_metrics,
            metrics_prefix: Some(self.config.service_name.clone()),
        };
        
        EnhancedRabbitMQEventPublisher::new(
            &self.config.rabbitmq_url, 
            &self.config.exchange_name, 
            retry_config, 
            None, 
            None
        ).await.map_err(|e| EventError::from(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_publisher_metrics() {
        let metrics = PublisherMetrics::new();
        assert_eq!(metrics.events_published, 0);
        assert_eq!(metrics.batch_events_published, 0);
        assert!(metrics.last_publish_time.is_none());
    }

    #[test]
    fn test_system_event_types() {
        let event = SystemEvent::ServiceStarted {
            service_name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            timestamp: Utc::now(),
        };
        assert_eq!(event.event_type(), "service.started");
    }

    #[test]
    fn test_business_event_types() {
        let event = BusinessEvent::UserRegistered {
            user_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            timestamp: Utc::now(),
        };
        assert_eq!(event.event_type(), "user.registered");
    }
} 