use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub elasticsearch_url: String,
    pub port: u16,
    pub rabbitmq_url: String,
    pub service_name: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://user:pass@localhost/product_db".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            elasticsearch_url: env::var("ELASTICSEARCH_URL")
                .unwrap_or_else(|_| "http://localhost:9200".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8002".to_string())
                .parse()?,
            rabbitmq_url: env::var("RABBITMQ_URL")
                                 .unwrap_or_else(|_| "amqp://localhost:5672".to_string()),
            service_name: env::var("SERVICE_NAME")
                .unwrap_or_else(|_| "product-service".to_string()),
        })
    }
} 