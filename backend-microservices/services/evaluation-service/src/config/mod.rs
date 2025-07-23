use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Config {
            server: ServerConfig {
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8008".to_string())
                    .parse()?,
                host: env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
            },
            database: DatabaseConfig {
                host: env::var("DATABASE_HOST")
                    .unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("DATABASE_PORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()?,
                username: env::var("DATABASE_USERNAME")
                    .unwrap_or_else(|_| "postgres".to_string()),
                password: env::var("DATABASE_PASSWORD")
                    .unwrap_or_else(|_| "password".to_string()),
                name: env::var("DATABASE_NAME")
                    .unwrap_or_else(|_| "soonshop".to_string()),
            },
        })
    }
} 