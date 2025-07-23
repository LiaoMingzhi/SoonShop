use reqwest::Client;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tokio::time::timeout;

use super::discovery::ServiceInstance;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Warning,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub timeout: Duration,
    pub interval: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub endpoints: Vec<String>,
    pub expected_status_codes: Vec<u16>,
    pub check_types: Vec<HealthCheckType>,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            interval: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(5),
            endpoints: vec!["/health".to_string(), "/ping".to_string()],
            expected_status_codes: vec![200, 204],
            check_types: vec![HealthCheckType::Http, HealthCheckType::Tcp],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthCheckType {
    Http,
    Tcp,
    Grpc,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub status: HealthStatus,
    pub response_time: Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message: Option<String>,
    pub details: HashMap<String, serde_json::Value>,
}

impl HealthCheckResult {
    pub fn healthy(response_time: Duration) -> Self {
        Self {
            status: HealthStatus::Healthy,
            response_time,
            timestamp: chrono::Utc::now(),
            message: None,
            details: HashMap::new(),
        }
    }
    
    pub fn unhealthy(message: String, response_time: Duration) -> Self {
        Self {
            status: HealthStatus::Unhealthy,
            response_time,
            timestamp: chrono::Utc::now(),
            message: Some(message),
            details: HashMap::new(),
        }
    }
    
    pub fn warning(message: String, response_time: Duration) -> Self {
        Self {
            status: HealthStatus::Warning,
            response_time,
            timestamp: chrono::Utc::now(),
            message: Some(message),
            details: HashMap::new(),
        }
    }
    
    pub fn with_details(mut self, details: HashMap<String, serde_json::Value>) -> Self {
        self.details = details;
        self
    }
}

#[derive(Clone)]
pub struct HealthChecker {
    client: Client,
    config: HealthCheckConfig,
}

