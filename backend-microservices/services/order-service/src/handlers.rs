use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::*;
use crate::services::OrderService;
use crate::db::Database;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

pub async fn create_order(
    db: web::Data<Database>,
    req: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse> {
    let service = OrderService::new(&db);
    
    match service.create_order(req.into_inner()).await {
        Ok(order) => Ok(HttpResponse::Created().json(order)),
        Err(e) => {
            tracing::error!("Failed to create order: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create order",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn get_order(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = OrderService::new(&db);
    let order_id = path.into_inner();
    
    match service.get_order(order_id).await {
        Ok(order) => Ok(HttpResponse::Ok().json(order)),
        Err(e) => {
            tracing::error!("Failed to get order: {}", e);
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Order not found"
            })))
        }
    }
}

pub async fn health_check() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        version: "1.0.0".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/orders")
                    .route("", web::post().to(create_order))
                    .route("/{id}", web::get().to(get_order))
            )
            .route("/health", web::get().to(health_check))
    );
} 