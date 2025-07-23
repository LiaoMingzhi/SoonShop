// 通知模型定义
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// 基本的通知结构，暂时定义在这里
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub message: String,
    pub channel: NotificationChannel,
    pub status: NotificationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
    InApp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
    Read,
}

// 添加缺失的类型以匹配处理器的导入
pub mod notification {
    use super::*;
    
    pub use super::{Notification, NotificationChannel, NotificationStatus};
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NotificationTemplate {
        pub id: Uuid,
        pub name: String,
        pub notification_type: NotificationType,
        pub title_template: String,
        pub body_template: String,
        pub template_variables: Option<serde_json::Value>,
        pub is_active: bool,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserNotificationPreference {
        pub id: Uuid,
        pub user_id: Uuid,
        pub notification_type: NotificationType,
        pub email_enabled: bool,
        pub push_enabled: bool,
        pub sms_enabled: bool,
        pub in_app_enabled: bool,
        pub frequency: NotificationFrequency,
        pub quiet_hours_start: Option<String>,
        pub quiet_hours_end: Option<String>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NotificationSubscription {
        pub id: Uuid,
        pub user_id: Uuid,
        pub endpoint: String,
        pub p256dh_key: String,
        pub auth_key: String,
        pub user_agent: Option<String>,
        pub created_at: DateTime<Utc>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
    pub enum NotificationType {
        System,
        Order,
        Payment,
        Security,
        Marketing,
        Update,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DeliveryStatus {
        Pending,
        Sent,
        Delivered,
        Failed,
        Bounced,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
    pub enum DeliveryChannel {
        Email,
        Sms,
        Push,
        InApp,
        Webhook,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum NotificationFrequency {
        Immediate,
        Hourly,
        Daily,
        Weekly,
        Never,
    }
}

// 重新导出主要类型以保持向后兼容
pub use notification::*; 