impl HealthChecker {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            config: HealthCheckConfig::default(),
        }
    }
    
    pub fn with_config(client: Client, config: HealthCheckConfig) -> Self {
        Self {
            client,
            config,
        }
    }
    
    pub async fn check_health(&self, instance: &ServiceInstance) -> Result<bool> {
        let result = self.perform_health_check(instance).await?;
        Ok(matches!(result.status, HealthStatus::Healthy | HealthStatus::Warning))
    }
    
    pub async fn perform_health_check(&self, instance: &ServiceInstance) -> Result<HealthCheckResult> {
        let mut last_error = None;
        
        for attempt in 0..=self.config.max_retries {
            if attempt > 0 {
                tokio::time::sleep(self.config.retry_delay).await;
            }
            
            match self.execute_check(instance).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    log::warn!("Health check attempt {} failed for {}: {}", 
                              attempt + 1, instance.id, last_error.as_ref().unwrap());
                }
            }
        }
        
        // 所有重试都失败了
        Ok(HealthCheckResult::unhealthy(
            format!("All {} attempts failed: {}", 
                   self.config.max_retries + 1, 
                   last_error.unwrap()),
            Duration::from_secs(0)
        ))
    }
    
    async fn execute_check(&self, instance: &ServiceInstance) -> Result<HealthCheckResult> {
        let start_time = Instant::now();
        
        // 尝试所有配置的检查类型
        for check_type in &self.config.check_types {
            match check_type {
                HealthCheckType::Http => {
                    if let Ok(result) = self.http_health_check(instance, start_time).await {
                        return Ok(result);
                    }
                }
                HealthCheckType::Tcp => {
                    if let Ok(result) = self.tcp_health_check(instance, start_time).await {
                        return Ok(result);
                    }
                }
                HealthCheckType::Grpc => {
                    if let Ok(result) = self.grpc_health_check(instance, start_time).await {
                        return Ok(result);
                    }
                }
                HealthCheckType::Custom(name) => {
                    if let Ok(result) = self.custom_health_check(instance, name, start_time).await {
                        return Ok(result);
                    }
                }
            }
        }
        
        Err(anyhow::anyhow!("All health check types failed"))
    }
    
    async fn http_health_check(&self, instance: &ServiceInstance, start_time: Instant) -> Result<HealthCheckResult> {
        for endpoint in &self.config.endpoints {
            let url = format!("{}{}", instance.url, endpoint);
            
            let response = timeout(
                self.config.timeout,
                self.client.get(&url)
                    .header("User-Agent", "SoonShop-API-Gateway/1.0")
                    .send()
            ).await;
            
            let response_time = start_time.elapsed();
            
            match response {
                Ok(Ok(resp)) => {
                    if self.config.expected_status_codes.contains(&resp.status().as_u16()) {
                        let mut details = HashMap::new();
                        details.insert("status_code".to_string(), 
                                      serde_json::Value::Number(resp.status().as_u16().into()));
                        details.insert("endpoint".to_string(), 
                                      serde_json::Value::String(endpoint.clone()));
                        
                        // 尝试读取响应体获取额外的健康信息
                        if let Ok(body) = resp.text().await {
                            if let Ok(health_data) = serde_json::from_str::<serde_json::Value>(&body) {
                                details.insert("response_body".to_string(), health_data);
                            }
                        }
                        
                        return Ok(HealthCheckResult::healthy(response_time).with_details(details));
                    } else {
                        return Ok(HealthCheckResult::unhealthy(
                            format!("Unexpected status code: {}", resp.status()),
                            response_time
                        ));
                    }
                }
                Ok(Err(e)) => {
                    return Ok(HealthCheckResult::unhealthy(
                        format!("HTTP request failed: {}", e),
                        response_time
                    ));
                }
                Err(_) => {
                    return Ok(HealthCheckResult::unhealthy(
                        "HTTP request timeout".to_string(),
                        response_time
                    ));
                }
            }
        }
        
        Err(anyhow::anyhow!("No valid HTTP endpoints"))
    }
    
    async fn tcp_health_check(&self, instance: &ServiceInstance, start_time: Instant) -> Result<HealthCheckResult> {
        use tokio::net::TcpStream;
        
        // 从URL解析主机和端口
        let url = url::Url::parse(&instance.url)?;
        let host = url.host_str().ok_or_else(|| anyhow::anyhow!("Invalid host"))?;
        let port = url.port().unwrap_or(match url.scheme() {
            "https" => 443,
            "http" => 80,
            _ => return Err(anyhow::anyhow!("Unknown scheme")),
        });
        
        let connect_result = timeout(
            self.config.timeout,
            TcpStream::connect(format!("{}:{}", host, port))
        ).await;
        
        let response_time = start_time.elapsed();
        
        match connect_result {
            Ok(Ok(_)) => {
                let mut details = HashMap::new();
                details.insert("host".to_string(), serde_json::Value::String(host.to_string()));
                details.insert("port".to_string(), serde_json::Value::Number(port.into()));
                
                Ok(HealthCheckResult::healthy(response_time).with_details(details))
            }
            Ok(Err(e)) => {
                Ok(HealthCheckResult::unhealthy(
                    format!("TCP connection failed: {}", e),
                    response_time
                ))
            }
            Err(_) => {
                Ok(HealthCheckResult::unhealthy(
                    "TCP connection timeout".to_string(),
                    response_time
                ))
            }
        }
    }
    
    async fn grpc_health_check(&self, _instance: &ServiceInstance, start_time: Instant) -> Result<HealthCheckResult> {
        // TODO: 实现gRPC健康检查
        let response_time = start_time.elapsed();
        Ok(HealthCheckResult::warning(
            "gRPC health check not implemented yet".to_string(),
            response_time
        ))
    }
    
    async fn custom_health_check(&self, _instance: &ServiceInstance, _check_name: &str, start_time: Instant) -> Result<HealthCheckResult> {
        // TODO: 实现自定义健康检查
        let response_time = start_time.elapsed();
        Ok(HealthCheckResult::warning(
            "Custom health check not implemented yet".to_string(),
            response_time
        ))
    }
}

// 高级健康检查器，带历史记录和统计
#[derive(Clone)]
pub struct AdvancedHealthChecker {
    base: HealthChecker,
    history: std::sync::Arc<tokio::sync::RwLock<HashMap<String, Vec<HealthCheckResult>>>>,
    max_history_size: usize,
}

impl AdvancedHealthChecker {
    pub fn new(client: Client, config: HealthCheckConfig) -> Self {
        Self {
            base: HealthChecker::with_config(client, config),
            history: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            max_history_size: 100,
        }
    }
    
    pub fn with_history_size(mut self, size: usize) -> Self {
        self.max_history_size = size;
        self
    }
    
    pub async fn check_health(&self, instance: &ServiceInstance) -> Result<bool> {
        let result = self.base.perform_health_check(instance).await?;
        self.record_result(&instance.id, result.clone()).await;
        
        Ok(matches!(result.status, HealthStatus::Healthy | HealthStatus::Warning))
    }
    
