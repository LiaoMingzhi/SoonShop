use serde::{Deserialize, Serialize};
use std::env;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub redis_url: String,
    pub jaeger_endpoint: String,
    pub rate_limit: RateLimitConfig,
    pub services: ServiceConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub user_service: String,
    pub product_service: String,
    pub order_service: String,
    pub payment_service: String,
    pub voucher_service: String,
    pub reward_service: String,
    pub evaluation_service: String,
    pub notification_service: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(AppConfig {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .unwrap_or(8000),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jaeger_endpoint: env::var("JAEGER_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:14268/api/traces".to_string()),
            rate_limit: RateLimitConfig {
                requests_per_minute: env::var("RATE_LIMIT_RPM")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()
                    .unwrap_or(60),
                burst_size: env::var("RATE_LIMIT_BURST")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            services: ServiceConfig {
                user_service: env::var("USER_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8001".to_string()),
                product_service: env::var("PRODUCT_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8002".to_string()),
                order_service: env::var("ORDER_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8003".to_string()),
                payment_service: env::var("PAYMENT_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8004".to_string()),
                voucher_service: env::var("VOUCHER_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8005".to_string()),
                reward_service: env::var("REWARD_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8006".to_string()),
                evaluation_service: env::var("EVALUATION_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8007".to_string()),
                notification_service: env::var("NOTIFICATION_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8008".to_string()),
            },
            cors: CorsConfig {
                allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
                    .split(',')
                    .map(|s| s.to_string())
                    .collect(),
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                allowed_headers: vec![
                    "Content-Type".to_string(),
                    "Authorization".to_string(),
                    "X-Requested-With".to_string(),
                    "Accept".to_string(),
                ],
                max_age: 3600,
            },
        })
    }
} 