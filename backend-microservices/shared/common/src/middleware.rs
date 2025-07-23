// Actix-web中间件相关工具 - 基本实现
use actix_web::{dev::ServiceRequest, Error, HttpMessage};

// 请求ID中间件
pub fn get_request_id(req: &ServiceRequest) -> String {
    req.extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
} 