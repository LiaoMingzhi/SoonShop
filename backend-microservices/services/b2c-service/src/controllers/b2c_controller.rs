use crate::models::*;
use crate::services::b2c_service::B2CService;
use actix_web::{web, HttpResponse, Result as ActixResult};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

pub struct B2CController {
    service: B2CService,
}

impl B2CController {
    pub fn new(service: B2CService) -> Self {
        Self { service }
    }

    // ============ 购物车相关接口 ============

    /// 添加商品到购物车
    /// POST /api/v1/b2c/cart/items
    pub async fn add_to_cart(
        service: web::Data<B2CService>,
        request: web::Json<AddToCartRequest>,
    ) -> ActixResult<HttpResponse> {
        // 验证请求数据
        if let Err(validation_errors) = request.validate() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "validation_failed",
                "details": validation_errors
            })));
        }

        match service.add_to_cart(request.into_inner()).await {
            Ok(cart) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": cart
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "add_to_cart_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 获取购物车
    /// GET /api/v1/b2c/cart/{user_id}
    pub async fn get_cart(
        service: web::Data<B2CService>,
        path: web::Path<Uuid>,
    ) -> ActixResult<HttpResponse> {
        let user_id = path.into_inner();

        match service.get_cart(user_id).await {
            Ok(cart_response) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": cart_response
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "get_cart_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 更新购物车商品
    /// PUT /api/v1/b2c/cart/{user_id}/items/{item_id}
    pub async fn update_cart_item(
        service: web::Data<B2CService>,
        path: web::Path<(Uuid, Uuid)>,
        request: web::Json<UpdateCartItemRequest>,
    ) -> ActixResult<HttpResponse> {
        let (user_id, item_id) = path.into_inner();

        // 验证请求数据
        if let Err(validation_errors) = request.validate() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "validation_failed",
                "details": validation_errors
            })));
        }

        match service.update_cart_item(user_id, item_id, request.into_inner()).await {
            Ok(cart) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": cart
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "update_cart_item_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 从购物车移除商品
    /// DELETE /api/v1/b2c/cart/{user_id}/items/{item_id}
    pub async fn remove_from_cart(
        service: web::Data<B2CService>,
        path: web::Path<(Uuid, Uuid)>,
    ) -> ActixResult<HttpResponse> {
        let (user_id, item_id) = path.into_inner();

        match service.remove_from_cart(user_id, item_id).await {
            Ok(cart) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": cart
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "remove_from_cart_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 清空购物车
    /// DELETE /api/v1/b2c/cart/{user_id}
    pub async fn clear_cart(
        service: web::Data<B2CService>,
        path: web::Path<Uuid>,
    ) -> ActixResult<HttpResponse> {
        let user_id = path.into_inner();

        match service.clear_cart(user_id).await {
            Ok(_) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "message": "购物车已清空"
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "clear_cart_failed",
                "message": e.to_string()
            }))),
        }
    }

    // ============ 订单相关接口 ============

    /// 创建订单
    /// POST /api/v1/b2c/orders
    pub async fn create_order(
        service: web::Data<B2CService>,
        request: web::Json<CreateOrderRequest>,
    ) -> ActixResult<HttpResponse> {
        // 验证请求数据
        if let Err(validation_errors) = request.validate() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "validation_failed",
                "details": validation_errors
            })));
        }

        match service.create_order(request.into_inner()).await {
            Ok(order_response) => Ok(HttpResponse::Created().json(json!({
                "success": true,
                "data": order_response
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "create_order_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 获取订单详情
    /// GET /api/v1/b2c/orders/{order_id}
    pub async fn get_order(
        service: web::Data<B2CService>,
        path: web::Path<Uuid>,
        query: web::Query<UserIdQuery>,
    ) -> ActixResult<HttpResponse> {
        let order_id = path.into_inner();
        let user_id = query.user_id;

        match service.get_order(user_id, order_id).await {
            Ok(order_response) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": order_response
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "get_order_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 获取用户订单列表
    /// GET /api/v1/b2c/users/{user_id}/orders
    pub async fn get_user_orders(
        service: web::Data<B2CService>,
        path: web::Path<Uuid>,
        query: web::Query<PaginationQuery>,
    ) -> ActixResult<HttpResponse> {
        let user_id = path.into_inner();
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(10);

        match service.get_user_orders(user_id, page, limit).await {
            Ok(orders) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": orders,
                "pagination": {
                    "page": page,
                    "limit": limit,
                    "total": orders.len()
                }
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "get_user_orders_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 取消订单
    /// POST /api/v1/b2c/orders/{order_id}/cancel
    pub async fn cancel_order(
        service: web::Data<B2CService>,
        path: web::Path<Uuid>,
        query: web::Query<UserIdQuery>,
    ) -> ActixResult<HttpResponse> {
        let order_id = path.into_inner();
        let user_id = query.user_id;

        match service.cancel_order(user_id, order_id).await {
            Ok(order_response) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": order_response,
                "message": "订单已取消"
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "cancel_order_failed",
                "message": e.to_string()
            }))),
        }
    }

    // ============ 商品搜索相关接口 ============

    /// 搜索商品
    /// GET /api/v1/b2c/products/search
    pub async fn search_products(
        service: web::Data<B2CService>,
        query: web::Query<ProductSearchRequest>,
    ) -> ActixResult<HttpResponse> {
        let search_request = query.into_inner();

        // 验证请求数据
        if let Err(validation_errors) = search_request.validate() {
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "validation_failed",
                "details": validation_errors
            })));
        }

        match service.search_products(search_request).await {
            Ok(search_response) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": search_response
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "search_products_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 获取商品推荐
    /// GET /api/v1/b2c/users/{user_id}/recommendations
    pub async fn get_recommendations(
        service: web::Data<B2CService>,
        path: web::Path<Uuid>,
    ) -> ActixResult<HttpResponse> {
        let user_id = path.into_inner();

        match service.get_product_recommendations(user_id).await {
            Ok(recommendations) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": recommendations
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "get_recommendations_failed",
                "message": e.to_string()
            }))),
        }
    }

    /// 记录商品浏览
    /// POST /api/v1/b2c/users/{user_id}/browse
    pub async fn record_browse(
        service: web::Data<B2CService>,
        path: web::Path<Uuid>,
        request: web::Json<RecordBrowseRequest>,
    ) -> ActixResult<HttpResponse> {
        let user_id = path.into_inner();
        let browse_request = request.into_inner();

        match service.record_product_browse(user_id, browse_request.product_id, browse_request.duration).await {
            Ok(_) => Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "message": "浏览记录已保存"
            }))),
            Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                "error": "record_browse_failed",
                "message": e.to_string()
            }))),
        }
    }

    // ============ 健康检查接口 ============

    /// 健康检查
    /// GET /api/v1/b2c/health
    pub async fn health_check() -> ActixResult<HttpResponse> {
        Ok(HttpResponse::Ok().json(json!({
            "status": "healthy",
            "service": "b2c-service",
            "timestamp": chrono::Utc::now()
        })))
    }
}

