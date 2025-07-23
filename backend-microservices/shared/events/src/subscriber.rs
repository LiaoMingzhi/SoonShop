// 事件订阅者 - RabbitMQ实现
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use lapin::{Connection, ConnectionProperties, Channel, Consumer};
use lapin::options::{ExchangeDeclareOptions, QueueDeclareOptions, QueueBindOptions, BasicConsumeOptions, BasicAckOptions, BasicNackOptions};
use lapin::types::FieldTable;
use lapin::ExchangeKind;
use futures::StreamExt;
use crate::{EventSubscriber, EventHandler, Event, EventJsonHandler};

pub struct RabbitMQEventSubscriber {
    channel: Channel,
    exchange: String,
    queue_prefix: String,
}

impl RabbitMQEventSubscriber {
    pub async fn new(rabbitmq_url: &str, exchange: &str, queue_prefix: &str) -> anyhow::Result<Self> {
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
            queue_prefix: queue_prefix.to_string(),
        })
    }
}

#[async_trait]
impl EventSubscriber for RabbitMQEventSubscriber {
    async fn subscribe_json(&self, routing_key: &str, handler: Box<dyn EventJsonHandler>) -> anyhow::Result<()> {
        let queue_name = format!("{}_{}", self.queue_prefix, routing_key.replace(".", "_"));
        
        // 声明队列
        self.channel.queue_declare(
            &queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        ).await?;

        // 绑定队列到交换机
        self.channel.queue_bind(
            &queue_name,
            &self.exchange,
            routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        ).await?;

        // 创建消费者
        let consumer = self.channel.basic_consume(
            &queue_name,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ).await?;

        // 启动消费者处理循环
        tokio::spawn(async move {
            consumer.for_each(|delivery| async {
                if let Ok(delivery) = delivery {
                    match serde_json::from_slice::<serde_json::Value>(&delivery.data) {
                        Ok(event_data) => {
                            if let Err(e) = handler.handle_json(&event_data).await {
                                tracing::error!("Error handling event: {}", e);
                                // 拒绝消息，不重新排队
                                if let Err(nack_err) = delivery.nack(BasicNackOptions::default()).await {
                                    tracing::error!("Error nacking message: {}", nack_err);
                                }
                            } else {
                                // 确认消息
                                if let Err(ack_err) = delivery.ack(BasicAckOptions::default()).await {
                                    tracing::error!("Error acking message: {}", ack_err);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error deserializing event: {}", e);
                            // 拒绝无法反序列化的消息
                            if let Err(nack_err) = delivery.nack(BasicNackOptions::default()).await {
                                tracing::error!("Error nacking message: {}", nack_err);
                            }
                        }
                    }
                }
            }).await;
        });

        tracing::info!("Subscribed to routing key '{}' on queue '{}'", routing_key, queue_name);
        Ok(())
    }
} 