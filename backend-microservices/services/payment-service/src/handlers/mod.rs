pub mod payment_handler;

use actix_web::{web, HttpResponse, Result};

pub fn payment_routes() -> actix_web::Scope {
    web::scope("/api/v1")
        .service(payment_handler::create_payment)
        .service(payment_handler::get_payment)
        .service(payment_handler::process_payment)
        .service(payment_handler::refund_payment)
        .service(payment_handler::cancel_payment)
        .service(payment_handler::list_payments)
}

pub fn webhook_routes() -> actix_web::Scope {
    web::scope("/webhooks")
        .service(payment_handler::stripe_webhook)
        .service(payment_handler::paypal_webhook)
}

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "payment-service",
        "timestamp": chrono::Utc::now()
    })))
} 