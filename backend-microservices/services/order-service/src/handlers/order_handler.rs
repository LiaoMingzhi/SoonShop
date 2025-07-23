use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use validator::Validate;

use crate::models::order::{CreateOrderRequest, UpdateOrderRequest, OrderResponse, OrderStatus};
use crate::services::order_service::OrderService;
use crate::db::Database;
use crate::error::ServiceError;

pub async fn create_order(
    req: web::Json<CreateOrderRequest>,
    db: web::Data<Database>,
    user_id: web::ReqData<Uuid>, // 从JWT中获取用户ID
) -> Result<HttpResponse, ServiceError> {
    req.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;
    
    let order_service = OrderService::new(db.get_ref());
    
    match order_service.create_order(&req, *user_id).await {
        Ok(order) => {
            let response = OrderResponse::from(order);
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => {
            log::error!("Failed to create order: {}", e);
            Err(ServiceError::InternalError("Failed to create order".to_string()))
        }
    }
}

pub async fn get_order(
    path: web::Path<Uuid>,
    db: web::Data<Database>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let order_id = path.into_inner();
    let order_service = OrderService::new(db.get_ref());
    
    match order_service.get_order_by_id(order_id, *user_id).await {
        Ok(Some(order)) => {
            let response = OrderResponse::from(order);
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Err(ServiceError::NotFound("Order not found".to_string())),
        Err(e) => {
            log::error!("Failed to get order: {}", e);
            Err(ServiceError::InternalError("Failed to get order".to_string()))
        }
    }
}

pub async fn update_order(
    path: web::Path<Uuid>,
    req: web::Json<UpdateOrderRequest>,
    db: web::Data<Database>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let order_id = path.into_inner();
    req.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;
    
    let order_service = OrderService::new(db.get_ref());
    
    match order_service.update_order(order_id, &req, *user_id).await {
        Ok(order) => {
            let response = OrderResponse::from(order);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to update order: {}", e);
            Err(ServiceError::InternalError("Failed to update order".to_string()))
        }
    }
}

pub async fn cancel_order(
    path: web::Path<Uuid>,
    db: web::Data<Database>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let order_id = path.into_inner();
    let order_service = OrderService::new(db.get_ref());
    
    match order_service.cancel_order(order_id, *user_id).await {
        Ok(order) => {
            let response = OrderResponse::from(order);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to cancel order: {}", e);
            Err(ServiceError::InternalError("Failed to cancel order".to_string()))
        }
    }
}

pub async fn confirm_order(
    path: web::Path<Uuid>,
    db: web::Data<Database>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let order_id = path.into_inner();
    let order_service = OrderService::new(db.get_ref());
    
    match order_service.confirm_order(order_id, *user_id).await {
        Ok(order) => {
            let response = OrderResponse::from(order);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            log::error!("Failed to confirm order: {}", e);
            Err(ServiceError::InternalError("Failed to confirm order".to_string()))
        }
    }
}

pub async fn get_user_orders(
    db: web::Data<Database>,
    user_id: web::ReqData<Uuid>,
    query: web::Query<OrderListQuery>,
) -> Result<HttpResponse, ServiceError> {
    let order_service = OrderService::new(db.get_ref());
    
    match order_service.get_user_orders(*user_id, &query).await {
        Ok(orders) => {
            let responses: Vec<OrderResponse> = orders.into_iter()
                .map(OrderResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(responses))
        }
        Err(e) => {
            log::error!("Failed to get user orders: {}", e);
            Err(ServiceError::InternalError("Failed to get user orders".to_string()))
        }
    }
}

pub async fn get_order_status(
    path: web::Path<Uuid>,
    db: web::Data<Database>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let order_id = path.into_inner();
    let order_service = OrderService::new(db.get_ref());
    
    match order_service.get_order_status(order_id, *user_id).await {
        Ok(Some(status)) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "order_id": order_id,
                "status": status
            })))
        }
        Ok(None) => Err(ServiceError::NotFound("Order not found".to_string())),
        Err(e) => {
            log::error!("Failed to get order status: {}", e);
            Err(ServiceError::InternalError("Failed to get order status".to_string()))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct OrderListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<OrderStatus>,
    pub order_type: Option<String>,
}

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "order-service",
        "timestamp": chrono::Utc::now()
    })))
} 