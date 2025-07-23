use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CircuitBreakerState {
    Closed,   // 正常状态，允许请求通过
    Open,     // 断开状态，拒绝请求
    HalfOpen, // 半开状态，允许少量请求测试服务恢复
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,       // 失败阈值
    pub success_threshold: u32,       // 半开状态下的成功阈值
    pub timeout: Duration,            // 超时时间
    pub reset_timeout: Duration,      // 重置超时时间
    pub half_open_max_calls: u32,     // 半开状态下最大请求数
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(30),
            reset_timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
        }
    }
}

#[derive(Debug, Clone)]
struct CircuitBreakerData {
    state: CircuitBreakerState,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<Instant>,
    last_state_change: Instant,
    config: CircuitBreakerConfig,
    half_open_calls: u32,
}

impl CircuitBreakerData {
    fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            last_state_change: Instant::now(),
            config,
            half_open_calls: 0,
        }
    }
    
    fn record_success(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count = 0;
            }
            CircuitBreakerState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.transition_to_closed();
                }
            }
            CircuitBreakerState::Open => {
                // 开路状态下不应该有成功请求
            }
        }
    }
    
    fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());
        
        match self.state {
            CircuitBreakerState::Closed => {
                if self.failure_count >= self.config.failure_threshold {
                    self.transition_to_open();
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.transition_to_open();
            }
            CircuitBreakerState::Open => {
                // 已经是开路状态
            }
        }
    }
    
    fn can_execute(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if self.should_attempt_reset() {
                    self.transition_to_half_open();
                    true
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => {
                if self.half_open_calls < self.config.half_open_max_calls {
                    self.half_open_calls += 1;
                    true
                } else {
                    false
                }
            }
        }
    }
    
    fn should_attempt_reset(&self) -> bool {
        if let Some(last_failure) = self.last_failure_time {
            last_failure.elapsed() > self.config.reset_timeout
        } else {
            false
        }
    }
    
    fn transition_to_closed(&mut self) {
        log::info!("Circuit breaker transitioning to CLOSED state");
        self.state = CircuitBreakerState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.half_open_calls = 0;
        self.last_state_change = Instant::now();
    }
    
    fn transition_to_open(&mut self) {
        log::warn!("Circuit breaker transitioning to OPEN state - failures: {}", self.failure_count);
        self.state = CircuitBreakerState::Open;
        self.success_count = 0;
        self.half_open_calls = 0;
        self.last_state_change = Instant::now();
    }
    
    fn transition_to_half_open(&mut self) {
        log::info!("Circuit breaker transitioning to HALF_OPEN state");
        self.state = CircuitBreakerState::HalfOpen;
        self.success_count = 0;
        self.half_open_calls = 0;
        self.last_state_change = Instant::now();
    }
}

#[derive(Clone)]
pub struct CircuitBreaker {
    data: Arc<RwLock<CircuitBreakerData>>,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            data: Arc::new(RwLock::new(CircuitBreakerData::new(config))),
        }
    }
    
    pub async fn can_execute(&self) -> bool {
        let mut data = self.data.write().await;
        data.can_execute()
    }
    
    pub async fn record_success(&self) {
        let mut data = self.data.write().await;
        data.record_success();
    }
    
    pub async fn record_failure(&self) {
        let mut data = self.data.write().await;
        data.record_failure();
    }
    
    pub async fn get_state(&self) -> CircuitBreakerState {
        let data = self.data.read().await;
        data.state
    }
    
    pub async fn get_metrics(&self) -> CircuitBreakerMetrics {
        let data = self.data.read().await;
        CircuitBreakerMetrics {
            state: data.state,
            failure_count: data.failure_count,
            success_count: data.success_count,
            last_state_change: data.last_state_change,
            half_open_calls: data.half_open_calls,
        }
    }
    
    pub async fn reset(&self) {
        let mut data = self.data.write().await;
        data.transition_to_closed();
    }
}

pub struct CircuitBreakerManager {
    breakers: Arc<RwLock<HashMap<String, CircuitBreaker>>>,
    default_config: CircuitBreakerConfig,
}

impl CircuitBreakerManager {
    pub fn new() -> Self {
        Self {
            breakers: Arc::new(RwLock::new(HashMap::new())),
            default_config: CircuitBreakerConfig::default(),
        }
    }
    
    pub fn with_config(config: CircuitBreakerConfig) -> Self {
        Self {
            breakers: Arc::new(RwLock::new(HashMap::new())),
            default_config: config,
        }
    }
    
