use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use sea_orm::DbErr;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] DbErr),
    
    #[error("Redis错误: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("HTTP客户端错误: {0}")]
    HttpClient(#[from] reqwest::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("配置错误: {0}")]
    Config(String),
    
    #[error("验证错误: {0}")]
    Validation(String),
    
    #[error("认证错误: {0}")]
    Authentication(String),
    
    #[error("授权错误: {0}")]
    Authorization(String),
    
    #[error("资源未找到: {resource_type} ID: {id}")]
    NotFound { resource_type: String, id: Uuid },
    
    #[error("资源已存在: {resource_type} {field}: {value}")]
    AlreadyExists { resource_type: String, field: String, value: String },
    
    #[error("业务逻辑错误: {0}")]
    BusinessLogic(String),
    
    #[error("外部服务错误: {service}: {message}")]
    ExternalService { service: String, message: String },
    
    #[error("内部服务器错误: {0}")]
    Internal(String),
}

impl AppError {
    pub fn not_found(resource_type: &str, id: Uuid) -> Self {
        Self::NotFound {
            resource_type: resource_type.to_string(),
            id,
        }
    }
    
    pub fn already_exists(resource_type: &str, field: &str, value: &str) -> Self {
        Self::AlreadyExists {
            resource_type: resource_type.to_string(),
            field: field.to_string(),
            value: value.to_string(),
        }
    }
    
    pub fn validation(message: &str) -> Self {
        Self::Validation(message.to_string())
    }
    
    pub fn business_logic(message: &str) -> Self {
        Self::BusinessLogic(message.to_string())
    }
    
    pub fn external_service(service: &str, message: &str) -> Self {
        Self::ExternalService {
            service: service.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            message: message.to_string(),
            details: None,
            request_id: None,
        }
    }
    
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
    
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Validation(msg) => ErrorResponse::new("VALIDATION_ERROR", &msg),
            AppError::NotFound { resource_type, id } => {
                ErrorResponse::new("NOT_FOUND", &format!("{} with ID {} not found", resource_type, id))
            }
            AppError::AlreadyExists { resource_type, field, value } => {
                ErrorResponse::new("ALREADY_EXISTS", &format!("{} with {} {} already exists", resource_type, field, value))
            }
            AppError::Authentication(msg) => ErrorResponse::new("AUTHENTICATION_ERROR", &msg),
            AppError::Authorization(msg) => ErrorResponse::new("AUTHORIZATION_ERROR", &msg),
            AppError::BusinessLogic(msg) => ErrorResponse::new("BUSINESS_LOGIC_ERROR", &msg),
            AppError::ExternalService { service, message } => {
                ErrorResponse::new("EXTERNAL_SERVICE_ERROR", &format!("{}: {}", service, message))
            }
            _ => ErrorResponse::new("INTERNAL_SERVER_ERROR", "An internal server error occurred"),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>; 