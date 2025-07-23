use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web::http::header::HeaderMap;
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::models::payment::{
    CreatePaymentRequest, ProcessPaymentRequest, RefundPaymentRequest,
    PaymentResponse, RefundResponse, PaymentStatus
};
use crate::services::payment_service::PaymentService;
use crate::providers::PaymentProviders;
use crate::db::Database;
use crate::error::ServiceError;

#[derive(Deserialize)]
pub struct PaymentListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<String>,
    pub payment_method: Option<String>,
    pub order_id: Option<Uuid>,
}

#[actix_web::post("/payments")]
pub async fn create_payment(
    req: web::Json<CreatePaymentRequest>,
    service: web::Data<PaymentService>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    // 手动验证 amount 字段
    if req.amount <= rust_decimal::Decimal::ZERO {
        return Err(ServiceError::ValidationError("Amount must be greater than 0".to_string()));
    }
    
    let payment = service.create_payment(&req, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(payment))
}

#[actix_web::get("/payments/{payment_id}")]
pub async fn get_payment(
    path: web::Path<Uuid>,
    service: web::Data<PaymentService>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let payment_id = path.into_inner();
    match service.get_payment_by_id(payment_id, user_id.into_inner()).await {
        Ok(Some(payment)) => Ok(HttpResponse::Ok().json(payment)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Payment not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to get payment: {}", e);
            Err(ServiceError::InternalError(e.to_string()))
        }
    }
}

#[actix_web::post("/payments/{payment_id}/process")]
pub async fn process_payment(
    path: web::Path<Uuid>,
    req: web::Json<ProcessPaymentRequest>,
    service: web::Data<PaymentService>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let payment_id = path.into_inner();
    match service.process_payment(payment_id, &req, user_id.into_inner()).await {
        Ok(payment) => Ok(HttpResponse::Ok().json(payment)),
        Err(e) => {
            tracing::error!("Failed to process payment: {}", e);
            Err(ServiceError::PaymentError(e.to_string()))
        }
    }
}

#[actix_web::post("/payments/{payment_id}/refund")]
pub async fn refund_payment(
    path: web::Path<Uuid>,
    req: web::Json<RefundPaymentRequest>,
    service: web::Data<PaymentService>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    // 手动验证 amount 字段
    if let Some(amount) = req.amount {
        if amount <= rust_decimal::Decimal::ZERO {
            return Err(ServiceError::ValidationError("Refund amount must be greater than 0".to_string()));
        }
    }
    
    let payment_id = path.into_inner();
    match service.refund_payment(payment_id, &req, user_id.into_inner()).await {
        Ok(refund) => Ok(HttpResponse::Ok().json(refund)),
        Err(e) => {
            tracing::error!("Failed to refund payment: {}", e);
            Err(ServiceError::PaymentError(e.to_string()))
        }
    }
}

#[actix_web::get("/payments")]
pub async fn list_payments(
    query: web::Query<PaymentListQuery>,
    service: web::Data<PaymentService>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    match service.get_user_payments(user_id.into_inner(), &query).await {
        Ok(payments) => Ok(HttpResponse::Ok().json(payments)),
        Err(e) => {
            tracing::error!("Failed to get user payments: {}", e);
            Err(ServiceError::InternalError(e.to_string()))
        }
    }
}

#[actix_web::post("/webhooks/stripe")]
pub async fn stripe_webhook(
    req: HttpRequest,
    body: web::Bytes,
    service: web::Data<PaymentService>,
) -> Result<HttpResponse, ServiceError> {
    let signature = req.headers()
        .get("stripe-signature")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ServiceError::ValidationError("Missing stripe signature".to_string()))?;

    let payload = String::from_utf8(body.to_vec())
        .map_err(|_| ServiceError::ValidationError("Invalid payload".to_string()))?;

    service.handle_stripe_webhook(&payload, signature).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"received": true})))
}

#[actix_web::post("/webhooks/paypal")]
pub async fn paypal_webhook(
    req: HttpRequest,
    body: web::Bytes,
    service: web::Data<PaymentService>,
) -> Result<HttpResponse, ServiceError> {
    let payload = String::from_utf8(body.to_vec())
        .map_err(|_| ServiceError::ValidationError("Invalid payload".to_string()))?;

    service.handle_paypal_webhook(&payload, req.headers()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"received": true})))
}

pub async fn get_payment_methods(
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    // 返回用户可用的支付方式
    let payment_methods = vec![
        serde_json::json!({
            "type": "solana",
            "name": "Solana Wallet",
            "description": "Pay with SOL or SPL tokens",
            "fees": "0.1%",
            "processing_time": "Instant"
        }),
        serde_json::json!({
            "type": "credit_card",
            "name": "Credit Card",
            "description": "Visa, Mastercard, American Express",
            "fees": "2.9% + $0.30",
            "processing_time": "Instant"
        }),
        serde_json::json!({
            "type": "paypal",
            "name": "PayPal",
            "description": "Pay with your PayPal account",
            "fees": "2.9% + $0.30",
            "processing_time": "Instant"
        }),
        serde_json::json!({
            "type": "bank_transfer",
            "name": "Bank Transfer",
            "description": "Direct bank transfer",
            "fees": "$5.00",
            "processing_time": "1-3 business days"
        }),
    ];
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "payment_methods": payment_methods
    })))
}

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "payment-service",
        "timestamp": chrono::Utc::now()
    })))
}

#[actix_web::get("/payments/{payment_id}/cancel")]
pub async fn cancel_payment(
    path: web::Path<Uuid>,
    service: web::Data<PaymentService>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let payment_id = path.into_inner();
    match service.cancel_payment(payment_id, user_id.into_inner()).await {
        Ok(payment) => Ok(HttpResponse::Ok().json(payment)),
        Err(e) => {
            tracing::error!("Failed to cancel payment: {}", e);
            Err(ServiceError::PaymentError(e.to_string()))
        }
    }
} 