    async fn get_or_create_breaker(&self, instance_id: &str) -> CircuitBreaker {
        let breakers = self.breakers.read().await;
        if let Some(breaker) = breakers.get(instance_id) {
            return breaker.clone();
        }
        drop(breakers);
        
        let mut breakers = self.breakers.write().await;
        // 双重检查
        if let Some(breaker) = breakers.get(instance_id) {
            breaker.clone()
        } else {
            let breaker = CircuitBreaker::new(self.default_config.clone());
            breakers.insert(instance_id.to_string(), breaker.clone());
            breaker
        }
    }
    
    pub async fn can_execute(&self, instance_id: &str) -> bool {
        let breaker = self.get_or_create_breaker(instance_id).await;
        breaker.can_execute().await
    }
    
    pub async fn record_success(&self, instance_id: &str) {
        let breaker = self.get_or_create_breaker(instance_id).await;
        breaker.record_success().await;
    }
    
    pub async fn record_failure(&self, instance_id: &str) {
        let breaker = self.get_or_create_breaker(instance_id).await;
        breaker.record_failure().await;
    }
    
    pub async fn is_open(&self, instance_id: &str) -> bool {
        let breaker = self.get_or_create_breaker(instance_id).await;
        breaker.get_state().await == CircuitBreakerState::Open
    }
    
    pub async fn get_state(&self, instance_id: &str) -> CircuitBreakerState {
        let breaker = self.get_or_create_breaker(instance_id).await;
        breaker.get_state().await
    }
    
    pub async fn get_metrics(&self, instance_id: &str) -> CircuitBreakerMetrics {
        let breaker = self.get_or_create_breaker(instance_id).await;
        breaker.get_metrics().await
    }
    
    pub async fn get_all_metrics(&self) -> HashMap<String, CircuitBreakerMetrics> {
        let breakers = self.breakers.read().await;
        let mut metrics = HashMap::new();
        
        for (instance_id, breaker) in breakers.iter() {
            let metric = breaker.get_metrics().await;
            metrics.insert(instance_id.clone(), metric);
        }
        
        metrics
    }
    
    pub async fn reset(&self, instance_id: &str) {
        let breaker = self.get_or_create_breaker(instance_id).await;
        breaker.reset().await;
    }
    
    pub async fn reset_all(&self) {
        let breakers = self.breakers.read().await;
        for breaker in breakers.values() {
            breaker.reset().await;
        }
    }
    
    pub async fn cleanup_inactive(&self, max_age: Duration) {
        let mut breakers = self.breakers.write().await;
        let now = Instant::now();
        
        breakers.retain(|instance_id, breaker| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let metrics = breaker.get_metrics().await;
                    let age = now.duration_since(metrics.last_state_change);
                    if age > max_age {
                        log::info!("Cleaning up inactive circuit breaker for instance: {}", instance_id);
                        false
                    } else {
                        true
                    }
                })
            })
        });
    }
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerMetrics {
    pub state: CircuitBreakerState,
    pub failure_count: u32,
    pub success_count: u32,
    pub last_state_change: Instant,
    pub half_open_calls: u32,
}

// 断路器装饰器 - 用于包装函数调用
pub struct CircuitBreakerDecorator<F, T, E> {
    circuit_breaker: CircuitBreaker,
    function: F,
    _phantom: std::marker::PhantomData<(T, E)>,
}

impl<F, T, E, Fut> CircuitBreakerDecorator<F, T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    pub fn new(circuit_breaker: CircuitBreaker, function: F) -> Self {
        Self {
            circuit_breaker,
            function,
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub async fn execute(&self) -> Result<T, CircuitBreakerError<E>> {
        if !self.circuit_breaker.can_execute().await {
            return Err(CircuitBreakerError::CircuitOpen);
        }
        
        let start_time = Instant::now();
        match (self.function)().await {
            Ok(result) => {
                self.circuit_breaker.record_success().await;
                Ok(result)
            }
            Err(error) => {
                self.circuit_breaker.record_failure().await;
                Err(CircuitBreakerError::ExecutionFailed(error))
            }
        }
    }
}

#[derive(Debug)]
pub enum CircuitBreakerError<E> {
    CircuitOpen,
    ExecutionFailed(E),
}

impl<E: std::fmt::Display> std::fmt::Display for CircuitBreakerError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitBreakerError::CircuitOpen => write!(f, "Circuit breaker is open"),
            CircuitBreakerError::ExecutionFailed(e) => write!(f, "Execution failed: {}", e),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for CircuitBreakerError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CircuitBreakerError::CircuitOpen => None,
            CircuitBreakerError::ExecutionFailed(e) => Some(e),
        }
    }
} 