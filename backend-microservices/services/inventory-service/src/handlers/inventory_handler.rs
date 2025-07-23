use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::db::Database;
use crate::services::InventoryService;
use crate::models::*;

// 查询参数
#[derive(Debug, Deserialize)]
pub struct InventoryQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub product_id: Option<Uuid>,
}

// 响应结构
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

// 基本的处理器
pub async fn create_product(
    db: web::Data<Database>,
    req: web::Json<CreateProductRequest>,
) -> Result<HttpResponse> {
    let service = InventoryService::new(&db);
    
    match service.create_product(req.into_inner()).await {
        Ok(product) => Ok(HttpResponse::Created().json(product)),
        Err(e) => {
            tracing::error!("Failed to create product: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create product",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn get_product(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = InventoryService::new(&db);
    let product_id = path.into_inner();
    
    match service.get_product(product_id).await {
        Ok(product) => Ok(HttpResponse::Ok().json(product)),
        Err(e) => {
            tracing::error!("Failed to get product: {}", e);
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Product not found"
            })))
        }
    }
}

pub async fn update_stock(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateStockRequest>,
) -> Result<HttpResponse> {
    let service = InventoryService::new(&db);
    let product_id = path.into_inner();
    
    match service.update_stock(product_id, req.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Stock updated successfully"
        }))),
        Err(e) => {
            tracing::error!("Failed to update stock: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update stock",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn get_stock(
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let service = InventoryService::new(&db);
    let product_id = path.into_inner();
    
    match service.get_stock(product_id).await {
        Ok(stock) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "product_id": product_id,
            "stock": stock
        }))),
        Err(e) => {
            tracing::error!("Failed to get stock: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get stock",
                "message": e.to_string()
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