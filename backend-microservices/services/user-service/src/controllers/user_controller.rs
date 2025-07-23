use actix_web::{web, HttpResponse, Result as ActixResult};
use uuid::Uuid;
use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::models::{CreateUserRequest, UpdateUserRequest, LoginRequest};
use crate::services::user_service::UserService;

#[derive(Debug, Deserialize)]
pub struct UserParams {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

pub async fn register_user(
    user_service: web::Data<UserService>,
    request: web::Json<CreateUserRequest>,
) -> ActixResult<HttpResponse> {
    // 验证请求数据
    if let Err(validation_errors) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        })));
    }

    match user_service.register_user(request.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Created().json(serde_json::json!({
            "message": "User registered successfully",
            "user": user
        }))),
        Err(e) => {
            tracing::error!("Failed to register user: {}", e);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Registration failed",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn login(
    user_service: web::Data<UserService>,
    request: web::Json<LoginRequest>,
) -> ActixResult<HttpResponse> {
    // 验证请求数据
    if let Err(validation_errors) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        })));
    }

    match user_service.login(request.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => {
            tracing::error!("Login failed: {}", e);
            Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Login failed",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn get_user(
    user_service: web::Data<UserService>,
    path: web::Path<UserParams>,
) -> ActixResult<HttpResponse> {
    match user_service.get_user(path.id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to get user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get user",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn update_user(
    user_service: web::Data<UserService>,
    path: web::Path<UserParams>,
    request: web::Json<UpdateUserRequest>,
) -> ActixResult<HttpResponse> {
    // 验证请求数据
    if let Err(validation_errors) = request.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        })));
    }

    match user_service.update_user(path.id, request.into_inner()).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "User updated successfully",
            "user": user
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to update user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update user",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn delete_user(
    user_service: web::Data<UserService>,
    path: web::Path<UserParams>,
) -> ActixResult<HttpResponse> {
    match user_service.delete_user(path.id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "User deleted successfully"
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
        Err(e) => {
            tracing::error!("Failed to delete user: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete user",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn list_users(
    user_service: web::Data<UserService>,
    query: web::Query<PaginationQuery>,
) -> ActixResult<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    match tokio::try_join!(
        user_service.list_users(page, page_size),
        user_service.count_users()
    ) {
        Ok((users, total)) => {
            let total_pages = (total + page_size - 1) / page_size;
            let response = PaginatedResponse {
                data: users,
                total,
                page,
                page_size,
                total_pages,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            tracing::error!("Failed to list users: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to list users",
                "message": e.to_string()
            })))
        }
    }
}

pub async fn health_check() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "user-service",
        "timestamp": chrono::Utc::now()
    })))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
            .route("", web::get().to(list_users))
    )
    .route("/health", web::get().to(health_check));
} 