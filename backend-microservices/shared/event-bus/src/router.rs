use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use crate::event::Event;
use crate::handler::EventHandlerBox;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    pub event_type: String,
    pub queue: String,
    pub exchange: String,
    pub routing_key: String,
}

/// 事件路由器接口
pub trait EventRouter: Send + Sync {
    async fn route_event(&self, event_type: &str, event_data: &Value) -> Result<String>;
}

/// 基于规则的路由器
pub struct RuleBasedEventRouter {
    routing_rules: HashMap<String, String>,
    default_routing_key: String,
}

impl RuleBasedEventRouter {
    pub fn new(default_routing_key: String) -> Self {
        Self {
            routing_rules: HashMap::new(),
            default_routing_key,
        }
    }
    
    pub fn add_rule(&mut self, event_type: String, routing_key: String) {
        self.routing_rules.insert(event_type, routing_key);
    }
}

impl EventRouter for RuleBasedEventRouter {
    async fn route_event(&self, event_type: &str, _event_data: &Value) -> Result<String> {
        Ok(self.routing_rules
            .get(event_type)
            .cloned()
            .unwrap_or_else(|| self.default_routing_key.clone()))
    }
}

/// 主题路由器
pub struct TopicEventRouter {
    topic_prefix: String,
}

impl TopicEventRouter {
    pub fn new(topic_prefix: String) -> Self {
        Self { topic_prefix }
    }
}

impl EventRouter for TopicEventRouter {
    async fn route_event(&self, event_type: &str, _event_data: &Value) -> Result<String> {
        Ok(format!("{}.{}", self.topic_prefix, event_type))
    }
}

/// 智能路由器
pub struct SmartEventRouter {
    rule_router: RuleBasedEventRouter,
    topic_router: TopicEventRouter,
}

impl SmartEventRouter {
    pub fn new() -> Self {
        Self {
            rule_router: RuleBasedEventRouter::new("default".to_string()),
            topic_router: TopicEventRouter::new("events".to_string()),
        }
    }
}

impl EventRouter for SmartEventRouter {
    async fn route_event(&self, event_type: &str, event_data: &Value) -> Result<String> {
        // 优先使用基于规则的路由
        let rule_result = self.rule_router.route_event(event_type, event_data).await?;
        if rule_result != "default" {
            return Ok(rule_result);
        }
        
        // 回退到主题路由
        self.topic_router.route_event(event_type, event_data).await
    }
}

pub struct EventRouterRegistry {
    handlers: HashMap<String, Vec<EventHandlerBox>>,
    router: Box<dyn EventRouter>,
}

impl EventRouterRegistry {
    pub fn new(router: Box<dyn EventRouter>) -> Self {
        Self {
            handlers: HashMap::new(),
            router,
        }
    }

    pub fn register_handler(&mut self, event_type: String, handler: EventHandlerBox) {
        self.handlers
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(handler);
    }

    pub async fn route_and_handle(&self, event: &Event) -> Result<()> {
        let _queue = self.router.route_event(&event.event_type, &event.data).await?;
        
        if let Some(handlers) = self.handlers.get(&event.event_type) {
            for handler in handlers {
                handler.handle(event).await?;
            }
        }
        
        Ok(())
    }
} 