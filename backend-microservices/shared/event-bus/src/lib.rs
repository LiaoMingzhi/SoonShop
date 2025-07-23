pub mod config;
pub mod error;
pub mod event;

// 重新导出核心类型
pub use config::EventBusConfig;
pub use error::EventBusError;
pub use event::{Event, EventMetadata, EventEnvelope};

// 简化的事件总线
use anyhow::Result;
use serde_json::Value;

/// 简化的事件总线
pub struct SimpleEventBus {
    config: EventBusConfig,
}

impl SimpleEventBus {
    pub fn new(config: EventBusConfig) -> Self {
        Self { config }
    }
    
    pub async fn publish(&self, event_type: &str, payload: Value) -> Result<()> {
        tracing::info!("Publishing event: {}", event_type);
        // TODO: 实现实际的发布逻辑
        Ok(())
    }
    
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Event bus started");
        Ok(())
    }
    
    pub async fn stop(&self) -> Result<()> {
        tracing::info!("Event bus stopped");
        Ok(())
    }
    
    pub fn health_check(&self) -> bool {
        true
    }
} 