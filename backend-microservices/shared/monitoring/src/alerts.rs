use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use async_trait::async_trait;

/// 告警级别
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertLevel::Info => write!(f, "info"),
            AlertLevel::Warning => write!(f, "warning"),
            AlertLevel::Error => write!(f, "error"),
            AlertLevel::Critical => write!(f, "critical"),
        }
    }
}

/// 告警状态
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
    Silenced,
}

impl std::fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertStatus::Active => write!(f, "active"),
            AlertStatus::Acknowledged => write!(f, "acknowledged"),
            AlertStatus::Resolved => write!(f, "resolved"),
            AlertStatus::Silenced => write!(f, "silenced"),
        }
    }
}

/// 告警
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub rule_id: String,
    pub level: AlertLevel,
    pub status: AlertStatus,
    pub title: String,
    pub message: String,
    pub service: String,
    pub instance: String,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<String>,
    pub silenced_until: Option<DateTime<Utc>>,
}

impl Alert {
    pub fn new(
        rule_id: String,
        level: AlertLevel,
        title: String,
        message: String,
        service: String,
        instance: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_id,
            level,
            status: AlertStatus::Active,
            title,
            message,
            service,
            instance,
            labels: HashMap::new(),
            annotations: HashMap::new(),
            started_at: Utc::now(),
            updated_at: Utc::now(),
            resolved_at: None,
            acknowledged_at: None,
            acknowledged_by: None,
            silenced_until: None,
        }
    }
    
    pub fn with_label(mut self, key: String, value: String) -> Self {
        self.labels.insert(key, value);
        self
    }
    
    pub fn with_annotation(mut self, key: String, value: String) -> Self {
        self.annotations.insert(key, value);
        self
    }
    
    pub fn acknowledge(&mut self, acknowledged_by: String) {
        self.status = AlertStatus::Acknowledged;
        self.acknowledged_at = Some(Utc::now());
        self.acknowledged_by = Some(acknowledged_by);
        self.updated_at = Utc::now();
    }
    
    pub fn resolve(&mut self) {
        self.status = AlertStatus::Resolved;
        self.resolved_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn silence(&mut self, until: DateTime<Utc>) {
        self.status = AlertStatus::Silenced;
        self.silenced_until = Some(until);
        self.updated_at = Utc::now();
    }
}

/// 告警规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: AlertLevel,
    pub metric: String,
    pub condition: AlertCondition,
    pub threshold: f64,
    pub duration: u64, // 持续时间（秒）
    pub evaluation_interval: u64, // 评估间隔（秒）
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 告警条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl AlertCondition {
    pub fn evaluate(&self, value: f64, threshold: f64) -> bool {
        match self {
            AlertCondition::GreaterThan => value > threshold,
            AlertCondition::LessThan => value < threshold,
            AlertCondition::Equal => (value - threshold).abs() < f64::EPSILON,
            AlertCondition::NotEqual => (value - threshold).abs() >= f64::EPSILON,
            AlertCondition::GreaterThanOrEqual => value >= threshold,
            AlertCondition::LessThanOrEqual => value <= threshold,
        }
    }
}

/// 告警通道接口
#[async_trait]
pub trait AlertChannel: Send + Sync {
    async fn send_alert(&self, alert: &Alert) -> Result<()>;
    async fn send_resolution(&self, alert: &Alert) -> Result<()>;
    fn channel_type(&self) -> &'static str;
}

/// 邮件告警通道
pub struct EmailAlertChannel {
    smtp_server: String,
    username: String,
    password: String,
    recipients: Vec<String>,
}

#[async_trait]
impl AlertChannel for EmailAlertChannel {
    async fn send_alert(&self, alert: &Alert) -> Result<()> {
        let subject = format!("[{}] {}", alert.level, alert.title);
        let body = self.format_alert_email(alert);
        
        // 这里实现邮件发送逻辑
        log::info!("Sending email alert: {}", subject);
        
        Ok(())
    }
    
    async fn send_resolution(&self, alert: &Alert) -> Result<()> {
        let subject = format!("[RESOLVED] {}", alert.title);
        let body = self.format_resolution_email(alert);
        
        log::info!("Sending email resolution: {}", subject);
        
        Ok(())
    }
    
    fn channel_type(&self) -> &'static str {
        "email"
    }
}

