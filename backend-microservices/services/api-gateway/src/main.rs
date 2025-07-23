use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use tracing_subscriber::EnvFilter;
// 暂时注释掉jaeger相关的导入，因为API可能有变化
// use opentelemetry_jaeger::JaegerTracer;
// use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

mod config;
mod middleware;
mod routes;
mod services;
mod handlers;
mod models;
mod utils;
mod error;

use config::AppConfig;
use middleware::{auth::AuthMiddleware, cors::setup_cors, rate_limit::RateLimitMiddleware};
use routes::configure_routes;
use services::ServiceRegistry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // 初始化配置
    let config = AppConfig::from_env().expect("Failed to load configuration");
    
    // 初始化日志和链路追踪
    init_tracing(&config).await;
    
    // 初始化服务注册中心
    let service_registry = ServiceRegistry::new(&config).await
        .expect("Failed to initialize service registry");
    
    let bind_address = format!("{}:{}", config.host, config.port);
    log::info!("Starting API Gateway on {}", bind_address);
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(service_registry.clone()))
            .wrap(Logger::default())
            .wrap(setup_cors())
            .wrap(AuthMiddleware::new(config.jwt_secret.clone()))
            .wrap(RateLimitMiddleware::new(config.rate_limit.clone()))
            .configure(configure_routes)
    })
    .bind(bind_address)?
    .run()
    .await
}

async fn init_tracing(_config: &AppConfig) {
    // 简化的tracing初始化，暂时不使用Jaeger
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env());
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");
} 