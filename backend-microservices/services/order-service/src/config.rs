use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8003".to_string())
            .parse::<u16>()?;
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/orders".to_string());
        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());
        
        Ok(Self {
            server: ServerConfig { host, port },
            database: DatabaseConfig {
                url: database_url,
                max_connections: 10,
            },
            redis: RedisConfig { url: redis_url },
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8003,
            },
            database: DatabaseConfig {
                url: "postgresql://localhost/orders".to_string(),
                max_connections: 10,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
            },
        }
    }
} 