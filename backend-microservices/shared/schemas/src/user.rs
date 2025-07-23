// 用户相关模式定义
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use std::collections::HashMap;

/// 用户实体模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSchema {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub role: UserRole,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub two_factor_enabled: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub preferences: UserPreferences,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 用户状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
    Banned,
}

/// 用户角色枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    Moderator,
    Customer,
    Merchant,
    Guest,
}

/// 用户偏好设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: String,
    pub timezone: String,
    pub currency: String,
    pub notification_settings: NotificationSettings,
    pub privacy_settings: PrivacySettings,
    pub theme: String,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: "zh-CN".to_string(),
            timezone: "Asia/Shanghai".to_string(),
            currency: "USD".to_string(),
            notification_settings: NotificationSettings::default(),
            privacy_settings: PrivacySettings::default(),
            theme: "light".to_string(),
        }
    }
}

/// 通知设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email_notifications: bool,
    pub sms_notifications: bool,
    pub push_notifications: bool,
    pub marketing_emails: bool,
    pub order_updates: bool,
    pub payment_alerts: bool,
    pub security_alerts: bool,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            email_notifications: true,
            sms_notifications: false,
            push_notifications: true,
            marketing_emails: false,
            order_updates: true,
            payment_alerts: true,
            security_alerts: true,
            quiet_hours_start: None,
            quiet_hours_end: None,
        }
    }
}

/// 隐私设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub profile_visibility: ProfileVisibility,
    pub show_email: bool,
    pub show_phone: bool,
    pub show_activity: bool,
    pub allow_friend_requests: bool,
    pub data_collection_consent: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            profile_visibility: ProfileVisibility::Public,
            show_email: false,
            show_phone: false,
            show_activity: true,
            allow_friend_requests: true,
            data_collection_consent: false,
        }
    }
}

/// 用户资料可见性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProfileVisibility {
    Public,
    Friends,
    Private,
}

/// 创建用户请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserSchema {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    
    #[validate(length(min = 1, max = 50))]
    pub first_name: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
    pub last_name: Option<String>,
    
    #[validate(length(min = 10, max = 20))]
    pub phone: Option<String>,
    
    pub role: Option<UserRole>,
    pub preferences: Option<UserPreferences>,
}

/// 更新用户请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserSchema {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
    pub first_name: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
    pub last_name: Option<String>,
    
    #[validate(length(min = 10, max = 20))]
    pub phone: Option<String>,
    
    pub avatar_url: Option<String>,
    pub status: Option<UserStatus>,
    pub role: Option<UserRole>,
    pub preferences: Option<UserPreferences>,
}

/// 用户登录请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct LoginSchema {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 1))]
    pub password: String,
    
    pub remember_me: Option<bool>,
    pub device_info: Option<DeviceInfo>,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_type: DeviceType,
    pub os: String,
    pub browser: Option<String>,
    pub ip_address: String,
    pub user_agent: String,
}

/// 设备类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    Unknown,
}

/// 密码重置请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct PasswordResetRequestSchema {
    #[validate(email)]
    pub email: String,
}

/// 密码重置确认模式
#[derive(Debug, Deserialize, Validate)]
pub struct PasswordResetConfirmSchema {
    pub token: String,
    
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
    
    #[validate(must_match(other = "new_password"))]
    pub confirm_password: String,
}

/// 用户地址模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAddressSchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub address_type: AddressType,
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 地址类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AddressType {
    Billing,
    Shipping,
    Both,
}

/// 创建地址请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct CreateAddressSchema {
    pub address_type: AddressType,
    
    #[validate(length(min = 1, max = 50))]
    pub first_name: String,
    
    #[validate(length(min = 1, max = 50))]
    pub last_name: String,
    
    #[validate(length(max = 100))]
    pub company: Option<String>,
    
    #[validate(length(min = 1, max = 255))]
    pub address_line_1: String,
    
    #[validate(length(max = 255))]
    pub address_line_2: Option<String>,
    
    #[validate(length(min = 1, max = 100))]
    pub city: String,
    
    #[validate(length(min = 1, max = 100))]
    pub state: String,
    
    #[validate(length(min = 1, max = 20))]
    pub postal_code: String,
    
    #[validate(length(min = 2, max = 2))]
    pub country: String,
    
    #[validate(length(min = 10, max = 20))]
    pub phone: Option<String>,
    
    pub is_default: Option<bool>,
}

/// 用户会话模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionSchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub refresh_token: Option<String>,
    pub device_info: DeviceInfo,
    pub is_active: bool,
    pub last_activity_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// 用户权限模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissionSchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub permission: Permission,
    pub resource: String,
    pub granted_by: Uuid,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// 权限枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Delete,
    Admin,
    Execute,
}

/// 用户活动模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivitySchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_type: ActivityType,
    pub description: String,
    pub ip_address: String,
    pub user_agent: String,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// 活动类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    Login,
    Logout,
    PasswordChange,
    ProfileUpdate,
    OrderPlaced,
    PaymentMade,
    AddressAdded,
    SecurityAlert,
    Other,
}

/// 用户响应模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponseSchema {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub role: UserRole,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub preferences: UserPreferences,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 用户列表查询模式
#[derive(Debug, Deserialize, Validate)]
pub struct UserListQuerySchema {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub status: Option<UserStatus>,
    pub role: Option<UserRole>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
}

/// 排序顺序
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// 分页响应模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
} 