impl EmailAlertChannel {
    pub fn new(
        smtp_server: String,
        username: String,
        password: String,
        recipients: Vec<String>,
    ) -> Self {
        Self {
            smtp_server,
            username,
            password,
            recipients,
        }
    }
    
    fn format_alert_email(&self, alert: &Alert) -> String {
        format!(
            "Alert Details:\n\
             - Service: {}\n\
             - Instance: {}\n\
             - Level: {}\n\
             - Message: {}\n\
             - Started: {}\n\
             - Labels: {:?}\n\
             - Annotations: {:?}",
            alert.service,
            alert.instance,
            alert.level,
            alert.message,
            alert.started_at,
            alert.labels,
            alert.annotations
        )
    }
    
    fn format_resolution_email(&self, alert: &Alert) -> String {
        format!(
            "Alert Resolved:\n\
             - Service: {}\n\
             - Instance: {}\n\
             - Level: {}\n\
             - Message: {}\n\
             - Started: {}\n\
             - Resolved: {}",
            alert.service,
            alert.instance,
            alert.level,
            alert.message,
            alert.started_at,
            alert.resolved_at.unwrap_or(Utc::now())
        )
    }
}

/// Slack告警通道
pub struct SlackAlertChannel {
    webhook_url: String,
    channel: String,
    username: String,
}

#[async_trait]
impl AlertChannel for SlackAlertChannel {
    async fn send_alert(&self, alert: &Alert) -> Result<()> {
        let payload = self.format_slack_message(alert, false);
        
        let client = reqwest::Client::new();
        let response = client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to send Slack alert: {}", response.status()));
        }
        
        log::info!("Sent Slack alert: {}", alert.title);
        Ok(())
    }
    
    async fn send_resolution(&self, alert: &Alert) -> Result<()> {
        let payload = self.format_slack_message(alert, true);
        
        let client = reqwest::Client::new();
        let response = client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to send Slack resolution: {}", response.status()));
        }
        
        log::info!("Sent Slack resolution: {}", alert.title);
        Ok(())
    }
    
    fn channel_type(&self) -> &'static str {
        "slack"
    }
}

impl SlackAlertChannel {
    pub fn new(webhook_url: String, channel: String, username: String) -> Self {
        Self {
            webhook_url,
            channel,
            username,
        }
    }
    
    fn format_slack_message(&self, alert: &Alert, is_resolution: bool) -> serde_json::Value {
        let color = if is_resolution {
            "good"
        } else {
            match alert.level {
                AlertLevel::Info => "good",
                AlertLevel::Warning => "warning",
                AlertLevel::Error => "danger",
                AlertLevel::Critical => "danger",
            }
        };
        
        let title = if is_resolution {
            format!("🟢 RESOLVED: {}", alert.title)
        } else {
            format!("🚨 ALERT: {}", alert.title)
        };
        
        serde_json::json!({
            "channel": self.channel,
            "username": self.username,
            "attachments": [{
                "color": color,
                "title": title,
                "text": alert.message,
                "fields": [
                    {
                        "title": "Service",
                        "value": alert.service,
                        "short": true
                    },
                    {
                        "title": "Instance",
                        "value": alert.instance,
                        "short": true
                    },
                    {
                        "title": "Level",
                        "value": alert.level.to_string(),
                        "short": true
                    },
                    {
                        "title": "Status",
                        "value": alert.status.to_string(),
                        "short": true
                    }
                ],
                "ts": alert.started_at.timestamp()
            }]
        })
    }
}

/// Webhook告警通道
pub struct WebhookAlertChannel {
    url: String,
    headers: HashMap<String, String>,
}

#[async_trait]
impl AlertChannel for WebhookAlertChannel {
    async fn send_alert(&self, alert: &Alert) -> Result<()> {
        let payload = serde_json::json!({
            "type": "alert",
            "alert": alert
        });
        
        let client = reqwest::Client::new();
        let mut request = client.post(&self.url).json(&payload);
        
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }
        
        let response = request.send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to send webhook alert: {}", response.status()));
        }
        
        log::info!("Sent webhook alert: {}", alert.title);
        Ok(())
    }
    
    async fn send_resolution(&self, alert: &Alert) -> Result<()> {
        let payload = serde_json::json!({
            "type": "resolution",
            "alert": alert
        });
        
        let client = reqwest::Client::new();
        let mut request = client.post(&self.url).json(&payload);
        
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }
        
        let response = request.send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to send webhook resolution: {}", response.status()));
        }
        
        log::info!("Sent webhook resolution: {}", alert.title);
        Ok(())
    }
    
    fn channel_type(&self) -> &'static str {
        "webhook"
    }
}

