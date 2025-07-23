pub mod discovery;
pub mod load_balancer;
pub mod circuit_breaker;
pub mod health_checker;

use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;
use tokio::sync::RwLock;
use anyhow::Result;

use crate::config::AppConfig;
use discovery::{ServiceDiscovery, ServiceInstance};
use load_balancer::{LoadBalancer, LoadBalancingStrategy};
use circuit_breaker::CircuitBreakerManager;
use health_checker::HealthChecker;

#[derive(Clone)]
pub struct ServiceRegistry {
    pub http_client: Client,
    pub config: Arc<AppConfig>,
    pub service_discovery: Arc<ServiceDiscovery>,
    pub load_balancer: Arc<LoadBalancer>,
    pub circuit_breaker: Arc<CircuitBreakerManager>,
    pub health_checker: Arc<HealthChecker>,
    pub services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
}

impl ServiceRegistry {
    pub async fn new(config: &AppConfig) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(20)
            .build()?;
        
        let services = Arc::new(RwLock::new(HashMap::new()));
        let service_discovery = Arc::new(ServiceDiscovery::new(config.clone()));
        let load_balancer = Arc::new(LoadBalancer::new(LoadBalancingStrategy::RoundRobin));
        let circuit_breaker = Arc::new(CircuitBreakerManager::new());
        let health_checker = Arc::new(HealthChecker::new(http_client.clone()));
        
        let registry = ServiceRegistry {
            http_client,
            config: Arc::new(config.clone()),
            service_discovery: service_discovery.clone(),
            load_balancer,
            circuit_breaker,
            health_checker: health_checker.clone(),
            services: services.clone(),
        };
        
        // 初始化服务发现
        registry.initialize_services().await?;
        
        // 启动健康检查
        registry.start_health_monitoring().await;
        
        Ok(registry)
    }
    
    async fn initialize_services(&self) -> Result<()> {
        let mut services = self.services.write().await;
        
        // 从配置中注册服务
        let service_configs = vec![
            ("user-service", &self.config.services.user_service),
            ("product-service", &self.config.services.product_service),
            ("order-service", &self.config.services.order_service),
            ("payment-service", &self.config.services.payment_service),
            ("voucher-service", &self.config.services.voucher_service),
            ("reward-service", &self.config.services.reward_service),
            ("evaluation-service", &self.config.services.evaluation_service),
            ("notification-service", &self.config.services.notification_service),
        ];
        
        for (service_name, url) in service_configs {
            let instance = ServiceInstance {
                id: format!("{}-1", service_name),
                name: service_name.to_string(),
                url: url.clone(),
                healthy: true,
                weight: 1,
                tags: vec!["primary".to_string()],
                metadata: HashMap::new(),
                last_check: chrono::Utc::now(),
            };
            
            services.insert(service_name.to_string(), vec![instance]);
        }
        
        Ok(())
    }
    
    async fn start_health_monitoring(&self) {
        let health_checker = self.health_checker.clone();
        let services = self.services.clone();
        let circuit_breaker = self.circuit_breaker.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                let services_snapshot = {
                    let services = services.read().await;
                    services.clone()
                };
                
                for (service_name, instances) in services_snapshot {
                    for instance in instances {
                        let health_result = health_checker.check_health(&instance).await;
                        
                        match health_result {
                            Ok(healthy) => {
                                if !healthy {
                                    log::warn!("Service {} instance {} is unhealthy", service_name, instance.id);
                                    circuit_breaker.record_failure(&instance.id).await;
                                } else {
                                    circuit_breaker.record_success(&instance.id).await;
                                }
                                
                                // 更新服务健康状态
                                let mut services = services.write().await;
                                if let Some(instances) = services.get_mut(&service_name) {
                                    for inst in instances.iter_mut() {
                                        if inst.id == instance.id {
                                            inst.healthy = healthy;
                                            inst.last_check = chrono::Utc::now();
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Health check failed for {} {}: {}", service_name, instance.id, e);
                                circuit_breaker.record_failure(&instance.id).await;
                            }
                        }
                    }
                }
            }
        });
    }
    
    pub async fn get_service_instance(&self, service_name: &str) -> Result<ServiceInstance> {
        let services = self.services.read().await;
        
        if let Some(instances) = services.get(service_name) {
            let mut healthy_instances = Vec::new();
            
            for instance in instances {
                if instance.healthy && !self.circuit_breaker.is_open(&instance.id).await {
                    healthy_instances.push(instance);
                }
            }
            
            if healthy_instances.is_empty() {
                return Err(anyhow::anyhow!("No healthy instances available for service: {}", service_name));
            }
            
            let selected = self.load_balancer.select_instance(&healthy_instances).await?;
            Ok(selected.clone())
        } else {
            Err(anyhow::anyhow!("Service not found: {}", service_name))
        }
    }
    
    pub async fn register_service(&self, instance: ServiceInstance) -> Result<()> {
        self.service_discovery.register_service(instance.clone()).await?;
        
        let mut services = self.services.write().await;
        services
            .entry(instance.name.clone())
            .or_insert_with(Vec::new)
            .push(instance);
        
        Ok(())
    }
    
    pub async fn deregister_service(&self, service_name: &str, instance_id: &str) -> Result<()> {
        self.service_discovery.deregister_service(service_name, instance_id).await?;
        
        let mut services = self.services.write().await;
        if let Some(instances) = services.get_mut(service_name) {
            instances.retain(|instance| instance.id != instance_id);
        }
        
        Ok(())
    }
    
    pub async fn get_service_stats(&self) -> Result<ServiceStats> {
        let services = self.services.read().await;
        let mut stats = ServiceStats {
            total_services: services.len(),
            total_instances: 0,
            healthy_instances: 0,
            service_details: HashMap::new(),
        };
        
        for (service_name, instances) in services.iter() {
            let healthy_count = instances.iter().filter(|i| i.healthy).count();
            stats.total_instances += instances.len();
            stats.healthy_instances += healthy_count;
            
            stats.service_details.insert(service_name.clone(), ServiceDetail {
                total_instances: instances.len(),
                healthy_instances: healthy_count,
                instances: instances.clone(),
            });
        }
        
        Ok(stats)
    }
    
    pub async fn health_check(&self) -> Result<()> {
        let stats = self.get_service_stats().await?;
        
        if stats.healthy_instances == 0 {
            return Err(anyhow::anyhow!("No healthy service instances available"));
        }
        
        log::info!("Service registry health: {}/{} instances healthy", 
                  stats.healthy_instances, stats.total_instances);
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceStats {
    pub total_services: usize,
    pub total_instances: usize,
    pub healthy_instances: usize,
    pub service_details: HashMap<String, ServiceDetail>,
}

#[derive(Debug, Clone)]
pub struct ServiceDetail {
    pub total_instances: usize,
    pub healthy_instances: usize,
    pub instances: Vec<ServiceInstance>,
} 