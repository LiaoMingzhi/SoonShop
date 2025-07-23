use actix_web::{web, HttpRequest, HttpResponse, Result};
use reqwest::Client;
use serde_json::Value;

use crate::config::AppConfig;
use crate::services::ServiceRegistry;

pub async fn proxy_to_user_service(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<AppConfig>,
    service_registry: web::Data<ServiceRegistry>,
) -> Result<HttpResponse> {
    proxy_request(req, body, &config.services.user_service, &service_registry).await
}

pub async fn proxy_to_product_service(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<AppConfig>,
    service_registry: web::Data<ServiceRegistry>,
) -> Result<HttpResponse> {
    proxy_request(req, body, &config.services.product_service, &service_registry).await
}

pub async fn proxy_to_voucher_service(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<AppConfig>,
    service_registry: web::Data<ServiceRegistry>,
) -> Result<HttpResponse> {
    proxy_request(req, body, &config.services.voucher_service, &service_registry).await
}

pub async fn proxy_to_order_service(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<AppConfig>,
    service_registry: web::Data<ServiceRegistry>,
) -> Result<HttpResponse> {
    proxy_request(req, body, &config.services.order_service, &service_registry).await
}

pub async fn proxy_to_payment_service(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<AppConfig>,
    service_registry: web::Data<ServiceRegistry>,
) -> Result<HttpResponse> {
    proxy_request(req, body, &config.services.payment_service, &service_registry).await
}

pub async fn proxy_to_reward_service(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<AppConfig>,
    service_registry: web::Data<ServiceRegistry>,
) -> Result<HttpResponse> {
    proxy_request(req, body, &config.services.reward_service, &service_registry).await
}

pub async fn proxy_to_evaluation_service(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<AppConfig>,
    service_registry: web::Data<ServiceRegistry>,
) -> Result<HttpResponse> {
    proxy_request(req, body, &config.services.evaluation_service, &service_registry).await
}

pub async fn proxy_to_notification_service(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<AppConfig>,
    service_registry: web::Data<ServiceRegistry>,
) -> Result<HttpResponse> {
    proxy_request(req, body, &config.services.notification_service, &service_registry).await
}

async fn proxy_request(
    req: HttpRequest,
    body: web::Bytes,
    target_service: &str,
    service_registry: &ServiceRegistry,
) -> Result<HttpResponse> {
    let client = &service_registry.http_client;
    let method = req.method().clone();
    let path = req.uri().path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or(req.uri().path());
    
    let url = format!("{}{}", target_service, path);
    
    // 转发请求头
    let mut request_builder = match method.as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "PATCH" => client.patch(&url),
        _ => return Ok(HttpResponse::MethodNotAllowed().finish()),
    };
    
    // 复制请求头
    for (key, value) in req.headers() {
        if key != "host" && key != "content-length" {
            request_builder = request_builder.header(key, value);
        }
    }
    
    // 添加请求体
    if !body.is_empty() {
        request_builder = request_builder.body(body.to_vec());
    }
    
    // 发送请求
    match request_builder.send().await {
        Ok(response) => {
            let status = response.status();
            let headers = response.headers().clone();
            let body = response.bytes().await.unwrap_or_default();
            
            let mut http_response = HttpResponse::build(
                actix_web::http::StatusCode::from_u16(status.as_u16())
                    .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
            );
            
            // 复制响应头
            for (key, value) in headers {
                if let Some(key) = key {
                    if key != "content-length" && key != "transfer-encoding" {
                        http_response.insert_header((key, value));
                    }
                }
            }
            
            Ok(http_response.body(body))
        }
        Err(e) => {
            log::error!("Failed to proxy request to {}: {}", url, e);
            Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "message": format!("Failed to connect to {}", target_service)
            })))
        }
    }
} 