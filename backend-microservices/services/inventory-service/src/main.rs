use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;

mod config;
mod handlers;
mod models;
mod services;
mod repositories;
mod events;
mod db;
mod error;

use config::AppConfig;
use db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let config = AppConfig::from_env().expect("Failed to load configuration");
    
    tracing::info!("Starting Inventory Service on {}:{}", config.server.host, config.server.port);
    
    // 初始化数据库
    let database = Database::new(&config.database).await
        .expect("Failed to connect to database");
    
    // 运行数据库迁移
    database.migrate().await.expect("Failed to run migrations");
    
    // 初始化Redis连接
    let redis_client = redis::Client::open(config.redis.url.as_str())
        .expect("Failed to create Redis client");
    
    // 保存服务器配置
    let server_config = config.server.clone();
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .wrap(Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(handlers::configure_routes)
    })
    .bind(format!("{}:{}", server_config.host, server_config.port))?
    .run()
    .await
} 