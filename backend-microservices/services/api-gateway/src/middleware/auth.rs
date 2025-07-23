use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, Result, HttpMessage,
    body::{MessageBody, EitherBody, BoxBody},
};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub struct AuthMiddleware {
    jwt_secret: String,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone();
        
        // 跳过认证的路径
        let skip_auth_paths = vec!["/health", "/metrics", "/auth/login", "/auth/register"];
        let path = req.path();
        
        if skip_auth_paths.iter().any(|&p| path.starts_with(p)) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_right_body())
            });
        }
        
        // 检查Authorization头
        let auth_header = req.headers().get("Authorization");
        
        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    
                    // 验证JWT
                    let validation = Validation::new(Algorithm::HS256);
                    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
                    
                    match decode::<Claims>(token, &decoding_key, &validation) {
                        Ok(token_data) => {
                            // 将用户信息添加到请求扩展中
                            req.extensions_mut().insert(token_data.claims);
                            
                            let fut = self.service.call(req);
                            return Box::pin(async move {
                                let res = fut.await?;
                                Ok(res.map_into_right_body())
                            });
                        }
                        Err(_) => {
                            return Box::pin(async move {
                                let (req, _) = req.into_parts();
                                let response = HttpResponse::Unauthorized()
                                    .json(serde_json::json!({
                                        "error": "Invalid token"
                                    }));
                                Ok(ServiceResponse::new(req, response).map_into_left_body())
                            });
                        }
                    }
                }
            }
        }
        
        // 未授权访问
        Box::pin(async move {
            let (req, _) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": "Missing or invalid authorization header"
                }));
            Ok(ServiceResponse::new(req, response).map_into_left_body())
        })
    }
} 