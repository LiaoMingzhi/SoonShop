use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub rabbitmq_url: String,
    pub apns: ApnsConfig,
    pub fcm: FcmConfig,
    pub email: EmailConfig,
    pub sms: SmsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApnsConfig {
    pub key_id: String,
    pub team_id: String,
    pub private_key_path: String,
    pub sandbox: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FcmConfig {
    pub server_key: String,
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsConfig {
    pub provider: String,
    pub api_key: String,
    pub from_number: String,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8008,
            database_url: "postgres://localhost:5432/notifications".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            rabbitmq_url: "amqp://localhost:5672".to_string(),
            apns: ApnsConfig::default(),
            fcm: FcmConfig::default(),
            email: EmailConfig::default(),
            sms: SmsConfig::default(),
        }
    }
}

impl Default for ApnsConfig {
    fn default() -> Self {
        Self {
            key_id: "".to_string(),
            team_id: "".to_string(),
            private_key_path: "".to_string(),
            sandbox: true,
        }
    }
}

impl Default for FcmConfig {
    fn default() -> Self {
        Self {
            server_key: "".to_string(),
            project_id: "".to_string(),
        }
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_host: "localhost".to_string(),
            smtp_port: 587,
            username: "".to_string(),
            password: "".to_string(),
            from_email: "noreply@soonshop.com".to_string(),
        }
    }
}

impl Default for SmsConfig {
    fn default() -> Self {
        Self {
            provider: "twilio".to_string(),
            api_key: "".to_string(),
            from_number: "".to_string(),
        }
    }
} 