// 事件系统配置
use serde::{Serialize, Deserialize};
use std::time::Duration;

/// 事件系统配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfig {
    /// 服务名称
    pub service_name: String,
    /// RabbitMQ连接URL
    pub rabbitmq_url: String,
    /// 交换机名称
    pub exchange_name: String,
    /// 队列前缀
    pub queue_prefix: String,
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试延迟（毫秒）
    pub retry_delay_ms: u64,
    /// 是否启用死信队列
    pub enable_dead_letter_queue: bool,
    /// 死信队列名称
    pub dead_letter_queue: Option<String>,
    /// 是否启用指标收集
    pub enable_metrics: bool,
    /// 是否启用错误存储
    pub enable_error_storage: bool,
    /// 连接超时（秒）
    pub connection_timeout_secs: u64,
    /// 发布超时（秒）
    pub publish_timeout_secs: u64,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            service_name: "soonshop".to_string(),
            rabbitmq_url: "amqp://localhost:5672".to_string(),
            exchange_name: "soonshop.events".to_string(),
            queue_prefix: "soonshop".to_string(),
            max_retries: 3,
            retry_delay_ms: 1000,
            enable_dead_letter_queue: true,
            dead_letter_queue: Some("soonshop.events.dlq".to_string()),
            enable_metrics: true,
            enable_error_storage: true,
            connection_timeout_secs: 30,
            publish_timeout_secs: 10,
        }
    }
}

impl EventConfig {
    /// 获取重试延迟
    pub fn retry_delay(&self) -> Duration {
        Duration::from_millis(self.retry_delay_ms)
    }
    
    /// 获取连接超时
    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_secs)
    }
    
    /// 获取发布超时
    pub fn publish_timeout(&self) -> Duration {
        Duration::from_secs(self.publish_timeout_secs)
    }
} 