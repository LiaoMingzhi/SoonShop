// 通知相关模式定义 - 完整实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// 通知模板实体
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

// 通知实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub template_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub read_at: Option<DateTime<Utc>>,
    pub delivery_status: DeliveryStatus,
    pub delivery_channel: DeliveryChannel,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 用户通知偏好
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
    pub quiet_hours_start: Option<String>, // HH:MM format
    pub quiet_hours_end: Option<String>,   // HH:MM format
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 通知订阅
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSubscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub endpoint: String,
    pub p256dh_key: String,
    pub auth_key: String,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 通知统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationStatistic {
    pub id: Uuid,
    pub date: chrono::NaiveDate,
    pub notification_type: NotificationType,
    pub delivery_channel: DeliveryChannel,
    pub total_sent: i64,
    pub total_delivered: i64,
    pub total_failed: i64,
    pub total_opened: i64,
    pub total_clicked: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 枚举定义
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationType {
    #[serde(rename = "order_status")]
    OrderStatus,
    #[serde(rename = "payment_status")]
    PaymentStatus,
    #[serde(rename = "product_alert")]
    ProductAlert,
    #[serde(rename = "inventory_alert")]
    InventoryAlert,
    #[serde(rename = "system_announcement")]
    SystemAnnouncement,
    #[serde(rename = "marketing")]
    Marketing,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "welcome")]
    Welcome,
    #[serde(rename = "reminder")]
    Reminder,
    #[serde(rename = "community")]
    Community,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeliveryStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "scheduled")]
    Scheduled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeliveryChannel {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "sms")]
    SMS,
    #[serde(rename = "in_app")]
    InApp,
    #[serde(rename = "webhook")]
    Webhook,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationFrequency {
    #[serde(rename = "immediate")]
    Immediate,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "never")]
    Never,
}

// 请求结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    pub template_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub delivery_channel: DeliveryChannel,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub notification_type: NotificationType,
    pub title_template: String,
    pub body_template: String,
    pub template_variables: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub title_template: Option<String>,
    pub body_template: Option<String>,
    pub template_variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreferenceRequest {
    pub notification_type: NotificationType,
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub in_app_enabled: Option<bool>,
    pub frequency: Option<NotificationFrequency>,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendBulkNotificationRequest {
    pub user_ids: Vec<Uuid>,
    pub template_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub delivery_channel: DeliveryChannel,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeRequest {
    pub endpoint: String,
    pub p256dh_key: String,
    pub auth_key: String,
    pub user_agent: Option<String>,
}

// 响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub read_at: Option<DateTime<Utc>>,
    pub delivery_status: DeliveryStatus,
    pub delivery_channel: DeliveryChannel,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub unread_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateResponse {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferenceResponse {
    pub notification_type: NotificationType,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub sms_enabled: bool,
    pub in_app_enabled: bool,
    pub frequency: NotificationFrequency,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsResponse {
    pub total_notifications: i64,
    pub delivery_stats: HashMap<DeliveryChannel, DeliveryStats>,
    pub type_stats: HashMap<NotificationType, TypeStats>,
    pub recent_activity: Vec<ActivityStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryStats {
    pub total_sent: i64,
    pub total_delivered: i64,
    pub total_failed: i64,
    pub delivery_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeStats {
    pub total_sent: i64,
    pub total_opened: i64,
    pub total_clicked: i64,
    pub open_rate: f64,
    pub click_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityStat {
    pub date: chrono::NaiveDate,
    pub notifications_sent: i64,
    pub notifications_delivered: i64,
}

// 查询参数
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub notification_type: Option<NotificationType>,
    pub delivery_status: Option<DeliveryStatus>,
    pub delivery_channel: Option<DeliveryChannel>,
    pub read_status: Option<ReadStatus>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReadStatus {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "unread")]
    Unread,
    #[serde(rename = "all")]
    All,
}

// 批量操作
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkMarkReadRequest {
    pub notification_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkDeleteRequest {
    pub notification_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkResponse {
    pub success_count: i32,
    pub failed_count: i32,
    pub errors: Vec<String>,
} 