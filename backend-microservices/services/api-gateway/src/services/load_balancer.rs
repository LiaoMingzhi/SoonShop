use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::discovery::ServiceInstance;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    IpHash,
    ConsistentHash,
}

pub struct LoadBalancer {
    strategy: LoadBalancingStrategy,
    round_robin_counter: Arc<AtomicUsize>,
    connection_counts: Arc<RwLock<HashMap<String, usize>>>,
    weighted_indices: Arc<RwLock<Vec<String>>>,
}

impl LoadBalancer {
    pub fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            strategy,
            round_robin_counter: Arc::new(AtomicUsize::new(0)),
            connection_counts: Arc::new(RwLock::new(HashMap::new())),
            weighted_indices: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn select_instance<'a>(&self, instances: &'a [&'a ServiceInstance]) -> Result<&'a ServiceInstance> {
        if instances.is_empty() {
            return Err(anyhow::anyhow!("No instances available for load balancing"));
        }
        
        match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                self.round_robin_select(instances).await
            }
            LoadBalancingStrategy::WeightedRoundRobin => {
                self.weighted_round_robin_select(instances).await
            }
            LoadBalancingStrategy::LeastConnections => {
                self.least_connections_select(instances).await
            }
            LoadBalancingStrategy::Random => {
                self.random_select(instances).await
            }
            LoadBalancingStrategy::IpHash => {
                // IP哈希需要请求上下文，这里使用轮询作为后备
                self.round_robin_select(instances).await
            }
            LoadBalancingStrategy::ConsistentHash => {
                self.consistent_hash_select(instances).await
            }
        }
    }
    
    pub async fn select_instance_with_key<'a>(&self, instances: &'a [&'a ServiceInstance], key: &str) -> Result<&'a ServiceInstance> {
        if instances.is_empty() {
            return Err(anyhow::anyhow!("No instances available for load balancing"));
        }
        
        match self.strategy {
            LoadBalancingStrategy::IpHash => {
                self.ip_hash_select(instances, key).await
            }
            LoadBalancingStrategy::ConsistentHash => {
                self.consistent_hash_select_with_key(instances, key).await
            }
            _ => {
                self.select_instance(instances).await
            }
        }
    }
    
    pub async fn record_connection_start(&self, instance_id: &str) {
        let mut counts = self.connection_counts.write().await;
        *counts.entry(instance_id.to_string()).or_insert(0) += 1;
    }
    
    pub async fn record_connection_end(&self, instance_id: &str) {
        let mut counts = self.connection_counts.write().await;
        if let Some(count) = counts.get_mut(instance_id) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }
    
    async fn round_robin_select<'a>(&self, instances: &'a [&ServiceInstance]) -> Result<&'a ServiceInstance> {
        let index = self.round_robin_counter.fetch_add(1, Ordering::Relaxed) % instances.len();
        Ok(instances[index])
    }
    
    async fn weighted_round_robin_select<'a>(&self, instances: &'a [&ServiceInstance]) -> Result<&'a ServiceInstance> {
        // 构建加权索引
        let mut weighted_list = Vec::new();
        for instance in instances {
            for _ in 0..instance.weight {
                weighted_list.push(instance.id.clone());
            }
        }
        
        if weighted_list.is_empty() {
            return self.round_robin_select(instances).await;
        }
        
        let index = self.round_robin_counter.fetch_add(1, Ordering::Relaxed) % weighted_list.len();
        let selected_id = &weighted_list[index];
        
        // 找到对应的实例
        for instance in instances {
            if &instance.id == selected_id {
                return Ok(instance);
            }
        }
        
        // 后备方案
        self.round_robin_select(instances).await
    }
    
    async fn least_connections_select<'a>(&self, instances: &'a [&ServiceInstance]) -> Result<&'a ServiceInstance> {
        let counts = self.connection_counts.read().await;
        
        let mut min_connections = usize::MAX;
        let mut selected_instance = instances[0];
        
        for instance in instances {
            let connections = counts.get(&instance.id).unwrap_or(&0);
            if *connections < min_connections {
                min_connections = *connections;
                selected_instance = instance;
            }
        }
        
        Ok(selected_instance)
    }
    
    async fn random_select<'a>(&self, instances: &'a [&ServiceInstance]) -> Result<&'a ServiceInstance> {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..instances.len());
        Ok(instances[index])
    }
    
    async fn ip_hash_select<'a>(&self, instances: &'a [&ServiceInstance], ip: &str) -> Result<&'a ServiceInstance> {
        let hash = self.hash_string(ip);
        let index = hash % instances.len();
        Ok(instances[index])
    }
    
    async fn consistent_hash_select<'a>(&self, instances: &'a [&ServiceInstance]) -> Result<&'a ServiceInstance> {
        // 简化的一致性哈希，基于实例ID
        let key = instances.iter().map(|i| i.id.as_str()).collect::<Vec<_>>().join(",");
        let hash = self.hash_string(&key);
        let index = hash % instances.len();
        Ok(instances[index])
    }
    
    async fn consistent_hash_select_with_key<'a>(&self, instances: &'a [&ServiceInstance], key: &str) -> Result<&'a ServiceInstance> {
        let hash = self.hash_string(key);
        let index = hash % instances.len();
        Ok(instances[index])
    }
    
    fn hash_string(&self, s: &str) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish() as usize
    }
}

