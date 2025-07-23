use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ready, Ready};
use prometheus::{
    Counter, Gauge, Histogram, HistogramOpts, IntCounter, IntGauge, Opts, Registry,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct MetricsMiddleware {
    registry: Arc<Registry>,
    http_requests_total: IntCounter,
    http_request_duration: Histogram,
    http_requests_in_flight: IntGauge,
}

impl MetricsMiddleware {
    pub fn new() -> Self {
        let registry = Arc::new(Registry::new());
        
        let http_requests_total = IntCounter::with_opts(
            Opts::new("http_requests_total", "Total number of HTTP requests")
                .namespace("api_gateway")
        ).unwrap();
        
        let http_request_duration = Histogram::with_opts(
            HistogramOpts::new("http_request_duration_seconds", "HTTP request duration")
                .namespace("api_gateway")
        ).unwrap();
        
        let http_requests_in_flight = IntGauge::with_opts(
            Opts::new("http_requests_in_flight", "Number of HTTP requests in flight")
                .namespace("api_gateway")
        ).unwrap();
        
        registry.register(Box::new(http_requests_total.clone())).unwrap();
        registry.register(Box::new(http_request_duration.clone())).unwrap();
        registry.register(Box::new(http_requests_in_flight.clone())).unwrap();
        
        Self {
            registry,
            http_requests_total,
            http_request_duration,
            http_requests_in_flight,
        }
    }
    
    pub fn registry(&self) -> Arc<Registry> {
        self.registry.clone()
    }
}

impl<S, B> Transform<S, ServiceRequest> for MetricsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MetricsMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(MetricsMiddlewareService {
            service,
            http_requests_total: self.http_requests_total.clone(),
            http_request_duration: self.http_request_duration.clone(),
            http_requests_in_flight: self.http_requests_in_flight.clone(),
        }))
    }
}

pub struct MetricsMiddlewareService<S> {
    service: S,
    http_requests_total: IntCounter,
    http_request_duration: Histogram,
    http_requests_in_flight: IntGauge,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddlewareService<S>
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
        let http_requests_total = self.http_requests_total.clone();
        let http_request_duration = self.http_request_duration.clone();
        let http_requests_in_flight = self.http_requests_in_flight.clone();
        
        // 增加进行中的请求数
        http_requests_in_flight.inc();
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            
            // 记录请求指标
            http_requests_total.inc();
            let duration = start_time.elapsed().as_secs_f64();
            http_request_duration.observe(duration);
            
            // 减少进行中的请求数
            http_requests_in_flight.dec();
            
            Ok(res)
        })
    }
} 