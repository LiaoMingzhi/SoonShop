use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBusMetrics {
    pub events_published: u64,
    pub events_consumed: u64,
    pub events_failed: u64,
    pub average_processing_time: f64,
    pub active_connections: u64,
    pub queue_sizes: HashMap<String, u64>,
    pub last_updated: DateTime<Utc>,
}

/// 指标收集器
pub struct MetricsCollector {
    events_published: Arc<AtomicU64>,
    events_processed: Arc<AtomicU64>,
    events_failed: Arc<AtomicU64>,
    start_time: DateTime<Utc>,
    processing_times: Arc<std::sync::Mutex<Vec<f64>>>,
    active_connections: AtomicU64,
    queue_sizes: Arc<std::sync::Mutex<HashMap<String, u64>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            events_published: Arc::new(AtomicU64::new(0)),
            events_processed: Arc::new(AtomicU64::new(0)),
            events_failed: Arc::new(AtomicU64::new(0)),
            start_time: Utc::now(),
            processing_times: Arc::new(std::sync::Mutex::new(Vec::new())),
            active_connections: AtomicU64::new(0),
            queue_sizes: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }
    
    pub fn increment_published(&self) {
        self.events_published.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn increment_processed(&self) {
        self.events_processed.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn increment_failed(&self) {
        self.events_failed.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn get_published_count(&self) -> u64 {
        self.events_published.load(Ordering::SeqCst)
    }
    
    pub fn get_processed_count(&self) -> u64 {
        self.events_processed.load(Ordering::SeqCst)
    }
    
    pub fn get_failed_count(&self) -> u64 {
        self.events_failed.load(Ordering::SeqCst)
    }
    
    pub fn get_uptime(&self) -> chrono::Duration {
        Utc::now() - self.start_time
    }

    pub fn record_processing_time(&self, time: f64) {
        if let Ok(mut times) = self.processing_times.lock() {
            times.push(time);
            // Keep only the last 1000 times for average calculation
            if times.len() > 1000 {
                times.drain(0..times.len() - 1000);
            }
        }
    }

    pub fn set_active_connections(&self, count: u64) {
        self.active_connections.store(count, Ordering::SeqCst);
    }

    pub fn set_queue_size(&self, queue: String, size: u64) {
        if let Ok(mut sizes) = self.queue_sizes.lock() {
            sizes.insert(queue, size);
        }
    }

    pub fn get_metrics(&self) -> EventBusMetrics {
        let avg_time = if let Ok(times) = self.processing_times.lock() {
            if times.is_empty() {
                0.0
            } else {
                times.iter().sum::<f64>() / times.len() as f64
            }
        } else {
            0.0
        };

        let queue_sizes = if let Ok(sizes) = self.queue_sizes.lock() {
            sizes.clone()
        } else {
            HashMap::new()
        };

        EventBusMetrics {
            events_published: self.events_published.load(Ordering::SeqCst),
            events_consumed: self.events_processed.load(Ordering::SeqCst),
            events_failed: self.events_failed.load(Ordering::SeqCst),
            average_processing_time: avg_time,
            active_connections: self.active_connections.load(Ordering::SeqCst),
            queue_sizes,
            last_updated: Utc::now(),
        }
    }

    pub fn reset(&self) {
        self.events_published.store(0, Ordering::SeqCst);
        self.events_processed.store(0, Ordering::SeqCst);
        self.events_failed.store(0, Ordering::SeqCst);
        if let Ok(mut times) = self.processing_times.lock() {
            times.clear();
        }
        if let Ok(mut sizes) = self.queue_sizes.lock() {
            sizes.clear();
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
} 