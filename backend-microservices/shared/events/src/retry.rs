// 事件重试策略和错误处理机制
// 文件路径: /d:/workspace/Solana/SoonShop/backend-microservices/shared/events/src/retry.rs

use std::time::Duration;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 重试策略类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryStrategy {
    /// 固定延迟重试
    FixedDelay {
        /// 延迟时间
        delay: Duration,
        /// 最大重试次数
        max_attempts: u32,
    },
    /// 指数退避重试
    ExponentialBackoff {
        /// 初始延迟
        initial_delay: Duration,
        /// 退避因子
        backoff_factor: f64,
        /// 最大延迟
        max_delay: Duration,
        /// 最大重试次数
        max_attempts: u32,
    },
    /// 自定义延迟重试
    CustomDelays {
        /// 延迟时间列表
        delays: Vec<Duration>,
    },
    /// 不重试
    NoRetry,
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::ExponentialBackoff {
            initial_delay: Duration::from_secs(1),
            backoff_factor: 2.0,
            max_delay: Duration::from_secs(300), // 5分钟
            max_attempts: 5,
        }
    }
}

impl RetryStrategy {
    /// 计算下一次重试的延迟时间
    pub fn calculate_delay(&self, attempt: u32) -> Option<Duration> {
        match self {
            RetryStrategy::FixedDelay { delay, max_attempts } => {
                if attempt < *max_attempts {
                    Some(*delay)
                } else {
                    None
                }
            }
            RetryStrategy::ExponentialBackoff {
                initial_delay,
                backoff_factor,
                max_delay,
                max_attempts,
            } => {
                if attempt < *max_attempts {
                    let delay_ms = (initial_delay.as_millis() as f64 * backoff_factor.powi(attempt as i32))
                        .min(max_delay.as_millis() as f64) as u64;
                    Some(Duration::from_millis(delay_ms))
                } else {
                    None
                }
            }
            RetryStrategy::CustomDelays { delays } => {
                delays.get(attempt as usize).copied()
            }
            RetryStrategy::NoRetry => None,
        }
    }

    /// 是否应该重试
    pub fn should_retry(&self, attempt: u32) -> bool {
        self.calculate_delay(attempt).is_some()
    }

    /// 获取最大重试次数
    pub fn max_attempts(&self) -> u32 {
        match self {
            RetryStrategy::FixedDelay { max_attempts, .. } => *max_attempts,
            RetryStrategy::ExponentialBackoff { max_attempts, .. } => *max_attempts,
            RetryStrategy::CustomDelays { delays } => delays.len() as u32,
            RetryStrategy::NoRetry => 0,
        }
    }
}

/// 错误类型分类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ErrorType {
    /// 临时错误（可重试）
    Transient,
    /// 永久错误（不可重试）
    Permanent,
    /// 网络错误
    Network,
    /// 序列化/反序列化错误
    Serialization,
    /// 业务逻辑错误
    Business,
    /// 系统错误
    System,
    /// 未知错误
    Unknown,
}

impl ErrorType {
    /// 判断错误是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(self, ErrorType::Transient | ErrorType::Network | ErrorType::System)
    }
}

/// 错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// 错误ID
    pub id: Uuid,
    /// 错误类型
    pub error_type: ErrorType,
    /// 错误消息
    pub message: String,
    /// 错误详情
    pub details: Option<String>,
    /// 错误发生时间
    pub occurred_at: DateTime<Utc>,
    /// 错误来源
    pub source: Option<String>,
    /// 错误栈跟踪
    pub stack_trace: Option<String>,
}

impl ErrorInfo {
    /// 创建新的错误信息
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            error_type,
            message,
            details: None,
            occurred_at: Utc::now(),
            source: None,
            stack_trace: None,
        }
    }

    /// 设置错误详情
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    /// 设置错误来源
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// 设置错误栈跟踪
    pub fn with_stack_trace(mut self, stack_trace: String) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    }
}

/// 重试状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryState {
    /// 尝试次数
    pub attempt: u32,
    /// 首次尝试时间
    pub first_attempt_at: DateTime<Utc>,
    /// 最后尝试时间
    pub last_attempt_at: DateTime<Utc>,
    /// 下次重试时间
    pub next_retry_at: Option<DateTime<Utc>>,
    /// 错误历史
    pub errors: Vec<ErrorInfo>,
    /// 是否已达到最大重试次数
    pub exhausted: bool,
}

