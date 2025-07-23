use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub url: String,
    pub healthy: bool,
    pub weight: u32,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub last_check: DateTime<Utc>,
}

impl ServiceInstance {
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            url,
            healthy: true,
            weight: 1,
            tags: Vec::new(),
            metadata: HashMap::new(),
            last_check: Utc::now(),
        }
    }
    
    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DiscoveryBackend {
    InMemory,
    Consul,
    Etcd,
    Kubernetes,
}

pub struct ServiceDiscovery {
    backend: DiscoveryBackend,
    config: AppConfig,
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
}

impl ServiceDiscovery {
    pub fn new(config: AppConfig) -> Self {
        Self {
            backend: DiscoveryBackend::InMemory, // 默认使用内存后端
            config,
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn with_backend(mut self, backend: DiscoveryBackend) -> Self {
        self.backend = backend;
        self
    }
    
    pub async fn register_service(&self, instance: ServiceInstance) -> Result<()> {
        match self.backend {
            DiscoveryBackend::InMemory => {
                self.register_in_memory(instance).await
            }
            DiscoveryBackend::Consul => {
                self.register_consul(instance).await
            }
            DiscoveryBackend::Etcd => {
                self.register_etcd(instance).await
            }
            DiscoveryBackend::Kubernetes => {
                self.register_kubernetes(instance).await
            }
        }
    }
    
    pub async fn deregister_service(&self, service_name: &str, instance_id: &str) -> Result<()> {
        match self.backend {
            DiscoveryBackend::InMemory => {
                self.deregister_in_memory(service_name, instance_id).await
            }
            DiscoveryBackend::Consul => {
                self.deregister_consul(service_name, instance_id).await
            }
            DiscoveryBackend::Etcd => {
                self.deregister_etcd(service_name, instance_id).await
            }
            DiscoveryBackend::Kubernetes => {
                self.deregister_kubernetes(service_name, instance_id).await
            }
        }
    }
    
    pub async fn discover_services(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        match self.backend {
            DiscoveryBackend::InMemory => {
                self.discover_in_memory(service_name).await
            }
            DiscoveryBackend::Consul => {
                self.discover_consul(service_name).await
            }
            DiscoveryBackend::Etcd => {
                self.discover_etcd(service_name).await
            }
            DiscoveryBackend::Kubernetes => {
                self.discover_kubernetes(service_name).await
            }
        }
    }
    
    pub async fn list_all_services(&self) -> Result<HashMap<String, Vec<ServiceInstance>>> {
        match self.backend {
            DiscoveryBackend::InMemory => {
                let services = self.services.read().await;
                Ok(services.clone())
            }
            _ => {
                // 对于外部服务发现系统，需要实现相应的列表方法
                Ok(HashMap::new())
            }
        }
    }
    
    // 内存后端实现
    async fn register_in_memory(&self, instance: ServiceInstance) -> Result<()> {
        let mut services = self.services.write().await;
        services
            .entry(instance.name.clone())
            .or_insert_with(Vec::new)
            .push(instance);
        
        log::info!("Registered service instance in memory");
        Ok(())
    }
    
    async fn deregister_in_memory(&self, service_name: &str, instance_id: &str) -> Result<()> {
        let mut services = self.services.write().await;
        if let Some(instances) = services.get_mut(service_name) {
            instances.retain(|instance| instance.id != instance_id);
            if instances.is_empty() {
                services.remove(service_name);
            }
        }
        
        log::info!("Deregistered service instance from memory");
        Ok(())
    }
    
    async fn discover_in_memory(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        let services = self.services.read().await;
        Ok(services.get(service_name).cloned().unwrap_or_default())
    }
    
    // Consul后端实现（占位符）
    async fn register_consul(&self, instance: ServiceInstance) -> Result<()> {
        // TODO: 实现Consul服务注册
        log::warn!("Consul backend not implemented yet");
        Ok(())
    }
    
    async fn deregister_consul(&self, _service_name: &str, _instance_id: &str) -> Result<()> {
        // TODO: 实现Consul服务注销
        log::warn!("Consul backend not implemented yet");
        Ok(())
    }
    
    async fn discover_consul(&self, _service_name: &str) -> Result<Vec<ServiceInstance>> {
        // TODO: 实现Consul服务发现
        log::warn!("Consul backend not implemented yet");
        Ok(Vec::new())
    }
    
    // Etcd后端实现（占位符）
    async fn register_etcd(&self, _instance: ServiceInstance) -> Result<()> {
        // TODO: 实现Etcd服务注册
        log::warn!("Etcd backend not implemented yet");
        Ok(())
    }
    
    async fn deregister_etcd(&self, _service_name: &str, _instance_id: &str) -> Result<()> {
        // TODO: 实现Etcd服务注销
        log::warn!("Etcd backend not implemented yet");
        Ok(())
    }
    
    async fn discover_etcd(&self, _service_name: &str) -> Result<Vec<ServiceInstance>> {
        // TODO: 实现Etcd服务发现
        log::warn!("Etcd backend not implemented yet");
        Ok(Vec::new())
    }
    
    // Kubernetes后端实现（占位符）
    async fn register_kubernetes(&self, _instance: ServiceInstance) -> Result<()> {
        // TODO: 实现Kubernetes服务注册
        log::warn!("Kubernetes backend not implemented yet");
        Ok(())
    }
    
    async fn deregister_kubernetes(&self, _service_name: &str, _instance_id: &str) -> Result<()> {
        // TODO: 实现Kubernetes服务注销
        log::warn!("Kubernetes backend not implemented yet");
        Ok(())
    }
    
    async fn discover_kubernetes(&self, _service_name: &str) -> Result<Vec<ServiceInstance>> {
        // TODO: 实现Kubernetes服务发现
        log::warn!("Kubernetes backend not implemented yet");
        Ok(Vec::new())
    }
}

// 服务发现事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryEvent {
    ServiceRegistered {
        service_name: String,
        instance: ServiceInstance,
    },
    ServiceDeregistered {
        service_name: String,
        instance_id: String,
    },
    ServiceHealthChanged {
        service_name: String,
        instance_id: String,
        healthy: bool,
    },
}

// 服务查询过滤器
#[derive(Debug, Clone, Default)]
pub struct ServiceQuery {
    pub name: Option<String>,
    pub tags: Vec<String>,
    pub healthy_only: bool,
    pub metadata_filters: HashMap<String, String>,
}

impl ServiceQuery {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn healthy_only(mut self) -> Self {
        self.healthy_only = true;
        self
    }
    
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata_filters.insert(key, value);
        self
    }
    
    pub fn matches(&self, instance: &ServiceInstance) -> bool {
        // 检查健康状态
        if self.healthy_only && !instance.healthy {
            return false;
        }
        
        // 检查标签
        if !self.tags.is_empty() {
            let has_all_tags = self.tags.iter().all(|tag| instance.tags.contains(tag));
            if !has_all_tags {
                return false;
            }
        }
        
        // 检查元数据
        for (key, value) in &self.metadata_filters {
            if instance.metadata.get(key) != Some(value) {
                return false;
            }
        }
        
        true
    }
} 