    pub async fn get_health_result(&self, instance: &ServiceInstance) -> Result<HealthCheckResult> {
        let result = self.base.perform_health_check(instance).await?;
        self.record_result(&instance.id, result.clone()).await;
        Ok(result)
    }
    
    async fn record_result(&self, instance_id: &str, result: HealthCheckResult) {
        let mut history = self.history.write().await;
        let instance_history = history.entry(instance_id.to_string()).or_insert_with(Vec::new);
        
        instance_history.push(result);
        
        // 限制历史记录大小
        if instance_history.len() > self.max_history_size {
            instance_history.remove(0);
        }
    }
    
    pub async fn get_health_history(&self, instance_id: &str) -> Vec<HealthCheckResult> {
        let history = self.history.read().await;
        history.get(instance_id).cloned().unwrap_or_default()
    }
    
    pub async fn get_health_statistics(&self, instance_id: &str) -> HealthStatistics {
        let history = self.get_health_history(instance_id).await;
        
        if history.is_empty() {
            return HealthStatistics::default();
        }
        
        let total_checks = history.len();
        let healthy_checks = history.iter().filter(|r| r.status == HealthStatus::Healthy).count();
        let warning_checks = history.iter().filter(|r| r.status == HealthStatus::Warning).count();
        let unhealthy_checks = history.iter().filter(|r| r.status == HealthStatus::Unhealthy).count();
        
        let avg_response_time = history.iter()
            .map(|r| r.response_time.as_millis() as f64)
            .sum::<f64>() / total_checks as f64;
        
        let uptime_percentage = (healthy_checks + warning_checks) as f64 / total_checks as f64 * 100.0;
        
        HealthStatistics {
            total_checks,
            healthy_checks,
            warning_checks,
            unhealthy_checks,
            uptime_percentage,
            average_response_time_ms: avg_response_time,
            last_check: history.last().cloned(),
        }
    }
    
    pub async fn cleanup_old_history(&self, max_age: Duration) {
        let mut history = self.history.write().await;
        let cutoff_time = chrono::Utc::now() - chrono::Duration::from_std(max_age).unwrap();
        
        for (instance_id, results) in history.iter_mut() {
            results.retain(|result| result.timestamp > cutoff_time);
            if results.is_empty() {
                log::info!("Cleaning up empty health history for instance: {}", instance_id);
            }
        }
        
        history.retain(|_, results| !results.is_empty());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatistics {
    pub total_checks: usize,
    pub healthy_checks: usize,
    pub warning_checks: usize,
    pub unhealthy_checks: usize,
    pub uptime_percentage: f64,
    pub average_response_time_ms: f64,
    pub last_check: Option<HealthCheckResult>,
}

impl Default for HealthStatistics {
    fn default() -> Self {
        Self {
            total_checks: 0,
            healthy_checks: 0,
            warning_checks: 0,
            unhealthy_checks: 0,
            uptime_percentage: 0.0,
            average_response_time_ms: 0.0,
            last_check: None,
        }
    }
}

// 健康检查调度器
pub struct HealthCheckScheduler {
    checker: AdvancedHealthChecker,
    instances: std::sync::Arc<tokio::sync::RwLock<HashMap<String, ServiceInstance>>>,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl HealthCheckScheduler {
    pub fn new(checker: AdvancedHealthChecker) -> Self {
        Self {
            checker,
            instances: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            running: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    pub async fn add_instance(&self, instance: ServiceInstance) {
        let mut instances = self.instances.write().await;
        instances.insert(instance.id.clone(), instance);
    }
    
    pub async fn remove_instance(&self, instance_id: &str) {
        let mut instances = self.instances.write().await;
        instances.remove(instance_id);
    }
    
    pub async fn start(&self, interval: Duration) {
        self.running.store(true, std::sync::atomic::Ordering::Relaxed);
        let checker = self.checker.clone();
        let instances = self.instances.clone();
        let running = self.running.clone();
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            while running.load(std::sync::atomic::Ordering::Relaxed) {
                interval_timer.tick().await;
                
                let instances_snapshot = {
                    let instances = instances.read().await;
                    instances.clone()
                };
                
                for instance in instances_snapshot.values() {
                    let checker = checker.clone();
                    let instance = instance.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = checker.check_health(&instance).await {
                            log::error!("Health check failed for {}: {}", instance.id, e);
                        }
                    });
                }
            }
        });
    }
    
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
    }
} 