// 负载均衡配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    pub strategy: LoadBalancingStrategy,
    pub health_check_enabled: bool,
    pub sticky_sessions: bool,
    pub session_timeout_seconds: u64,
}

impl Default for LoadBalancerConfig {
    fn default() -> Self {
        Self {
            strategy: LoadBalancingStrategy::RoundRobin,
            health_check_enabled: true,
            sticky_sessions: false,
            session_timeout_seconds: 1800, // 30分钟
        }
    }
}

// 负载均衡统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerStats {
    pub strategy: LoadBalancingStrategy,
    pub total_requests: u64,
    pub instance_stats: HashMap<String, InstanceStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStats {
    pub instance_id: String,
    pub request_count: u64,
    pub active_connections: usize,
    pub average_response_time: f64,
    pub error_rate: f64,
    pub last_selected: chrono::DateTime<chrono::Utc>,
}

// 高级负载均衡器，带统计功能
pub struct AdvancedLoadBalancer {
    base: LoadBalancer,
    config: LoadBalancerConfig,
    stats: Arc<RwLock<LoadBalancerStats>>,
    request_counter: Arc<AtomicUsize>,
}

impl AdvancedLoadBalancer {
    pub fn new(config: LoadBalancerConfig) -> Self {
        let stats = LoadBalancerStats {
            strategy: config.strategy,
            total_requests: 0,
            instance_stats: HashMap::new(),
        };
        
        Self {
            base: LoadBalancer::new(config.strategy),
            config,
            stats: Arc::new(RwLock::new(stats)),
            request_counter: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub async fn select_instance<'a>(&self, instances: &'a [&'a ServiceInstance]) -> Result<&'a ServiceInstance> {
        let selected = self.base.select_instance(instances).await?;
        
        // 记录统计信息
        self.record_selection(selected).await;
        
        Ok(selected)
    }
    
    pub async fn select_instance_with_key<'a>(&self, instances: &'a [&'a ServiceInstance], key: &str) -> Result<&'a ServiceInstance> {
        let selected = self.base.select_instance_with_key(instances, key).await?;
        
        // 记录统计信息
        self.record_selection(selected).await;
        
        Ok(selected)
    }
    
    async fn record_selection(&self, instance: &ServiceInstance) {
        self.request_counter.fetch_add(1, Ordering::Relaxed);
        
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        
        let instance_stat = stats.instance_stats
            .entry(instance.id.clone())
            .or_insert_with(|| InstanceStats {
                instance_id: instance.id.clone(),
                request_count: 0,
                active_connections: 0,
                average_response_time: 0.0,
                error_rate: 0.0,
                last_selected: chrono::Utc::now(),
            });
        
        instance_stat.request_count += 1;
        instance_stat.last_selected = chrono::Utc::now();
    }
    
    pub async fn record_request_completed(&self, instance_id: &str, response_time_ms: u64, success: bool) {
        let mut stats = self.stats.write().await;
        
        if let Some(instance_stat) = stats.instance_stats.get_mut(instance_id) {
            // 更新平均响应时间（简单移动平均）
            let alpha = 0.1; // 平滑因子
            instance_stat.average_response_time = 
                alpha * response_time_ms as f64 + (1.0 - alpha) * instance_stat.average_response_time;
            
            // 更新错误率（简单移动平均）
            let error_value = if success { 0.0 } else { 1.0 };
            instance_stat.error_rate = 
                alpha * error_value + (1.0 - alpha) * instance_stat.error_rate;
        }
    }
    
    pub async fn get_stats(&self) -> LoadBalancerStats {
        let stats = self.stats.read().await;
        stats.clone()
    }
    
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        stats.total_requests = 0;
        stats.instance_stats.clear();
        self.request_counter.store(0, Ordering::Relaxed);
    }
} 