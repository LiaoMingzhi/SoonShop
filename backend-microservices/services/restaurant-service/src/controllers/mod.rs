pub mod restaurant_controller;

use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/restaurant")
            .route("/health", web::get().to(health_check))
            .route("/restaurants/search", web::get().to(search_restaurants))
            .route("/reservations", web::post().to(create_reservation))
            .route("/reservations/{id}", web::get().to(get_reservation))
            .route("/reservations/{id}/cancel", web::post().to(cancel_reservation))
            .route("/restaurants/{id}/availability", web::get().to(check_availability))
    );
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "restaurant-service",
        "timestamp": chrono::Utc::now()
    })))
}

async fn search_restaurants() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "restaurants": [],
            "total_count": 0,
            "page": 1,
            "limit": 10,
            "has_more": false
        }
    })))
}

async fn create_reservation() -> Result<HttpResponse> {
    Ok(HttpResponse::Created().json(json!({
        "success": true,
        "message": "预订已创建",
        "data": {
            "reservation_id": uuid::Uuid::new_v4(),
            "status": "pending"
        }
    })))
}

async fn get_reservation() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "reservation": {
                "id": uuid::Uuid::new_v4(),
                "status": "confirmed"
            }
        }
    })))
}

async fn cancel_reservation() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "预订已取消"
    })))
}

async fn check_availability() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "available_times": []
        }
    })))
} 