impl RetryState {
    /// 创建新的重试状态
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            attempt: 0,
            first_attempt_at: now,
            last_attempt_at: now,
            next_retry_at: None,
            errors: Vec::new(),
            exhausted: false,
        }
    }

    /// 记录错误并更新重试状态
    pub fn record_error(&mut self, error: ErrorInfo, strategy: &RetryStrategy) {
        self.attempt += 1;
        self.last_attempt_at = Utc::now();
        self.errors.push(error.clone());

        // 计算下次重试时间
        if let Some(delay) = strategy.calculate_delay(self.attempt) {
            self.next_retry_at = Some(self.last_attempt_at + chrono::Duration::from_std(delay).unwrap_or_default());
        } else {
            self.next_retry_at = None;
            self.exhausted = true;
        }
    }

    /// 是否可以重试
    pub fn can_retry(&self, strategy: &RetryStrategy) -> bool {
        !self.exhausted && strategy.should_retry(self.attempt)
    }

    /// 是否到了重试时间
    pub fn should_retry_now(&self) -> bool {
        match self.next_retry_at {
            Some(next_retry) => Utc::now() >= next_retry,
            None => false,
        }
    }

    /// 获取总失败次数
    pub fn total_failures(&self) -> usize {
        self.errors.len()
    }

    /// 获取最后一个错误
    pub fn last_error(&self) -> Option<&ErrorInfo> {
        self.errors.last()
    }

    /// 重置重试状态
    pub fn reset(&mut self) {
        self.attempt = 0;
        self.first_attempt_at = Utc::now();
        self.last_attempt_at = Utc::now();
        self.next_retry_at = None;
        self.errors.clear();
        self.exhausted = false;
    }
}

/// 重试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// 重试策略
    pub strategy: RetryStrategy,
    /// 是否启用死信队列
    pub enable_dead_letter_queue: bool,
    /// 死信队列名称
    pub dead_letter_queue: Option<String>,
    /// 是否启用错误持久化
    pub enable_error_persistence: bool,
    /// 错误存储表名
    pub error_storage_table: Option<String>,
    /// 是否启用指标收集
    pub enable_metrics: bool,
    /// 指标前缀
    pub metrics_prefix: Option<String>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            strategy: RetryStrategy::default(),
            enable_dead_letter_queue: true,
            dead_letter_queue: Some("soonshop.dead_letter".to_string()),
            enable_error_persistence: true,
            error_storage_table: Some("event_errors".to_string()),
            enable_metrics: true,
            metrics_prefix: Some("soonshop_events".to_string()),
        }
    }
}

impl RetryConfig {
    /// 创建无重试配置
    pub fn no_retry() -> Self {
        Self {
            strategy: RetryStrategy::NoRetry,
            enable_dead_letter_queue: false,
            dead_letter_queue: None,
            enable_error_persistence: false,
            error_storage_table: None,
            enable_metrics: false,
            metrics_prefix: None,
        }
    }

    /// 创建简单重试配置
    pub fn simple_retry(max_attempts: u32, delay: Duration) -> Self {
        Self {
            strategy: RetryStrategy::FixedDelay {
                delay,
                max_attempts,
            },
            ..Self::default()
        }
    }

    /// 创建指数退避重试配置
    pub fn exponential_backoff(
        initial_delay: Duration,
        max_attempts: u32,
        backoff_factor: f64,
        max_delay: Duration,
    ) -> Self {
        Self {
            strategy: RetryStrategy::ExponentialBackoff {
                initial_delay,
                backoff_factor,
                max_delay,
                max_attempts,
            },
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_delay_strategy() {
        let strategy = RetryStrategy::FixedDelay {
            delay: Duration::from_secs(1),
            max_attempts: 3,
        };

        assert_eq!(strategy.calculate_delay(0), Some(Duration::from_secs(1)));
        assert_eq!(strategy.calculate_delay(1), Some(Duration::from_secs(1)));
        assert_eq!(strategy.calculate_delay(2), Some(Duration::from_secs(1)));
        assert_eq!(strategy.calculate_delay(3), None);
    }

    #[test]
    fn test_exponential_backoff_strategy() {
        let strategy = RetryStrategy::ExponentialBackoff {
            initial_delay: Duration::from_secs(1),
            backoff_factor: 2.0,
            max_delay: Duration::from_secs(10),
            max_attempts: 4,
        };

        assert_eq!(strategy.calculate_delay(0), Some(Duration::from_secs(1)));
        assert_eq!(strategy.calculate_delay(1), Some(Duration::from_secs(2)));
        assert_eq!(strategy.calculate_delay(2), Some(Duration::from_secs(4)));
        assert_eq!(strategy.calculate_delay(3), Some(Duration::from_secs(8)));
        assert_eq!(strategy.calculate_delay(4), None);
    }

    #[test]
    fn test_retry_state() {
        let mut state = RetryState::new();
        let strategy = RetryStrategy::FixedDelay {
            delay: Duration::from_secs(1),
            max_attempts: 2,
        };

        assert!(state.can_retry(&strategy));

        let error = ErrorInfo::new(ErrorType::Transient, "Test error".to_string());
        state.record_error(error, &strategy);

        assert_eq!(state.attempt, 1);
        assert!(state.can_retry(&strategy));
        assert!(!state.exhausted);

        let error2 = ErrorInfo::new(ErrorType::Transient, "Test error 2".to_string());
        state.record_error(error2, &strategy);

        assert_eq!(state.attempt, 2);
        assert!(!state.can_retry(&strategy));
        assert!(state.exhausted);
    }

    #[test]
    fn test_error_type_retryable() {
        assert!(ErrorType::Transient.is_retryable());
        assert!(ErrorType::Network.is_retryable());
        assert!(ErrorType::System.is_retryable());
        assert!(!ErrorType::Permanent.is_retryable());
        assert!(!ErrorType::Serialization.is_retryable());
        assert!(!ErrorType::Business.is_retryable());
    }
} 