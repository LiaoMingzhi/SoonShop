use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

mod config;
mod db;
mod models;
mod handlers;
mod services;
mod repositories;
mod events;
mod error;

use config::NotificationConfig;
use crate::db::Database;
use error::Result;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let config = NotificationConfig::default();
    let bind_address = format!("{}:{}", config.host, config.port);
    
    log::info!("Starting Notification Service on {}:{}", config.host, config.port);
    
    // 连接数据库
    let database = db::connect_db(&config.database_url)
        .await
        .expect("Failed to connect to database");
    
    // 注意：如果需要运行迁移，应该使用sqlx-cli或者单独的迁移工具
    // sqlx migrate run --database-url=$DATABASE_URL
    
    // 初始化Redis连接
    let redis_client = redis::Client::open(config.redis_url.as_str())
        .expect("Failed to create Redis client");
    
    // 初始化消息队列
    let amqp_connection = lapin::Connection::connect(
        &config.rabbitmq_url,
        lapin::ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect to RabbitMQ");
    
    let amqp_channel = amqp_connection
        .create_channel()
        .await
        .expect("Failed to create AMQP channel");
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(amqp_channel.clone()))
            .service(
                web::scope("/api/v1/notifications")
                    .route("/", web::post().to(handlers::notification_handler::create_notification))
                    .route("/", web::get().to(handlers::notification_handler::list_notifications))
                    .route("/{id}", web::get().to(handlers::notification_handler::get_notification))
                    .route("/{id}", web::put().to(handlers::notification_handler::update_notification))
                    .route("/{id}", web::delete().to(handlers::notification_handler::delete_notification))
                    .route("/{id}/read", web::post().to(handlers::notification_handler::mark_as_read))
                    .route("/{id}/unread", web::post().to(handlers::notification_handler::mark_as_unread))
                    .route("/bulk/read", web::post().to(handlers::notification_handler::bulk_mark_as_read))
                    .route("/bulk/delete", web::delete().to(handlers::notification_handler::bulk_delete))
                    .route("/bulk", web::post().to(handlers::notification_handler::send_bulk_notification))
            )
            .service(
                web::scope("/api/v1/templates")
                    .route("/", web::post().to(handlers::notification_handler::create_template))
                    .route("/", web::get().to(handlers::notification_handler::list_templates))
                    .route("/{id}", web::get().to(handlers::notification_handler::get_template))
                    .route("/{id}", web::put().to(handlers::notification_handler::update_template))
                    .route("/{id}", web::delete().to(handlers::notification_handler::delete_template))
                    .route("/{id}/preview", web::post().to(handlers::notification_handler::preview_template))
            )
            .service(
                web::scope("/api/v1/preferences")
                    .route("/", web::get().to(handlers::notification_handler::get_user_preferences))
                    .route("/", web::put().to(handlers::notification_handler::update_user_preferences))
                    .route("/reset", web::post().to(handlers::notification_handler::reset_user_preferences))
            )
            .service(
                web::scope("/api/v1/subscriptions")
                    .route("/", web::get().to(handlers::notification_handler::list_subscriptions))
                    .route("/", web::post().to(handlers::notification_handler::subscribe))
                    .route("/{id}", web::delete().to(handlers::notification_handler::unsubscribe))
                    .route("/push", web::post().to(handlers::notification_handler::subscribe_push))
            )
            .service(
                web::scope("/api/v1/statistics")
                    .route("/", web::get().to(handlers::notification_handler::get_statistics))
                    .route("/delivery", web::get().to(handlers::notification_handler::get_delivery_statistics))
                    .route("/engagement", web::get().to(handlers::notification_handler::get_engagement_statistics))
            )
            .route("/health", web::get().to(handlers::notification_handler::health_check))
            .wrap(Logger::default())
    })
    .bind(bind_address)?
    .run()
    .await
} 