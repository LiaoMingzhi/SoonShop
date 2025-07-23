use std::collections::HashMap;
use std::time::Duration;
use serde_json::Value;
use reqwest::Client;
use tokio::time::{sleep, timeout};
use uuid::Uuid;

pub mod test_client;
pub mod test_data;
pub mod test_containers;
pub mod assertions;

pub use test_client::*;
pub use test_data::*;
pub use test_containers::*;
pub use assertions::*;

/// 测试配置
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub api_gateway_url: String,
    pub user_service_url: String,
    pub product_service_url: String,
    pub order_service_url: String,
    pub payment_service_url: String,
    pub inventory_service_url: String,
    pub notification_service_url: String,
    pub database_url: String,
    pub redis_url: String,
    pub rabbitmq_url: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            api_gateway_url: "http://localhost:8000".to_string(),
            user_service_url: "http://localhost:8001".to_string(),
            product_service_url: "http://localhost:8002".to_string(),
            order_service_url: "http://localhost:8003".to_string(),
            payment_service_url: "http://localhost:8004".to_string(),
            inventory_service_url: "http://localhost:8009".to_string(),
            notification_service_url: "http://localhost:8008".to_string(),
            database_url: "postgres://soonshop:soonshop123@localhost:5432/soonshop_test".to_string(),
            redis_url: "redis://localhost:6379/1".to_string(),
            rabbitmq_url: "amqp://soonshop:soonshop123@localhost:5672".to_string(),
        }
    }
}

/// 测试环境管理器
pub struct TestEnvironment {
    pub config: TestConfig,
    pub client: Client,
    pub auth_token: Option<String>,
}

impl TestEnvironment {
    /// 创建新的测试环境
    pub fn new() -> Self {
        Self {
            config: TestConfig::default(),
            client: Client::new(),
            auth_token: None,
        }
    }

    /// 等待服务启动
    pub async fn wait_for_services(&self) -> Result<(), Box<dyn std::error::Error>> {
        let services = vec![
            (&self.config.api_gateway_url, "/health"),
            (&self.config.user_service_url, "/health"),
            (&self.config.product_service_url, "/health"),
            (&self.config.order_service_url, "/health"),
            (&self.config.payment_service_url, "/health"),
            (&self.config.inventory_service_url, "/health"),
            (&self.config.notification_service_url, "/health"),
        ];

        for (base_url, health_path) in services {
            self.wait_for_service(base_url, health_path).await?;
        }

        Ok(())
    }

    /// 等待单个服务启动
    async fn wait_for_service(&self, base_url: &str, health_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}{}", base_url, health_path);
        let max_attempts = 30;
        let delay = Duration::from_secs(2);

        for attempt in 1..=max_attempts {
            match timeout(Duration::from_secs(5), self.client.get(&url).send()).await {
                Ok(Ok(response)) if response.status().is_success() => {
                    println!("✅ Service {} is ready", base_url);
                    return Ok(());
                }
                _ => {
                    if attempt == max_attempts {
                        return Err(format!("Service {} failed to start after {} attempts", base_url, max_attempts).into());
                    }
                    println!("⏳ Waiting for service {} (attempt {}/{})", base_url, attempt, max_attempts);
                    sleep(delay).await;
                }
            }
        }

        Ok(())
    }

    /// 登录并获取认证令牌
    pub async fn login(&mut self, email: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let login_data = serde_json::json!({
            "email": email,
            "password": password
        });

        let response = self.client
            .post(&format!("{}/api/auth/login", self.config.api_gateway_url))
            .json(&login_data)
            .send()
            .await?;

        let body: Value = response.json().await?;
        let token = body["data"]["token"].as_str()
            .ok_or("Token not found in response")?
            .to_string();

        self.auth_token = Some(token.clone());
        Ok(token)
    }

    /// 创建测试用户
    pub async fn create_test_user(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let user_data = serde_json::json!({
            "username": format!("test_user_{}", Uuid::new_v4()),
            "email": format!("test_{}@example.com", Uuid::new_v4()),
            "password": "test_password_123",
            "full_name": "测试用户"
        });

        let response = self.client
            .post(&format!("{}/api/auth/register", self.config.api_gateway_url))
            .json(&user_data)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    /// 获取认证头
    pub fn auth_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        if let Some(token) = &self.auth_token {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }
        headers
    }

    /// 清理测试数据
    pub async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 这里可以添加清理逻辑，比如删除测试数据
        // 由于是集成测试，通常使用专门的测试数据库
        println!("🧹 Cleaning up test data...");
        Ok(())
    }
}

/// 测试断言宏
#[macro_export]
macro_rules! assert_status {
    ($response:expr, $expected:expr) => {
        assert_eq!($response.status(), $expected, "Unexpected status code");
    };
}

#[macro_export]
macro_rules! assert_json_field {
    ($json:expr, $field:expr, $expected:expr) => {
        assert_eq!($json[$field], $expected, "JSON field {} mismatch", $field);
    };
}

#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!($haystack.contains($needle), "String does not contain expected substring");
    };
}

/// 生成随机测试数据
pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// 生成随机价格
pub fn generate_random_price() -> i64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(100..100000) // 1.00 到 1000.00
} 