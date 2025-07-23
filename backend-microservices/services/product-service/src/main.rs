use actix_web::{web, App, HttpServer, Result, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use tracing::info;

mod controllers;
mod services;
mod repositories;
mod models;
mod events;
mod config;

use config::Config;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载配置
    let config = Config::from_env()?;
    
    // TODO: 初始化数据库连接（使用SeaORM）
    // let db_connection = Database::connect(&config.database_url).await?;
    
    // 初始化Redis连接
    let redis_client = redis::Client::open(config.redis_url.as_str())?;
    
    // TODO: 运行数据库迁移
    // Migrator::up(&db_connection, None).await?;
    
    let bind_addr = format!("0.0.0.0:{}", config.port);
    info!("Product service listening on {}", bind_addr);
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            // .app_data(web::Data::new(db_connection.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api/v1/products")
                    .route("", web::post().to(controllers::product_controller::create_product))
                    .route("", web::get().to(controllers::product_controller::list_products))
                    .route("/search", web::get().to(controllers::product_controller::search_products))
                    .route("/{id}", web::get().to(controllers::product_controller::get_product))
                    .route("/{id}", web::put().to(controllers::product_controller::update_product))
                    .route("/{id}", web::delete().to(controllers::product_controller::delete_product))
            )
    })
    .bind(bind_addr)?
    .run()
    .await?;
    
    Ok(())
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "product-service",
        "timestamp": chrono::Utc::now()
    })))
} 