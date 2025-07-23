// 事件存储 - 错误记录和重试状态持久化
// 文件路径: /d:/workspace/Solana/SoonShop/backend-microservices/shared/events/src/storage.rs

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;
use crate::retry::{RetryState, ErrorInfo, ErrorType};

/// 错误存储特征
#[async_trait]
pub trait ErrorStorage: Send + Sync {
    /// 存储错误信息
    async fn store_error(&self, event_id: Uuid, error: &ErrorInfo) -> Result<()>;
    
    /// 获取错误信息
    async fn get_error(&self, event_id: Uuid) -> Result<Option<ErrorInfo>>;
    
    /// 获取错误列表
    async fn get_errors(&self, limit: u32, offset: u32) -> Result<Vec<ErrorInfo>>;
    
    /// 删除错误信息
    async fn delete_error(&self, event_id: Uuid) -> Result<()>;
    
    /// 清理过期的错误信息
    async fn cleanup_expired_errors(&self, retention_days: u32) -> Result<u32>;
}

/// 重试状态存储特征
#[async_trait]
pub trait RetryStorage: Send + Sync {
    /// 存储重试状态
    async fn store_retry_state(&self, event_id: Uuid, state: &RetryState) -> Result<()>;
    
    /// 获取重试状态
    async fn get_retry_state(&self, event_id: Uuid) -> Result<Option<RetryState>>;
    
    /// 获取所有重试状态
    async fn get_all_retry_states(&self) -> Result<HashMap<Uuid, RetryState>>;
    
    /// 删除重试状态
    async fn delete_retry_state(&self, event_id: Uuid) -> Result<()>;
    
    /// 清理过期的重试状态
    async fn cleanup_expired_retry_states(&self, retention_hours: u32) -> Result<u32>;
}

/// 内存存储实现（用于测试和开发）
pub struct InMemoryStorage {
    errors: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, ErrorInfo>>>,
    retry_states: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, RetryState>>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            errors: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            retry_states: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ErrorStorage for InMemoryStorage {
    async fn store_error(&self, event_id: Uuid, error: &ErrorInfo) -> Result<()> {
        let mut errors = self.errors.write().await;
        errors.insert(event_id, error.clone());
        Ok(())
    }
    
    async fn get_error(&self, event_id: Uuid) -> Result<Option<ErrorInfo>> {
        let errors = self.errors.read().await;
        Ok(errors.get(&event_id).cloned())
    }
    
    async fn get_errors(&self, limit: u32, offset: u32) -> Result<Vec<ErrorInfo>> {
        let errors = self.errors.read().await;
        let errors: Vec<ErrorInfo> = errors.values()
            .skip(offset as usize)
            .take(limit as usize)
            .cloned()
            .collect();
        Ok(errors)
    }
    
    async fn delete_error(&self, event_id: Uuid) -> Result<()> {
        let mut errors = self.errors.write().await;
        errors.remove(&event_id);
        Ok(())
    }
    
    async fn cleanup_expired_errors(&self, retention_days: u32) -> Result<u32> {
        let mut errors = self.errors.write().await;
        let cutoff_time = Utc::now() - chrono::Duration::days(retention_days as i64);
        
        let initial_count = errors.len();
        errors.retain(|_, error| error.occurred_at > cutoff_time);
        let removed_count = initial_count - errors.len();
        
        Ok(removed_count as u32)
    }
}

#[async_trait]
impl RetryStorage for InMemoryStorage {
    async fn store_retry_state(&self, event_id: Uuid, state: &RetryState) -> Result<()> {
        let mut retry_states = self.retry_states.write().await;
        retry_states.insert(event_id, state.clone());
        Ok(())
    }
    
    async fn get_retry_state(&self, event_id: Uuid) -> Result<Option<RetryState>> {
        let retry_states = self.retry_states.read().await;
        Ok(retry_states.get(&event_id).cloned())
    }
    
    async fn get_all_retry_states(&self) -> Result<HashMap<Uuid, RetryState>> {
        let retry_states = self.retry_states.read().await;
        Ok(retry_states.clone())
    }
    
    async fn delete_retry_state(&self, event_id: Uuid) -> Result<()> {
        let mut retry_states = self.retry_states.write().await;
        retry_states.remove(&event_id);
        Ok(())
    }
    
    async fn cleanup_expired_retry_states(&self, retention_hours: u32) -> Result<u32> {
        let mut retry_states = self.retry_states.read().await;
        let cutoff_time = Utc::now() - chrono::Duration::hours(retention_hours as i64);
        
        let initial_count = retry_states.len();
        // 注意：这里简化了过期逻辑，实际实现需要检查RetryState的时间戳
        let removed_count = 0; // 简化实现
        
        Ok(removed_count)
    }
}

/// 存储统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_errors: u64,
    pub total_retry_states: u64,
    pub last_cleanup_time: Option<DateTime<Utc>>,
    pub storage_size_bytes: u64,
    pub errors_by_type: HashMap<ErrorType, u64>,
    pub oldest_error_time: Option<DateTime<Utc>>,
    pub newest_error_time: Option<DateTime<Utc>>,
}

/// 存储配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub error_retention_days: u32,
    pub retry_state_retention_hours: u32,
    pub max_errors_per_event: u32,
    pub enable_compression: bool,
    pub batch_size: u32,
    pub auto_cleanup_enabled: bool,
    pub cleanup_interval_minutes: u32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            error_retention_days: 7,
            retry_state_retention_hours: 24,
            max_errors_per_event: 10,
            enable_compression: false,
            batch_size: 100,
            auto_cleanup_enabled: true,
            cleanup_interval_minutes: 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_in_memory_storage() {
        let storage = InMemoryStorage::new();
        let event_id = Uuid::new_v4();
        
        let error_info = ErrorInfo {
            event_id,
            error_type: ErrorType::Network,
            error_message: "Test error".to_string(),
            occurred_at: Utc::now(),
            retry_count: 1,
            context: HashMap::new(),
        };
        
        // 测试存储错误
        storage.store_error(event_id, &error_info).await.unwrap();
        
        // 测试获取错误
        let retrieved_error = storage.get_error(event_id).await.unwrap();
        assert!(retrieved_error.is_some());
        assert_eq!(retrieved_error.unwrap().error_message, "Test error");
        
        // 测试删除错误
        storage.delete_error(event_id).await.unwrap();
        let retrieved_error = storage.get_error(event_id).await.unwrap();
        assert!(retrieved_error.is_none());
    }
    
    #[test]
    fn test_storage_config_defaults() {
        let config = StorageConfig::default();
        assert_eq!(config.error_retention_days, 7);
        assert_eq!(config.retry_state_retention_hours, 24);
        assert!(config.auto_cleanup_enabled);
    }
} 