// 事件发布者 - RabbitMQ实现
use async_trait::async_trait;
use serde::Serialize;
use lapin::{Connection, ConnectionProperties, Channel, BasicProperties, ExchangeKind};
use lapin::options::{ExchangeDeclareOptions, BasicPublishOptions};
use crate::{Event, EventPublisher};

pub struct RabbitMQEventPublisher {
    channel: Channel,
    exchange: String,
}

impl RabbitMQEventPublisher {
    pub async fn new(rabbitmq_url: &str, exchange: &str) -> anyhow::Result<Self> {
        let conn = Connection::connect(rabbitmq_url, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;
        
        // 声明交换机
        channel.exchange_declare(
            exchange,
            ExchangeKind::Topic,
            ExchangeDeclareOptions::default(),
            Default::default(),
        ).await?;

        Ok(Self {
            channel,
            exchange: exchange.to_string(),
        })
    }
}

#[async_trait]
impl EventPublisher for RabbitMQEventPublisher {
    async fn publish_json(&self, routing_key: &str, event_data: &serde_json::Value) -> anyhow::Result<()> {
        // 从 JSON 中反序列化事件
        let event: Event<serde_json::Value> = serde_json::from_value(event_data.clone())?;
        
        let payload = serde_json::to_vec(&event)?;
        
        self.channel.basic_publish(
            &self.exchange,
            routing_key,
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        ).await?;

        tracing::info!(
            "Event {} published to exchange '{}' with routing key '{}'",
            event.id,
            self.exchange,
            routing_key
        );

        Ok(())
    }
} 