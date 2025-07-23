use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::Result;
use crate::event::Event;
use serde_json::Value;

/// 事件处理器接口
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event_data: &Value) -> Result<()>;
    fn get_routing_key(&self) -> String;
}

pub type EventHandlerBox = Arc<dyn EventHandler>;

/// 默认事件处理器实现
pub struct DefaultEventHandler {
    routing_key: String,
}

impl DefaultEventHandler {
    pub fn new(routing_key: String) -> Self {
        Self { routing_key }
    }
}

impl EventHandler for DefaultEventHandler {
    async fn handle(&self, _event_data: &Value) -> Result<()> {
        // 默认处理：什么都不做
        tracing::info!("Event handled by default handler");
        Ok(())
    }
    
    fn get_routing_key(&self) -> String {
        self.routing_key.clone()
    }
}

pub struct EventHandlerRegistry {
    handlers: std::collections::HashMap<String, Vec<EventHandlerBox>>,
}

impl EventHandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, event_type: String, handler: EventHandlerBox) {
        self.handlers
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(handler);
    }

    pub fn get_handlers(&self, event_type: &str) -> Option<&Vec<EventHandlerBox>> {
        self.handlers.get(event_type)
    }
}

impl Default for EventHandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
} 