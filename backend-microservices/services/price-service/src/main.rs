use actix_web::{web, App, HttpServer, Result};
use actix_cors::Cors;
use tracing::{info, error};

mod config;
mod controllers;
mod services;
mod repositories;
mod models;
mod events;

use config::Config;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env().unwrap_or_else(|err| {
        error!("Failed to load configuration: {}", err);
        std::process::exit(1);
    });

    info!("Starting Price Service on port {}", config.server.port);

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.name
    );

    let db = sea_orm::Database::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let price_service = services::price_service::PriceService::new(db.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(price_service.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(
                web::scope("/api/v1")
                    .configure(controllers::price_controller::config)
            )
    })
    .bind(format!("0.0.0.0:{}", config.server.port))?
    .run()
    .await?;

    Ok(())
} 