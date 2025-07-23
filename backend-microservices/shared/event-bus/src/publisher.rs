use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use crate::event::Event;
use serde_json::Value;
use std::sync::Arc;

/// 事件发布器接口
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event_type: &str, event_data: &Value) -> Result<()>;
}

/// 默认事件发布器
pub struct DefaultEventPublisher {
    name: String,
}

impl DefaultEventPublisher {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl EventPublisher for DefaultEventPublisher {
    async fn publish(&self, event_type: &str, _event_data: &Value) -> Result<()> {
        tracing::info!("Publishing event {} via {}", event_type, self.name);
        Ok(())
    }
}

/// 多发布器管理器
pub struct MultiEventPublisher {
    publishers: HashMap<String, Arc<dyn EventPublisher>>,
}

impl MultiEventPublisher {
    pub fn new() -> Self {
        Self {
            publishers: HashMap::new(),
        }
    }
    
    pub fn add_publisher(&mut self, name: String, publisher: Arc<dyn EventPublisher>) {
        self.publishers.insert(name, publisher);
    }
    
    pub async fn publish_to(&self, publisher_name: &str, event_type: &str, event_data: &Value) -> Result<()> {
        if let Some(publisher) = self.publishers.get(publisher_name) {
            publisher.publish(event_type, event_data).await
        } else {
            Err(anyhow::anyhow!("Publisher {} not found", publisher_name))
        }
    }
    
    pub async fn broadcast(&self, event_type: &str, event_data: &Value) -> Result<()> {
        for (name, publisher) in &self.publishers {
            if let Err(e) = publisher.publish(event_type, event_data).await {
                tracing::error!("Failed to publish to {}: {}", name, e);
            }
        }
        Ok(())
    }
}

impl Default for MultiEventPublisher {
    fn default() -> Self {
        Self::new()
    }
} 