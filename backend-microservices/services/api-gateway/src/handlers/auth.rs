use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use chrono::{Duration, Utc};

use crate::config::AppConfig;
use crate::middleware::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub wallet_address: Option<String>,
}

pub async fn login(
    req: web::Json<LoginRequest>,
    config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // TODO: 实际的用户认证逻辑，这里只是示例
    let user = authenticate_user(&req.username, &req.password).await?;
    
    let claims = Claims {
        sub: user.id.clone(),
        role: user.role.clone(),
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to create token")
    })?;
    
    // 生成刷新令牌
    let refresh_claims = Claims {
        sub: user.id.clone(),
        role: user.role.clone(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };
    
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to create refresh token")
    })?;
    
    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        refresh_token,
        expires_in: 86400, // 24 hours
        user,
    }))
}

pub async fn register(
    req: web::Json<RegisterRequest>,
    config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // TODO: 实际的用户注册逻辑
    let user = create_user(&req).await?;
    
    let claims = Claims {
        sub: user.id.clone(),
        role: user.role.clone(),
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to create token")
    })?;
    
    let refresh_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to create refresh token")
    })?;
    
    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        refresh_token,
        expires_in: 86400,
        user,
    }))
}

pub async fn refresh(
    config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // TODO: 实现刷新令牌逻辑
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Token refreshed successfully"
    })))
}

pub async fn logout() -> Result<HttpResponse> {
    // TODO: 实现登出逻辑（黑名单等）
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

// 辅助函数
async fn authenticate_user(username: &str, password: &str) -> Result<UserInfo> {
    // TODO: 实际的数据库查询和密码验证
    // 这里只是示例数据
    if username == "admin" && password == "password" {
        Ok(UserInfo {
            id: "1".to_string(),
            username: username.to_string(),
            email: "admin@soonshop.com".to_string(),
            role: "admin".to_string(),
            wallet_address: None,
        })
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
    }
}

async fn create_user(req: &RegisterRequest) -> Result<UserInfo> {
    // TODO: 实际的用户创建逻辑
    Ok(UserInfo {
        id: uuid::Uuid::new_v4().to_string(),
        username: req.username.clone(),
        email: req.email.clone(),
        role: "user".to_string(),
        wallet_address: req.wallet_address.clone(),
    })
} 