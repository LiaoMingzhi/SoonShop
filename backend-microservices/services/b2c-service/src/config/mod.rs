use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub rabbitmq: RabbitMQConfig,
    pub external_services: ExternalServicesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabbitMQConfig {
    pub url: String,
    pub exchange_name: String,
    pub queue_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServicesConfig {
    pub product_service_url: String,
    pub inventory_service_url: String,
    pub payment_service_url: String,
    pub user_service_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8010".to_string())
                    .parse()
                    .unwrap_or(8010),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgresql://user:pass@localhost/b2c_db".to_string()),
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()
                    .unwrap_or(20),
                timeout: env::var("DB_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                pool_size: env::var("REDIS_POOL_SIZE")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            rabbitmq: RabbitMQConfig {
                url: env::var("RABBITMQ_URL")
                    .unwrap_or_else(|_| "amqp://localhost:5672".to_string()),
                exchange_name: env::var("RABBITMQ_EXCHANGE")
                    .unwrap_or_else(|_| "soonshop_exchange".to_string()),
                queue_name: env::var("RABBITMQ_QUEUE")
                    .unwrap_or_else(|_| "b2c_queue".to_string()),
            },
            external_services: ExternalServicesConfig {
                product_service_url: env::var("PRODUCT_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8002".to_string()),
                inventory_service_url: env::var("INVENTORY_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8005".to_string()),
                payment_service_url: env::var("PAYMENT_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8004".to_string()),
                user_service_url: env::var("USER_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8001".to_string()),
            },
        };

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8010,
            },
            database: DatabaseConfig {
                url: "postgresql://user:pass@localhost/b2c_db".to_string(),
                max_connections: 20,
                timeout: 30,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
            },
            rabbitmq: RabbitMQConfig {
                url: "amqp://localhost:5672".to_string(),
                exchange_name: "soonshop_exchange".to_string(),
                queue_name: "b2c_queue".to_string(),
            },
            external_services: ExternalServicesConfig {
                product_service_url: "http://localhost:8002".to_string(),
                inventory_service_url: "http://localhost:8005".to_string(),
                payment_service_url: "http://localhost:8004".to_string(),
                user_service_url: "http://localhost:8001".to_string(),
            },
        }
    }
} 