use anyhow::Result;
use dashmap::DashMap;
use lapin::{Connection, Channel, BasicProperties};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};
use uuid::Uuid;

use crate::{
    config::EventBusConfig,
    error::EventBusError,
    event::{Event, EventEnvelope, EventMetadata},
    handler::EventHandler,
    storage::EventStorage,
    router::EventRouter,
    publisher::EventPublisher,
    subscriber::EventSubscriber,
    metrics::MetricsCollector,
};

pub type EventHandlerBox = Box<dyn EventHandler + Send + Sync>;

/// 简化的事件总线接口
pub trait EventBus: Send + Sync {
    async fn publish_json(&self, event_type: &str, payload: serde_json::Value) -> Result<(), EventBusError>;
    async fn subscribe(&self, event_type: String, handler: EventHandlerBox) -> Result<(), EventBusError>;
    async fn start(&self) -> Result<(), EventBusError>;
    async fn stop(&self) -> Result<(), EventBusError>;
    async fn health_check(&self) -> Result<bool, EventBusError>;
}

/// RabbitMQ 事件总线实现
pub struct RabbitMQEventBus {
    config: EventBusConfig,
    connection: Arc<RwLock<Option<Connection>>>,
    channel: Arc<RwLock<Option<Channel>>>,
    redis_client: Option<redis::Client>,
    handlers: Arc<DashMap<String, EventHandlerBox>>,
    metrics: Arc<MetricsCollector>,
    is_running: Arc<RwLock<bool>>,
}

impl RabbitMQEventBus {
    pub async fn new(config: EventBusConfig) -> Result<Self, EventBusError> {
        // Redis client initialization (optional)
        let redis_client = if !config.redis.host.is_empty() {
            redis::Client::open(config.redis_url())
                .map_err(|e| EventBusError::ConnectionError(e.to_string()))
                .ok()
        } else {
            None
        };
        
        let handlers = Arc::new(DashMap::new());
        let metrics = Arc::new(MetricsCollector::new());
        
        Ok(Self {
            config,
            connection: Arc::new(RwLock::new(None)),
            channel: Arc::new(RwLock::new(None)),
            redis_client,
            handlers,
            metrics,
            is_running: Arc::new(RwLock::new(false)),
        })
    }
    
    async fn connect(&self) -> Result<(), EventBusError> {
        info!("Connecting to RabbitMQ at {}", self.config.rabbitmq_url());
        
        let connection = Connection::connect(&self.config.rabbitmq_url(), lapin::ConnectionProperties::default())
            .await
            .map_err(|e| EventBusError::ConnectionError(e.to_string()))?;
        
        let channel = connection.create_channel()
            .await
            .map_err(|e| EventBusError::ConnectionError(e.to_string()))?;
        
        // 声明交换器
        channel.exchange_declare(
            &self.config.exchange_name,
            lapin::ExchangeKind::Topic,
            lapin::options::ExchangeDeclareOptions {
                durable: true,
                auto_delete: false,
                internal: false,
                nowait: false,
                passive: false,
            },
            lapin::types::FieldTable::default(),
        )
        .await
        .map_err(|e| EventBusError::ConnectionError(e.to_string()))?;
        
        *self.connection.write().await = Some(connection);
        *self.channel.write().await = Some(channel);
        
        info!("Successfully connected to RabbitMQ");
        Ok(())
    }
    
    async fn get_channel(&self) -> Result<Channel, EventBusError> {
        let channel_guard = self.channel.read().await;
        if let Some(channel) = channel_guard.as_ref() {
            Ok(channel.clone())
        } else {
            Err(EventBusError::ConnectionError("No active channel".to_string()))
        }
    }
}

impl EventBus for RabbitMQEventBus {
    async fn publish_json(&self, event_type: &str, payload: serde_json::Value) -> Result<(), EventBusError> {
        let channel = self.get_channel().await?;
        
        // 创建事件元数据
        let metadata = EventMetadata::new(event_type.to_string(), "event-bus".to_string());
        
        // 构建事件包装器
        let envelope = EventEnvelope {
            metadata: metadata.clone(),
            payload,
        };
        
        // 序列化事件
        let payload_bytes = serde_json::to_vec(&envelope)
            .map_err(|e| EventBusError::SerializationError(e.to_string()))?;
        
        // 发布到交换器
        channel.basic_publish(
            &self.config.exchange_name,
            event_type, // 使用事件类型作为路由键
            lapin::options::BasicPublishOptions::default(),
            &payload_bytes,
            BasicProperties::default()
                .with_message_id(metadata.event_id.to_string().into())
                .with_timestamp(metadata.timestamp.timestamp() as u64)
                .with_content_type("application/json".into())
                .with_delivery_mode(2), // 持久化
        )
        .await
        .map_err(|e| EventBusError::PublishError(e.to_string()))?;
        
        // 更新指标
        self.metrics.increment_published();
        
        info!("Event published: {}", event_type);
        Ok(())
    }
    
    async fn subscribe(&self, event_type: String, handler: EventHandlerBox) -> Result<(), EventBusError> {
        let subscription_id = Uuid::new_v4().to_string();
        self.handlers.insert(subscription_id.clone(), handler);
        
        info!("Subscribed to event type: {}", event_type);
        Ok(())
    }
    
    async fn start(&self) -> Result<(), EventBusError> {
        self.connect().await?;
        *self.is_running.write().await = true;
        info!("Event bus started successfully");
        Ok(())
    }
    
    async fn stop(&self) -> Result<(), EventBusError> {
        *self.is_running.write().await = false;
        *self.channel.write().await = None;
        *self.connection.write().await = None;
        info!("Event bus stopped successfully");
        Ok(())
    }
    
    async fn health_check(&self) -> Result<bool, EventBusError> {
        let is_running = *self.is_running.read().await;
        let has_connection = self.connection.read().await.is_some();
        Ok(is_running && has_connection)
    }
} 