// ============ 查询参数结构体 ============

#[derive(serde::Deserialize)]
pub struct UserIdQuery {
    pub user_id: Uuid,
}

#[derive(serde::Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(serde::Deserialize)]
pub struct RecordBrowseRequest {
    pub product_id: Uuid,
    pub duration: Option<u32>,
}

// ============ 路由配置 ============

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/b2c")
            // 健康检查
            .route("/health", web::get().to(B2CController::health_check))
            
            // 购物车相关路由
            .route("/cart/items", web::post().to(B2CController::add_to_cart))
            .route("/cart/{user_id}", web::get().to(B2CController::get_cart))
            .route("/cart/{user_id}", web::delete().to(B2CController::clear_cart))
            .route("/cart/{user_id}/items/{item_id}", web::put().to(B2CController::update_cart_item))
            .route("/cart/{user_id}/items/{item_id}", web::delete().to(B2CController::remove_from_cart))
            
            // 订单相关路由
            .route("/orders", web::post().to(B2CController::create_order))
            .route("/orders/{order_id}", web::get().to(B2CController::get_order))
            .route("/orders/{order_id}/cancel", web::post().to(B2CController::cancel_order))
            .route("/users/{user_id}/orders", web::get().to(B2CController::get_user_orders))
            
            // 商品搜索和推荐路由
            .route("/products/search", web::get().to(B2CController::search_products))
            .route("/users/{user_id}/recommendations", web::get().to(B2CController::get_recommendations))
            .route("/users/{user_id}/browse", web::post().to(B2CController::record_browse))
    );
} 