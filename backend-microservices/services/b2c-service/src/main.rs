use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use sea_orm::Database;
use std::env;

mod config;
mod models;
mod services;
mod repositories;
mod controllers;
mod events;

use config::Config;
use services::b2c_service::B2CService;
use controllers::b2c_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载配置
    let config = Config::from_env().expect("Failed to load configuration");
    
    // 连接数据库
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:pass@localhost/b2c_db".to_string());
    
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    // 创建服务
    let b2c_service = B2CService::new(db);
    
    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    
    println!("🚀 B2C Service starting on {}:{}", server_host, server_port);
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .app_data(web::Data::new(b2c_service.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(b2c_controller::configure_routes)
    })
    .bind((server_host, server_port))?
    .run()
    .await
} 