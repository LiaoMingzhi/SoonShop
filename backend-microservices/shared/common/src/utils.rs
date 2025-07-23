use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

impl PaginationRequest {
    pub fn normalize(&self) -> (u64, u64) {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit.unwrap_or(20).min(100).max(1);
        (page, limit)
    }
    
    pub fn offset(&self) -> u64 {
        let (page, limit) = self.normalize();
        (page - 1) * limit
    }
}

// 生成唯一ID
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

// 时间戳工具
pub fn current_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
} 