// 监控错误类型模块
// 这里可以添加监控相关的错误类型

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonitoringError {
    #[error("Metrics collection failed: {0}")]
    MetricsError(String),
    
    #[error("Alert delivery failed: {0}")]
    AlertError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
} 