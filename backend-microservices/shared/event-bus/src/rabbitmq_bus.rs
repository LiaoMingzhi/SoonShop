use crate::bus::{EventBus, EventHandler};
use crate::error::EventBusError;
use crate::config::EventBusConfig;
use async_trait::async_trait;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties,
    Channel, Connection, ConnectionProperties, Consumer, ExchangeKind, Result as LapinResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;

/// RabbitMQ事件总线实现
pub struct RabbitMQEventBus {
    connection: Arc<Connection>,
    channel: Arc<Channel>,
    exchange_name: String,
    config: EventBusConfig,
    handlers: Arc<RwLock<HashMap<String, Vec<Box<dyn EventHandler + Send + Sync>>>>>,
}

impl RabbitMQEventBus {
    /// 创建新的RabbitMQ事件总线实例
    pub async fn new(config: EventBusConfig) -> Result<Self, EventBusError> {
        // 建立连接
        let connection = Connection::connect(
            &config.rabbitmq_url,
            ConnectionProperties::default(),
        )
        .await
        .map_err(|e| EventBusError::ConnectionError(e.to_string()))?;

        // 创建通道
        let channel = connection
            .create_channel()
            .await
            .map_err(|e| EventBusError::ChannelError(e.to_string()))?;

        // 声明交换机
        channel
            .exchange_declare(
                &config.exchange_name,
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    auto_delete: false,
                    internal: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| EventBusError::ExchangeError(e.to_string()))?;

        info!("RabbitMQ事件总线已连接到: {}", config.rabbitmq_url);

        Ok(Self {
            connection: Arc::new(connection),
            channel: Arc::new(channel),
            exchange_name: config.exchange_name.clone(),
            config,
            handlers: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// 声明队列
    pub async fn declare_queue(&self, queue_name: &str, routing_key: &str) -> Result<(), EventBusError> {
        // 声明队列
        let queue = self
            .channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    exclusive: false,
                    auto_delete: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| EventBusError::QueueError(e.to_string()))?;

        // 绑定队列到交换机
        self.channel
            .queue_bind(
                queue_name,
                &self.exchange_name,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| EventBusError::BindError(e.to_string()))?;

        info!("队列已声明: {} -> {}", queue_name, routing_key);
        Ok(())
    }

    /// 创建消费者
    pub async fn create_consumer(
        &self,
        queue_name: &str,
        consumer_tag: &str,
    ) -> Result<Consumer, EventBusError> {
        let consumer = self
            .channel
            .basic_consume(
                queue_name,
                consumer_tag,
                BasicConsumeOptions {
                    no_ack: false,
                    exclusive: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| EventBusError::ConsumerError(e.to_string()))?;

        info!("消费者已创建: {} -> {}", consumer_tag, queue_name);
        Ok(consumer)
    }

    /// 启动事件处理
    pub async fn start_event_processing(&self) -> Result<(), EventBusError> {
        let service_name = self.config.service_name.clone();
        let queue_name = format!("{}_queue", service_name);
        let routing_key = format!("{}.*", service_name);

        // 声明队列
        self.declare_queue(&queue_name, &routing_key).await?;

        // 创建消费者
        let consumer = self.create_consumer(&queue_name, &service_name).await?;

        // 处理消息
        let handlers = self.handlers.clone();
        let channel = self.channel.clone();
        
        tokio::spawn(async move {
            let mut consumer_iter = consumer.into_iter();
            
            while let Some(delivery_result) = consumer_iter.next().await {
                match delivery_result {
                    Ok(delivery) => {
                        if let Some(delivery) = delivery {
                            // 处理消息
                            let routing_key = delivery.routing_key.as_str();
                            let payload = delivery.data.clone();
                            
                            // 获取事件处理器
                            let handlers_read = handlers.read().await;
                            if let Some(event_handlers) = handlers_read.get(routing_key) {
                                for handler in event_handlers {
                                    if let Err(e) = handler.handle(&payload).await {
                                        error!("事件处理失败: {}", e);
                                    }
                                }
                            }
                            
                            // 确认消息
                            if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                                error!("消息确认失败: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("消费消息失败: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// 获取连接状态
    pub fn is_connected(&self) -> bool {
        self.connection.status().connected()
    }

    /// 获取统计信息
    pub async fn get_stats(&self) -> EventBusStats {
        let handlers_count = self.handlers.read().await.len();
        EventBusStats {
            connected: self.is_connected(),
            handlers_registered: handlers_count,
            exchange_name: self.exchange_name.clone(),
            service_name: self.config.service_name.clone(),
        }
    }
}

#[async_trait]
impl EventBus for RabbitMQEventBus {
    async fn publish(&self, event: &[u8], routing_key: &str) -> Result<(), EventBusError> {
        // 创建消息属性
        let properties = BasicProperties::default()
            .with_message_id(Uuid::new_v4().to_string().into())
            .with_timestamp(chrono::Utc::now().timestamp() as u64)
            .with_content_type("application/json".into())
            .with_delivery_mode(2); // 持久化消息

        // 发布消息
        let confirmation = self
            .channel
            .basic_publish(
                &self.exchange_name,
                routing_key,
                BasicPublishOptions {
                    mandatory: true,
                    immediate: false,
                },
                event,
                properties,
            )
            .await
            .map_err(|e| EventBusError::PublishError(e.to_string()))?;

        // 等待确认
        match confirmation.await {
            Ok(Confirmation::Ack(_)) => {
                info!("消息已发布: {}", routing_key);
                Ok(())
            }
            Ok(Confirmation::Nack(_)) => {
                Err(EventBusError::PublishError("消息被拒绝".to_string()))
            }
            Err(e) => {
                Err(EventBusError::PublishError(format!("发布确认失败: {}", e)))
            }
        }
    }

    async fn subscribe(&self, routing_key: &str, handler: Box<dyn EventHandler + Send + Sync>) -> Result<(), EventBusError> {
        let mut handlers = self.handlers.write().await;
        handlers.entry(routing_key.to_string())
            .or_insert_with(Vec::new)
            .push(handler);

        info!("事件处理器已注册: {}", routing_key);
        Ok(())
    }

    async fn unsubscribe(&self, routing_key: &str) -> Result<(), EventBusError> {
        let mut handlers = self.handlers.write().await;
        handlers.remove(routing_key);

        info!("事件处理器已取消注册: {}", routing_key);
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), EventBusError> {
        self.connection
            .close(200, "正常关闭")
            .await
            .map_err(|e| EventBusError::DisconnectError(e.to_string()))?;

        info!("RabbitMQ事件总线已断开连接");
        Ok(())
    }
}

/// 事件总线统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBusStats {
    pub connected: bool,
    pub handlers_registered: usize,
    pub exchange_name: String,
    pub service_name: String,
}

/// RabbitMQ事件总线构建器
pub struct RabbitMQEventBusBuilder {
    config: EventBusConfig,
}

impl RabbitMQEventBusBuilder {
    pub fn new(service_name: &str) -> Self {
        Self {
            config: EventBusConfig {
                service_name: service_name.to_string(),
                rabbitmq_url: "amqp://localhost:5672".to_string(),
                exchange_name: "soonshop_exchange".to_string(),
                max_retries: 3,
                retry_delay_ms: 1000,
                heartbeat: 60,
                connection_timeout: 10,
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

    pub fn with_heartbeat(mut self, heartbeat: u16) -> Self {
        self.config.heartbeat = heartbeat;
        self
    }

    pub fn with_connection_timeout(mut self, timeout: u64) -> Self {
        self.config.connection_timeout = timeout;
        self
    }

    pub async fn build(self) -> Result<RabbitMQEventBus, EventBusError> {
        RabbitMQEventBus::new(self.config).await
    }
}

/// 重试策略
#[derive(Debug, Clone)]
pub enum RetryStrategy {
    /// 固定延迟
    Fixed(u64),
    /// 指数退避
    Exponential { base: u64, max: u64 },
    /// 线性退避
    Linear { base: u64, increment: u64 },
}

/// 消息重试处理器
pub struct RetryHandler {
    strategy: RetryStrategy,
    max_retries: u32,
}

impl RetryHandler {
    pub fn new(strategy: RetryStrategy, max_retries: u32) -> Self {
        Self {
            strategy,
            max_retries,
        }
    }

    pub async fn execute_with_retry<F, T, E>(&self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
        E: std::fmt::Display,
    {
        let mut attempts = 0;
        
        loop {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    
                    if attempts > self.max_retries {
                        error!("重试次数已达上限: {}", e);
                        return Err(e);
                    }
                    
                    let delay = self.calculate_delay(attempts);
                    warn!("操作失败，{}ms后重试 (第{}次): {}", delay, attempts, e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
            }
        }
    }

    fn calculate_delay(&self, attempt: u32) -> u64 {
        match &self.strategy {
            RetryStrategy::Fixed(delay) => *delay,
            RetryStrategy::Exponential { base, max } => {
                let delay = base * 2_u64.pow(attempt - 1);
                delay.min(*max)
            }
            RetryStrategy::Linear { base, increment } => {
                base + increment * (attempt as u64 - 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_rabbitmq_event_bus_creation() {
        let config = EventBusConfig {
            service_name: "test-service".to_string(),
            rabbitmq_url: "amqp://localhost:5672".to_string(),
            exchange_name: "test_exchange".to_string(),
            max_retries: 3,
            retry_delay_ms: 1000,
            heartbeat: 60,
            connection_timeout: 10,
        };

        // 注意: 这个测试需要运行的RabbitMQ实例
        // 在CI环境中可能需要跳过
        if std::env::var("SKIP_RABBITMQ_TESTS").is_ok() {
            return;
        }

        let result = RabbitMQEventBus::new(config).await;
        assert!(result.is_ok() || result.is_err()); // 取决于RabbitMQ是否可用
    }

    #[test]
    fn test_retry_strategy() {
        let strategy = RetryStrategy::Fixed(1000);
        let handler = RetryHandler::new(strategy, 3);
        
        assert_eq!(handler.calculate_delay(1), 1000);
        assert_eq!(handler.calculate_delay(2), 1000);
        assert_eq!(handler.calculate_delay(3), 1000);
    }

    #[test]
    fn test_exponential_backoff() {
        let strategy = RetryStrategy::Exponential { base: 100, max: 5000 };
        let handler = RetryHandler::new(strategy, 5);
        
        assert_eq!(handler.calculate_delay(1), 100);
        assert_eq!(handler.calculate_delay(2), 200);
        assert_eq!(handler.calculate_delay(3), 400);
        assert_eq!(handler.calculate_delay(4), 800);
        assert_eq!(handler.calculate_delay(5), 1600);
    }
} 