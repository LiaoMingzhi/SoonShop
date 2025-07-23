use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBusConfig {
    pub rabbitmq: RabbitMQConfig,
    pub redis: RedisConfig,
    pub retry: RetryConfig,
    pub metrics: MetricsConfig,
    pub tracing: TracingConfig,
    pub exchange_name: String,
    pub service_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabbitMQConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub virtual_host: String,
    pub connection_timeout: Duration,
    pub heartbeat: Duration,
    pub max_channels: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub database: u8,
    pub pool_size: u32,
    pub connection_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub exponential_backoff: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub level: String,
    pub format: String,
}

impl Default for EventBusConfig {
    fn default() -> Self {
        Self {
            rabbitmq: RabbitMQConfig::default(),
            redis: RedisConfig::default(),
            retry: RetryConfig::default(),
            metrics: MetricsConfig::default(),
            tracing: TracingConfig::default(),
            exchange_name: "events".to_string(),
            service_name: "event-bus".to_string(),
        }
    }
}

impl Default for RabbitMQConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5672,
            username: "guest".to_string(),
            password: "guest".to_string(),
            virtual_host: "/".to_string(),
            connection_timeout: Duration::from_secs(30),
            heartbeat: Duration::from_secs(60),
            max_channels: 100,
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 6379,
            password: None,
            database: 0,
            pool_size: 10,
            connection_timeout: Duration::from_secs(5),
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            exponential_backoff: true,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(3600 * 24), // 24 hours
        }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: "info".to_string(),
            format: "json".to_string(),
        }
    }
}

impl EventBusConfig {
    pub fn rabbitmq_url(&self) -> String {
        format!("amqp://{}:{}@{}:{}/{}", 
            self.rabbitmq.username, 
            self.rabbitmq.password, 
            self.rabbitmq.host, 
            self.rabbitmq.port,
            self.rabbitmq.virtual_host.trim_start_matches('/'))
    }
    
    pub fn redis_url(&self) -> String {
        if let Some(password) = &self.redis.password {
            format!("redis://:{}@{}:{}/{}", password, self.redis.host, self.redis.port, self.redis.database)
        } else {
            format!("redis://{}:{}/{}", self.redis.host, self.redis.port, self.redis.database)
        }
    }
} 