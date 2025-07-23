use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventBusError {
    ConnectionError(String),
    PublishError(String),
    SubscribeError(String),
    RouterError(String),
    HandlerError(String),
    ConfigurationError(String),
    SerializationError(String),
    ValidationError(String),
    TimeoutError(String),
    Other(String),
}

impl fmt::Display for EventBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventBusError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            EventBusError::PublishError(msg) => write!(f, "Publish error: {}", msg),
            EventBusError::SubscribeError(msg) => write!(f, "Subscribe error: {}", msg),
            EventBusError::RouterError(msg) => write!(f, "Router error: {}", msg),
            EventBusError::HandlerError(msg) => write!(f, "Handler error: {}", msg),
            EventBusError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            EventBusError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            EventBusError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            EventBusError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            EventBusError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for EventBusError {}

impl From<anyhow::Error> for EventBusError {
    fn from(err: anyhow::Error) -> Self {
        EventBusError::Other(err.to_string())
    }
}

impl From<serde_json::Error> for EventBusError {
    fn from(err: serde_json::Error) -> Self {
        EventBusError::SerializationError(err.to_string())
    }
}

impl From<lapin::Error> for EventBusError {
    fn from(err: lapin::Error) -> Self {
        EventBusError::ConnectionError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, EventBusError>; 