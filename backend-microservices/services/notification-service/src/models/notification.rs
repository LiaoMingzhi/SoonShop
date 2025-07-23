use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub priority: NotificationPriority,
    pub title: String,
    pub content: String,
    pub template_id: Option<String>,
    pub template_data: Option<serde_json::Value>,
    pub status: NotificationStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub max_retries: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    OrderConfirmation,
    PaymentSuccess,
    PaymentFailed,
    ShippingUpdate,
    DeliveryNotification,
    VoucherAvailable,
    VoucherExpiring,
    AccountVerification,
    PasswordReset,
    SecurityAlert,
    SystemMaintenance,
    Marketing,
    Custom,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::OrderConfirmation => write!(f, "order_confirmation"),
            NotificationType::PaymentSuccess => write!(f, "payment_success"),
            NotificationType::PaymentFailed => write!(f, "payment_failed"),
            NotificationType::ShippingUpdate => write!(f, "shipping_update"),
            NotificationType::DeliveryNotification => write!(f, "delivery_notification"),
            NotificationType::VoucherAvailable => write!(f, "voucher_available"),
            NotificationType::VoucherExpiring => write!(f, "voucher_expiring"),
            NotificationType::AccountVerification => write!(f, "account_verification"),
            NotificationType::PasswordReset => write!(f, "password_reset"),
            NotificationType::SecurityAlert => write!(f, "security_alert"),
            NotificationType::SystemMaintenance => write!(f, "system_maintenance"),
            NotificationType::Marketing => write!(f, "marketing"),
            NotificationType::Custom => write!(f, "custom"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationChannel {
    Email,
    SMS,
    Push,
    InApp,
    Webhook,
}

impl std::fmt::Display for NotificationChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationChannel::Email => write!(f, "email"),
            NotificationChannel::SMS => write!(f, "sms"),
            NotificationChannel::Push => write!(f, "push"),
            NotificationChannel::InApp => write!(f, "in_app"),
            NotificationChannel::Webhook => write!(f, "webhook"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl std::fmt::Display for NotificationPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationPriority::Low => write!(f, "low"),
            NotificationPriority::Normal => write!(f, "normal"),
            NotificationPriority::High => write!(f, "high"),
            NotificationPriority::Critical => write!(f, "critical"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationStatus {
    Pending,
    Scheduled,
    Sent,
    Delivered,
    Read,
    Failed,
    Cancelled,
}

impl std::fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationStatus::Pending => write!(f, "pending"),
            NotificationStatus::Scheduled => write!(f, "scheduled"),
            NotificationStatus::Sent => write!(f, "sent"),
            NotificationStatus::Delivered => write!(f, "delivered"),
            NotificationStatus::Read => write!(f, "read"),
            NotificationStatus::Failed => write!(f, "failed"),
            NotificationStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub subject_template: String,
    pub content_template: String,
    pub variables: Vec<TemplateVariable>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub data_type: String,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRecipient {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub push_token: Option<String>,
    pub platform: Option<String>, // ios, android, web
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 请求和响应模型
#[derive(Debug, Deserialize, Validate)]
pub struct SendNotificationRequest {
    pub recipient_id: Uuid,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub priority: NotificationPriority,
    pub title: String,
    pub content: String,
    pub template_id: Option<String>,
    pub template_data: Option<serde_json::Value>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendBulkNotificationRequest {
    pub recipient_ids: Vec<Uuid>,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub priority: NotificationPriority,
    pub title: String,
    pub content: String,
    pub template_id: Option<String>,
    pub template_data: Option<serde_json::Value>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNotificationPreferenceRequest {
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRecipientRequest {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub push_token: Option<String>,
    pub platform: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub priority: NotificationPriority,
    pub title: String,
    pub content: String,
    pub status: NotificationStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub max_retries: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct NotificationStatsResponse {
    pub total_sent: i64,
    pub total_delivered: i64,
    pub total_read: i64,
    pub total_failed: i64,
    pub delivery_rate: f64,
    pub read_rate: f64,
    pub channel_stats: Vec<ChannelStats>,
    pub type_stats: Vec<TypeStats>,
}

#[derive(Debug, Serialize)]
pub struct ChannelStats {
    pub channel: NotificationChannel,
    pub sent: i64,
    pub delivered: i64,
    pub failed: i64,
    pub delivery_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct TypeStats {
    pub notification_type: NotificationType,
    pub sent: i64,
    pub delivered: i64,
    pub read: i64,
    pub failed: i64,
}

#[derive(Debug, Serialize)]
pub struct NotificationPreferenceResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct NotificationRecipientResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub push_token: Option<String>,
    pub platform: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 事件模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    pub id: Uuid,
    pub notification_id: Uuid,
    pub event_type: NotificationEventType,
    pub timestamp: DateTime<Utc>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationEventType {
    Created,
    Sent,
    Delivered,
    Read,
    Failed,
    Cancelled,
    Bounced,
    Clicked,
    Opened,
}

impl From<Notification> for NotificationResponse {
    fn from(notification: Notification) -> Self {
        NotificationResponse {
            id: notification.id,
            recipient_id: notification.recipient_id,
            notification_type: notification.notification_type,
            channel: notification.channel,
            priority: notification.priority,
            title: notification.title,
            content: notification.content,
            status: notification.status,
            scheduled_at: notification.scheduled_at,
            sent_at: notification.sent_at,
            delivered_at: notification.delivered_at,
            read_at: notification.read_at,
            error_message: notification.error_message,
            retry_count: notification.retry_count,
            max_retries: notification.max_retries,
            created_at: notification.created_at,
            updated_at: notification.updated_at,
        }
    }
}

impl From<NotificationPreference> for NotificationPreferenceResponse {
    fn from(preference: NotificationPreference) -> Self {
        NotificationPreferenceResponse {
            id: preference.id,
            user_id: preference.user_id,
            notification_type: preference.notification_type,
            channel: preference.channel,
            enabled: preference.enabled,
            created_at: preference.created_at,
            updated_at: preference.updated_at,
        }
    }
}

impl From<NotificationRecipient> for NotificationRecipientResponse {
    fn from(recipient: NotificationRecipient) -> Self {
        NotificationRecipientResponse {
            id: recipient.id,
            user_id: recipient.user_id,
            email: recipient.email,
            phone: recipient.phone,
            push_token: recipient.push_token,
            platform: recipient.platform,
            timezone: recipient.timezone,
            language: recipient.language,
            is_active: recipient.is_active,
            created_at: recipient.created_at,
            updated_at: recipient.updated_at,
        }
    }
} 