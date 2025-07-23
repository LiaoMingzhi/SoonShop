// 认证相关工具 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub roles: Vec<String>,
}

impl AuthUser {
    pub fn new(id: Uuid, username: String, roles: Vec<String>) -> Self {
        Self { id, username, roles }
    }
    
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
} 