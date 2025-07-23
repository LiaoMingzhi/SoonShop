use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, Result,
    body::{MessageBody, EitherBody, BoxBody},
};
use futures::future::{ready, Ready};
use governor::{clock::DefaultClock, state::keyed::DashMapStateStore, Quota, RateLimiter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::num::NonZeroU32;

use crate::config::RateLimitConfig;

pub struct RateLimitMiddleware {
    config: RateLimitConfig,
}

impl RateLimitMiddleware {
    pub fn new(config: RateLimitConfig) -> Self {
        Self { config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let burst_size = NonZeroU32::new(self.config.burst_size.max(1)).unwrap_or(NonZeroU32::new(10).unwrap());
        let requests_per_minute = NonZeroU32::new(self.config.requests_per_minute.max(1)).unwrap_or(NonZeroU32::new(60).unwrap());
        
        let limiter = RateLimiter::keyed(
            Quota::per_minute(requests_per_minute).allow_burst(burst_size)
        );
        
        ready(Ok(RateLimitMiddlewareService {
            service,
            limiter: Arc::new(limiter),
            config: self.config.clone(),
        }))
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: S,
    limiter: Arc<RateLimiter<String, DashMapStateStore<String>, DefaultClock>>,
    config: RateLimitConfig,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
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
        let limiter = self.limiter.clone();
        
        // 获取客户端IP
        let client_ip = req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string();
        
        // 检查限流
        match limiter.check_key(&client_ip) {
            Ok(_) => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res.map_into_right_body())
                })
            }
            Err(_) => {
                Box::pin(async move {
                    let (req, _) = req.into_parts();
                    let response = HttpResponse::TooManyRequests()
                        .json(serde_json::json!({
                            "error": "Too many requests",
                            "message": "Rate limit exceeded"
                        }));
                    Ok(ServiceResponse::new(req, response).map_into_left_body())
                })
            }
        }
    }
} 