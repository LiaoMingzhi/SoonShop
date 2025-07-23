use actix_web::{web, App, HttpServer, Result, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use tracing::info;
use std::sync::Arc;

mod controllers;
mod services;
mod repositories;
mod models;
mod events;
mod config;

use config::Config;
use controllers::user_controller;
use services::user_service::UserService;
use repositories::user_repository::UserRepository;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载配置
    let config = Config::from_env()?;
    
    // 初始化数据库连接 (TODO: 实现实际的数据库连接)
    let database_connection = "placeholder";
    
    // 初始化Redis连接
    let redis_client = redis::Client::open(config.redis_url.as_str())?;
    
    // 初始化服务依赖
    let user_repository = Arc::new(UserRepository::new(database_connection));
    let user_service = UserService::new(user_repository, config.jwt_secret.clone());
    
    let bind_addr = format!("0.0.0.0:{}", config.port);
    info!("User service listening on {}", bind_addr);
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(user_controller::configure_routes)
            .route("/health", web::get().to(health_check))
    })
    .bind(bind_addr)?
    .run()
    .await?;
    
    Ok(())
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "user-service",
        "timestamp": chrono::Utc::now()
    })))
} 