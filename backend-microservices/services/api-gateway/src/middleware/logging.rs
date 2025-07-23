use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ready, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::{info, error, warn};

pub struct LoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService { service }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = std::time::Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        let remote_addr = req.connection_info().remote_addr().unwrap_or("unknown").to_string();
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let status = res.status();
            
            match status.as_u16() {
                200..=299 => {
                    info!(
                        method = %method,
                        path = %path,
                        status = %status,
                        duration_ms = %duration.as_millis(),
                        remote_addr = %remote_addr,
                        "Request completed successfully"
                    );
                }
                400..=499 => {
                    warn!(
                        method = %method,
                        path = %path,
                        status = %status,
                        duration_ms = %duration.as_millis(),
                        remote_addr = %remote_addr,
                        "Client error"
                    );
                }
                500..=599 => {
                    error!(
                        method = %method,
                        path = %path,
                        status = %status,
                        duration_ms = %duration.as_millis(),
                        remote_addr = %remote_addr,
                        "Server error"
                    );
                }
                _ => {
                    info!(
                        method = %method,
                        path = %path,
                        status = %status,
                        duration_ms = %duration.as_millis(),
                        remote_addr = %remote_addr,
                        "Request completed"
                    );
                }
            }
            
            Ok(res)
        })
    }
} 