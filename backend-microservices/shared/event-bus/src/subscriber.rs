use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use crate::event::Event;
use crate::handler::EventHandlerBox;
use serde_json::Value;
use uuid::Uuid;

use crate::handler::EventHandler;

/// 事件订阅器接口
#[async_trait]
pub trait EventSubscriber: Send + Sync {
    async fn subscribe(&self, event_type: String, handler: Box<dyn EventHandler>) -> Result<String>;
    async fn unsubscribe(&self, subscription_id: &str) -> Result<()>;
}

/// 默认事件订阅器
pub struct DefaultEventSubscriber {
    subscriptions: HashMap<String, (String, Box<dyn EventHandler>)>,
}

impl DefaultEventSubscriber {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }
}

#[async_trait]
impl EventSubscriber for DefaultEventSubscriber {
    async fn subscribe(&self, event_type: String, _handler: Box<dyn EventHandler>) -> Result<String> {
        let subscription_id = Uuid::new_v4().to_string();
        tracing::info!("Subscribed to {} with id {}", event_type, subscription_id);
        Ok(subscription_id)
    }
    
    async fn unsubscribe(&self, subscription_id: &str) -> Result<()> {
        tracing::info!("Unsubscribed {}", subscription_id);
        Ok(())
    }
}

pub struct EventSubscriberRegistry {
    subscribers: HashMap<String, Box<dyn EventSubscriber>>,
}

impl EventSubscriberRegistry {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub fn register_subscriber(&mut self, name: String, subscriber: Box<dyn EventSubscriber>) {
        self.subscribers.insert(name, subscriber);
    }

    pub async fn subscribe_all(&self, event_type: &str, handler: EventHandlerBox) -> Result<()> {
        for subscriber in self.subscribers.values() {
            subscriber.subscribe(event_type.to_string(), handler.clone()).await?;
        }
        Ok(())
    }

    pub async fn start_all(&self) -> Result<()> {
        for subscriber in self.subscribers.values() {
            subscriber.start().await?;
        }
        Ok(())
    }

    pub async fn stop_all(&self) -> Result<()> {
        for subscriber in self.subscribers.values() {
            subscriber.stop().await?;
        }
        Ok(())
    }
}

impl Default for EventSubscriberRegistry {
    fn default() -> Self {
        Self::new()
    }
} 