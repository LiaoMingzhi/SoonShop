use crate::models::PriceUpdateRequest;
use crate::services::price_service::PriceService;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use validator::Validate;

pub async fn update_price(
    service: web::Data<PriceService>,
    request: web::Json<PriceUpdateRequest>,
) -> Result<HttpResponse> {
    if let Err(errors) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors
        })));
    }

    match service.update_price(request.into_inner()).await {
        Ok(record) => Ok(HttpResponse::Ok().json(record)),
        Err(e) => {
            tracing::error!("Failed to update price: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update price"
            })))
        }
    }
}

pub async fn get_price_history(
    service: web::Data<PriceService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let product_id = path.into_inner();

    match service.get_price_history(product_id, "30d").await {
        Ok(history) => Ok(HttpResponse::Ok().json(history)),
        Err(e) => {
            tracing::error!("Failed to get price history: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get price history"
            })))
        }
    }
}

pub async fn analyze_price_trends(
    service: web::Data<PriceService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let product_id = path.into_inner();

    match service.analyze_price_trends(product_id).await {
        Ok(analysis) => Ok(HttpResponse::Ok().json(analysis)),
        Err(e) => {
            tracing::error!("Failed to analyze price trends: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to analyze price trends"
            })))
        }
    }
}

pub async fn detect_anomalies(
    service: web::Data<PriceService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let product_id = path.into_inner();

    match service.detect_price_anomalies(product_id).await {
        Ok(alerts) => Ok(HttpResponse::Ok().json(alerts)),
        Err(e) => {
            tracing::error!("Failed to detect anomalies: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to detect anomalies"
            })))
        }
    }
}

pub async fn get_market_trends(
    service: web::Data<PriceService>,
) -> Result<HttpResponse> {
    match service.get_market_trends("30d").await {
        Ok(trends) => Ok(HttpResponse::Ok().json(trends)),
        Err(e) => {
            tracing::error!("Failed to get market trends: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get market trends"
            })))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/prices")
            .route("", web::post().to(update_price))
            .route("/trends", web::get().to(get_market_trends))
            .route("/{product_id}/history", web::get().to(get_price_history))
            .route("/{product_id}/analysis", web::get().to(analyze_price_trends))
            .route("/{product_id}/anomalies", web::get().to(detect_anomalies))
    );
} 