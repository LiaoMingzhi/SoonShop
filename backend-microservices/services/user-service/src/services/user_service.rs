use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, error};

use crate::models::{CreateUserRequest, UpdateUserRequest, LoginRequest, LoginResponse, User, UserWithPassword};
use crate::repositories::user_repository::UserRepositoryTrait;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepositoryTrait>,
    jwt_secret: String,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryTrait>, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    pub async fn register_user(&self, request: CreateUserRequest) -> Result<User> {
        // 检查用户是否已存在
        if let Some(_existing_user) = self.user_repository.get_user_by_email(&request.email).await? {
            return Err(anyhow::anyhow!("User with this email already exists"));
        }

        // 哈希密码
        let password_hash = hash(&request.password, DEFAULT_COST)?;

        // 创建用户
        let user_with_password = self.user_repository.create_user(request, password_hash).await?;

        info!("User registered successfully: {}", user_with_password.email);
        
        Ok(user_with_password.into())
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        // 通过用户名查找用户
        let user = self.user_repository.get_user_by_username(&request.username).await?
            .ok_or_else(|| anyhow::anyhow!("Invalid username or password"))?;

        // 验证密码
        if !verify(&request.password, &user.password_hash)? {
            return Err(anyhow::anyhow!("Invalid username or password"));
        }

        // 生成JWT token
        let token = self.generate_jwt_token(&user)?;

        info!("User logged in successfully: {}", user.email);

        Ok(LoginResponse {
            token,
            user: user.into(),
        })
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = self.user_repository.get_user_by_id(user_id).await?;
        Ok(user.map(|u| u.into()))
    }

    pub async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<Option<User>> {
        let user = self.user_repository.update_user(user_id, request).await?;
        Ok(user.map(|u| u.into()))
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<bool> {
        let success = self.user_repository.delete_user(user_id).await?;
        if success {
            info!("User deleted successfully: {}", user_id);
        }
        Ok(success)
    }

    pub async fn list_users(&self, page: u64, page_size: u64) -> Result<Vec<User>> {
        let users = self.user_repository.list_users(page, page_size).await?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn count_users(&self) -> Result<u64> {
        self.user_repository.count_users().await
    }

    pub fn verify_jwt_token(&self, token: &str) -> Result<Claims> {
        let key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<Claims>(token, &key, &validation)?;
        Ok(token_data.claims)
    }

    fn generate_jwt_token(&self, user: &UserWithPassword) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: expiration,
        };

        let key = EncodingKey::from_secret(self.jwt_secret.as_ref());
        let token = encode(&Header::default(), &claims, &key)?;

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use uuid::Uuid;
    use chrono::Utc;

    // Mock repository for testing
    struct MockUserRepository {
        users: HashMap<Uuid, User>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }
    }

    #[async_trait]
    impl UserRepositoryTrait for MockUserRepository {
        async fn create_user(&self, _user_data: CreateUserRequest, _password_hash: String) -> Result<User> {
            let user = User {
                id: Uuid::new_v4(),
                email: "test@example.com".to_string(),
                password_hash: "hashed_password".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                phone: None,
                wallet_address: None,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            Ok(user)
        }

        async fn get_user_by_id(&self, _user_id: Uuid) -> Result<Option<User>> {
            Ok(None)
        }

        async fn get_user_by_email(&self, _email: &str) -> Result<Option<User>> {
            Ok(None)
        }

        async fn get_user_by_wallet_address(&self, _wallet_address: &str) -> Result<Option<User>> {
            Ok(None)
        }

        async fn update_user(&self, _user_id: Uuid, _user_data: UpdateUserRequest) -> Result<Option<User>> {
            Ok(None)
        }

        async fn delete_user(&self, _user_id: Uuid) -> Result<bool> {
            Ok(true)
        }

        async fn list_users(&self, _page: u64, _page_size: u64) -> Result<Vec<User>> {
            Ok(vec![])
        }

        async fn count_users(&self) -> Result<u64> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_register_user() {
        let mock_repo = Arc::new(MockUserRepository::new());
        let service = UserService::new(mock_repo, "test_secret".to_string());

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: None,
            wallet_address: None,
        };

        let result = service.register_user(request).await;
        assert!(result.is_ok());
    }
} 