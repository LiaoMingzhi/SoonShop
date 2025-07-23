use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::Result;
use serde_json::Value;
use uuid::Uuid;

/// 存储的事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub id: Uuid,
    pub event_type: String,
    pub data: Value,
    pub timestamp: DateTime<Utc>,
}

/// 事件存储接口
#[async_trait]
pub trait EventStorage: Send + Sync {
    async fn store_event(&self, event: StoredEvent) -> Result<()>;
    async fn get_event(&self, event_id: Uuid) -> Result<Option<StoredEvent>>;
    async fn get_events_by_type(&self, event_type: &str) -> Result<Vec<StoredEvent>>;
    async fn get_events_for_aggregate(&self, aggregate_id: Uuid) -> Result<Vec<StoredEvent>>;
}

/// 内存事件存储
pub struct InMemoryEventStorage {
    events: std::sync::RwLock<Vec<StoredEvent>>,
}

impl InMemoryEventStorage {
    pub fn new() -> Self {
        Self {
            events: std::sync::RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl EventStorage for InMemoryEventStorage {
    async fn store_event(&self, event: StoredEvent) -> Result<()> {
        let mut events = self.events.write().unwrap();
        events.push(event);
        Ok(())
    }
    
    async fn get_event(&self, event_id: Uuid) -> Result<Option<StoredEvent>> {
        let events = self.events.read().unwrap();
        Ok(events.iter().find(|e| e.id == event_id).cloned())
    }
    
    async fn get_events_by_type(&self, event_type: &str) -> Result<Vec<StoredEvent>> {
        let events = self.events.read().unwrap();
        Ok(events.iter()
            .filter(|e| e.event_type == event_type)
            .cloned()
            .collect())
    }
    
    async fn get_events_for_aggregate(&self, _aggregate_id: Uuid) -> Result<Vec<StoredEvent>> {
        // 简化实现：返回所有事件
        let events = self.events.read().unwrap();
        Ok(events.clone())
    }
} 