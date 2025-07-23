use actix_web::{web, App, HttpServer};
use actix_cors;
use tracing_actix_web;
use dotenv::dotenv;
use std::env;

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod providers;
mod repositories;
mod events;
mod security;
mod services;

use config::AppConfig;
use db::Database;
use services::payment_service::PaymentService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let config = AppConfig::from_env().expect("Failed to load configuration");
    let bind_address = format!("{}:{}", config.host, config.port);

    // Initialize logging
    tracing_subscriber::fmt::init();

    tracing::info!("Starting payment service on {}", bind_address);
    
    // Initialize database
    let db = Database::new(&config.database_url)
        .await
        .expect("Failed to initialize database");
    
    // Initialize payment service
    let payment_service = PaymentService::new(db, config.clone())
        .await
        .expect("Failed to initialize payment service");
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(payment_service.clone()))
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(actix_cors::Cors::permissive())
            .service(handlers::payment_routes())
            .service(handlers::webhook_routes())
            .route("/health", web::get().to(handlers::health))
    })
    .bind(bind_address)?
    .run()
    .await
} 