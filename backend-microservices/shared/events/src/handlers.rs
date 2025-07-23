// 事件处理器 - 基本实现
use async_trait::async_trait;
use crate::{Event, EventHandler};

pub struct LoggingEventHandler;

#[async_trait]
impl<T> EventHandler<T> for LoggingEventHandler {
    async fn handle(&self, data: &T) -> anyhow::Result<()> {
        tracing::info!("Received event: {:?}", std::any::type_name::<T>());
        Ok(())
    }
} 