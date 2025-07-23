// 事件系统错误处理
use thiserror::Error;
use serde::{Serialize, Deserialize};

/// 事件系统错误
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum EventError {
    #[error("Connection error: {message}")]
    ConnectionError { message: String },
    
    #[error("Serialization error: {message}")]
    SerializationError { message: String },
    
    #[error("Publishing error: {message}")]
    PublishingError { message: String },
    
    #[error("Subscription error: {message}")]
    SubscriptionError { message: String },
    
    #[error("Retry exhausted: {message}")]
    RetryExhausted { message: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Timeout error: {message}")]
    TimeoutError { message: String },
    
    #[error("Storage error: {message}")]
    StorageError { message: String },
    
    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

impl EventError {
    /// 检查错误是否可重试
    pub fn is_retryable(&self) -> bool {
        match self {
            EventError::ConnectionError { .. } => true,
            EventError::PublishingError { .. } => true,
            EventError::TimeoutError { .. } => true,
            EventError::StorageError { .. } => true,
            EventError::SerializationError { .. } => false,
            EventError::SubscriptionError { .. } => false,
            EventError::RetryExhausted { .. } => false,
            EventError::ConfigurationError { .. } => false,
            EventError::Unknown { .. } => false,
        }
    }
    
    /// 获取错误代码
    pub fn error_code(&self) -> &'static str {
        match self {
            EventError::ConnectionError { .. } => "CONNECTION_ERROR",
            EventError::SerializationError { .. } => "SERIALIZATION_ERROR",
            EventError::PublishingError { .. } => "PUBLISHING_ERROR",
            EventError::SubscriptionError { .. } => "SUBSCRIPTION_ERROR",
            EventError::RetryExhausted { .. } => "RETRY_EXHAUSTED",
            EventError::ConfigurationError { .. } => "CONFIGURATION_ERROR",
            EventError::TimeoutError { .. } => "TIMEOUT_ERROR",
            EventError::StorageError { .. } => "STORAGE_ERROR",
            EventError::Unknown { .. } => "UNKNOWN_ERROR",
        }
    }
}

/// 从 anyhow::Error 转换为 EventError
impl From<anyhow::Error> for EventError {
    fn from(err: anyhow::Error) -> Self {
        EventError::Unknown {
            message: err.to_string(),
        }
    }
}

/// 从 serde_json::Error 转换为 EventError
impl From<serde_json::Error> for EventError {
    fn from(err: serde_json::Error) -> Self {
        EventError::SerializationError {
            message: err.to_string(),
        }
    }
}

/// 从 lapin::Error 转换为 EventError
impl From<lapin::Error> for EventError {
    fn from(err: lapin::Error) -> Self {
        EventError::ConnectionError {
            message: err.to_string(),
        }
    }
} 