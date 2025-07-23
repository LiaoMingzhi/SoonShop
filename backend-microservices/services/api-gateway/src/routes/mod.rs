use actix_web::{web, HttpResponse, Result};

use crate::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // 健康检查
        .route("/health", web::get().to(health_check))
        .route("/metrics", web::get().to(metrics))
        
        // 认证路由
        .service(
            web::scope("/auth")
                .route("/login", web::post().to(handlers::auth::login))
                .route("/register", web::post().to(handlers::auth::register))
                .route("/refresh", web::post().to(handlers::auth::refresh))
                .route("/logout", web::post().to(handlers::auth::logout))
        )
        
        // 用户服务路由
        .service(
            web::scope("/api/users")
                .route("", web::get().to(handlers::proxy::proxy_to_user_service))
                .route("", web::post().to(handlers::proxy::proxy_to_user_service))
                .route("/{id}", web::get().to(handlers::proxy::proxy_to_user_service))
                .route("/{id}", web::put().to(handlers::proxy::proxy_to_user_service))
                .route("/{id}", web::delete().to(handlers::proxy::proxy_to_user_service))
        )
        
        // 产品服务路由
        .service(
            web::scope("/api/products")
                .route("", web::get().to(handlers::proxy::proxy_to_product_service))
                .route("", web::post().to(handlers::proxy::proxy_to_product_service))
                .route("/{id}", web::get().to(handlers::proxy::proxy_to_product_service))
                .route("/{id}", web::put().to(handlers::proxy::proxy_to_product_service))
                .route("/{id}", web::delete().to(handlers::proxy::proxy_to_product_service))
                .route("/search", web::get().to(handlers::proxy::proxy_to_product_service))
        )
        
        // 提货券服务路由
        .service(
            web::scope("/api/vouchers")
                .route("", web::get().to(handlers::proxy::proxy_to_voucher_service))
                .route("", web::post().to(handlers::proxy::proxy_to_voucher_service))
                .route("/{id}", web::get().to(handlers::proxy::proxy_to_voucher_service))
                .route("/{id}", web::put().to(handlers::proxy::proxy_to_voucher_service))
                .route("/{id}", web::delete().to(handlers::proxy::proxy_to_voucher_service))
                .route("/{id}/claim", web::post().to(handlers::proxy::proxy_to_voucher_service))
                .route("/{id}/consume", web::post().to(handlers::proxy::proxy_to_voucher_service))
        )
        
        // 订单服务路由
        .service(
            web::scope("/api/orders")
                .route("", web::get().to(handlers::proxy::proxy_to_order_service))
                .route("", web::post().to(handlers::proxy::proxy_to_order_service))
                .route("/{id}", web::get().to(handlers::proxy::proxy_to_order_service))
                .route("/{id}", web::put().to(handlers::proxy::proxy_to_order_service))
                .route("/{id}/cancel", web::post().to(handlers::proxy::proxy_to_order_service))
                .route("/{id}/confirm", web::post().to(handlers::proxy::proxy_to_order_service))
        )
        
        // 支付服务路由
        .service(
            web::scope("/api/payments")
                .route("", web::get().to(handlers::proxy::proxy_to_payment_service))
                .route("", web::post().to(handlers::proxy::proxy_to_payment_service))
                .route("/{id}", web::get().to(handlers::proxy::proxy_to_payment_service))
                .route("/{id}/refund", web::post().to(handlers::proxy::proxy_to_payment_service))
        )
        
        // 奖励服务路由
        .service(
            web::scope("/api/rewards")
                .route("", web::get().to(handlers::proxy::proxy_to_reward_service))
                .route("/calculate", web::post().to(handlers::proxy::proxy_to_reward_service))
                .route("/distribute", web::post().to(handlers::proxy::proxy_to_reward_service))
                .route("/{id}", web::get().to(handlers::proxy::proxy_to_reward_service))
        )
        
        // 企业评估服务路由
        .service(
            web::scope("/api/evaluations")
                .route("", web::get().to(handlers::proxy::proxy_to_evaluation_service))
                .route("", web::post().to(handlers::proxy::proxy_to_evaluation_service))
                .route("/{id}", web::get().to(handlers::proxy::proxy_to_evaluation_service))
                .route("/{id}", web::put().to(handlers::proxy::proxy_to_evaluation_service))
                .route("/{id}/submit", web::post().to(handlers::proxy::proxy_to_evaluation_service))
        )
        
        // 通知服务路由
        .service(
            web::scope("/api/notifications")
                .route("", web::get().to(handlers::proxy::proxy_to_notification_service))
                .route("", web::post().to(handlers::proxy::proxy_to_notification_service))
                .route("/{id}/read", web::post().to(handlers::proxy::proxy_to_notification_service))
        );
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "service": "api-gateway"
    })))
}

async fn metrics() -> Result<HttpResponse> {
    // TODO: 实现Prometheus指标收集
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "metrics": "TODO: implement metrics collection"
    })))
} 