impl WebhookAlertChannel {
    pub fn new(url: String, headers: HashMap<String, String>) -> Self {
        Self { url, headers }
    }
}

/// 告警管理器
pub struct AlertManager {
    rules: HashMap<String, AlertRule>,
    channels: Vec<Box<dyn AlertChannel>>,
    active_alerts: HashMap<Uuid, Alert>,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            channels: Vec::new(),
            active_alerts: HashMap::new(),
        }
    }
    
    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.insert(rule.id.clone(), rule);
    }
    
    pub fn remove_rule(&mut self, rule_id: &str) {
        self.rules.remove(rule_id);
    }
    
    pub fn add_channel(&mut self, channel: Box<dyn AlertChannel>) {
        self.channels.push(channel);
    }
    
    pub async fn evaluate_rules(&mut self, metrics: &HashMap<String, f64>) -> Result<()> {
        let rules: Vec<AlertRule> = self.rules.values().cloned().collect();
        
        for rule in rules {
            if !rule.enabled {
                continue;
            }
            
            if let Some(&metric_value) = metrics.get(&rule.metric) {
                let should_alert = rule.condition.evaluate(metric_value, rule.threshold);
                
                if should_alert {
                    self.trigger_alert(&rule, metric_value).await?;
                } else {
                    self.resolve_alert(&rule).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn trigger_alert(&mut self, rule: &AlertRule, metric_value: f64) -> Result<()> {
        // 检查是否已有活跃的告警
        let existing_alert = self.active_alerts.values()
            .find(|alert| alert.rule_id == rule.id && alert.status == AlertStatus::Active);
        
        if existing_alert.is_some() {
            return Ok(()); // 已有活跃告警，不重复发送
        }
        
        let mut alert = Alert::new(
            rule.id.clone(),
            rule.level,
            rule.name.clone(),
            rule.description.clone(),
            "soonshop".to_string(),
            "localhost".to_string(),
        );
        
        // 添加标签和注释
        for (key, value) in &rule.labels {
            alert.labels.insert(key.clone(), value.clone());
        }
        
        for (key, value) in &rule.annotations {
            alert.annotations.insert(key.clone(), value.clone());
        }
        
        alert.annotations.insert("metric_value".to_string(), metric_value.to_string());
        alert.annotations.insert("threshold".to_string(), rule.threshold.to_string());
        
        // 发送告警到所有通道
        for channel in &self.channels {
            if let Err(e) = channel.send_alert(&alert).await {
                log::error!("Failed to send alert via {}: {}", channel.channel_type(), e);
            }
        }
        
        self.active_alerts.insert(alert.id, alert);
        
        Ok(())
    }
    
    async fn resolve_alert(&mut self, rule: &AlertRule) -> Result<()> {
        let alert_ids: Vec<Uuid> = self.active_alerts.values()
            .filter(|alert| alert.rule_id == rule.id && alert.status == AlertStatus::Active)
            .map(|alert| alert.id)
            .collect();
        
        for alert_id in alert_ids {
            if let Some(mut alert) = self.active_alerts.remove(&alert_id) {
                alert.resolve();
                
                // 发送解决通知到所有通道
                for channel in &self.channels {
                    if let Err(e) = channel.send_resolution(&alert).await {
                        log::error!("Failed to send resolution via {}: {}", channel.channel_type(), e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn acknowledge_alert(&mut self, alert_id: Uuid, acknowledged_by: String) -> Result<()> {
        if let Some(alert) = self.active_alerts.get_mut(&alert_id) {
            alert.acknowledge(acknowledged_by);
        }
        
        Ok(())
    }
    
    pub async fn silence_alert(&mut self, alert_id: Uuid, until: DateTime<Utc>) -> Result<()> {
        if let Some(alert) = self.active_alerts.get_mut(&alert_id) {
            alert.silence(until);
        }
        
        Ok(())
    }
    
    pub fn get_active_alerts(&self) -> Vec<&Alert> {
        self.active_alerts.values()
            .filter(|alert| alert.status == AlertStatus::Active)
            .collect()
    }
    
    pub fn get_all_alerts(&self) -> Vec<&Alert> {
        self.active_alerts.values().collect()
    }
} 