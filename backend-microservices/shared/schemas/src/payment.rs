// 支付相关模式定义 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use std::collections::HashMap;

/// 支付实体模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSchema {
    pub id: Uuid,
    pub payment_number: String,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub payment_provider: PaymentProvider,
    pub status: PaymentStatus,
    pub provider_transaction_id: Option<String>,
    pub provider_payment_id: Option<String>,
    pub wallet_address: Option<String>,
    pub transaction_hash: Option<String>,
    pub block_number: Option<u64>,
    pub confirmation_count: u32,
    pub payment_intent_id: Option<String>,
    pub failure_reason: Option<String>,
    pub processing_fee: f64,
    pub net_amount: Option<f64>,
    pub metadata: HashMap<String, String>,
    pub authorized_at: Option<DateTime<Utc>>,
    pub captured_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 支付状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaymentStatus {
    Pending,
    Authorized,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartiallyRefunded,
    Expired,
}

/// 支付方式枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    BankTransfer,
    PayPal,
    Stripe,
    Cryptocurrency,
    DigitalWallet,
    GiftCard,
    StoreCredit,
}

/// 支付提供商枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaymentProvider {
    Stripe,
    PayPal,
    Solana,
    BankTransfer,
    Square,
    Alipay,
    WeChat,
    ApplePay,
    GooglePay,
}

/// 支付事件日志模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentEventSchema {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub event_type: PaymentEventType,
    pub event_data: serde_json::Value,
    pub provider_event_id: Option<String>,
    pub is_processed: bool,
    pub processed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 支付事件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentEventType {
    PaymentCreated,
    PaymentAuthorized,
    PaymentCaptured,
    PaymentFailed,
    PaymentCancelled,
    PaymentRefunded,
    PaymentExpired,
    WebhookReceived,
    TransactionConfirmed,
}

/// 支付方式配置模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodConfigSchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub method_type: PaymentMethod,
    pub provider: PaymentProvider,
    pub provider_method_id: Option<String>,
    pub wallet_address: Option<String>,
    pub card_last_four: Option<String>,
    pub card_brand: Option<String>,
    pub card_expiry_month: Option<u8>,
    pub card_expiry_year: Option<u16>,
    pub is_default: bool,
    pub is_active: bool,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 钱包余额模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalanceSchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub currency: String,
    pub available_balance: f64,
    pub pending_balance: f64,
    pub frozen_balance: f64,
    pub total_balance: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 钱包交易模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionSchema {
    pub id: Uuid,
    pub wallet_balance_id: Uuid,
    pub payment_id: Option<Uuid>,
    pub transaction_type: WalletTransactionType,
    pub amount: f64,
    pub previous_balance: f64,
    pub new_balance: f64,
    pub description: Option<String>,
    pub reference_id: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// 钱包交易类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletTransactionType {
    Deposit,
    Withdrawal,
    Payment,
    Refund,
    Transfer,
    Fee,
    Reward,
    Adjustment,
}

/// 创建支付请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentSchema {
    pub order_id: Uuid,
    
    #[validate(range(min = 0.01))]
    pub amount: f64,
    
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    
    pub payment_method: PaymentMethod,
    pub payment_provider: PaymentProvider,
    pub payment_method_id: Option<String>,
    pub return_url: Option<String>,
    pub cancel_url: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// 确认支付请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct ConfirmPaymentSchema {
    pub payment_id: Uuid,
    pub payment_method_id: Option<String>,
    pub confirmation_token: Option<String>,
    pub security_code: Option<String>,
}

/// 创建退款请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct CreateRefundSchema {
    pub payment_id: Uuid,
    
    #[validate(range(min = 0.01))]
    pub amount: Option<f64>, // None表示全额退款
    
    #[validate(length(max = 500))]
    pub reason: Option<String>,
    
    pub metadata: Option<HashMap<String, String>>,
}

/// 添加支付方式请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct AddPaymentMethodSchema {
    pub method_type: PaymentMethod,
    pub provider: PaymentProvider,
    pub provider_method_id: Option<String>,
    pub wallet_address: Option<String>,
    pub card_token: Option<String>,
    pub is_default: Option<bool>,
    pub metadata: Option<HashMap<String, String>>,
}

/// 钱包充值请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct WalletDepositSchema {
    #[validate(range(min = 0.01))]
    pub amount: f64,
    
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    
    pub payment_method: PaymentMethod,
    pub payment_provider: PaymentProvider,
    pub payment_method_id: Option<String>,
}

/// 钱包提现请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct WalletWithdrawSchema {
    #[validate(range(min = 0.01))]
    pub amount: f64,
    
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    
    pub withdrawal_method: WithdrawalMethod,
    pub destination_account: String, // 银行账户号或钱包地址
    pub description: Option<String>,
}

/// 提现方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WithdrawalMethod {
    BankTransfer,
    CryptocurrencyTransfer,
    PayPal,
    DigitalWallet,
}

/// 钱包转账请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct WalletTransferSchema {
    pub recipient_user_id: Uuid,
    
    #[validate(range(min = 0.01))]
    pub amount: f64,
    
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

/// 支付搜索查询参数
#[derive(Debug, Deserialize, Validate)]
pub struct PaymentSearchQuerySchema {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>, // 支付号或交易ID搜索
    pub user_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub status: Option<PaymentStatus>,
    pub payment_method: Option<PaymentMethod>,
    pub payment_provider: Option<PaymentProvider>,
    
    #[validate(range(min = 0.0))]
    pub min_amount: Option<f64>,
    
    #[validate(range(min = 0.0))]
    pub max_amount: Option<f64>,
    
    pub currency: Option<String>,
    pub sort_by: Option<PaymentSortField>,
    pub sort_order: Option<SortOrder>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub captured_after: Option<DateTime<Utc>>,
    pub captured_before: Option<DateTime<Utc>>,
}

/// 支付排序字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentSortField {
    PaymentNumber,
    Amount,
    Status,
    PaymentMethod,
    CreatedAt,
    CapturedAt,
    UpdatedAt,
}

/// 排序顺序
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// 支付统计模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatsSchema {
    pub total_payments: u64,
    pub total_amount: f64,
    pub successful_payments: u64,
    pub failed_payments: u64,
    pub refunded_payments: u64,
    pub average_payment_amount: f64,
    pub payments_by_method: HashMap<PaymentMethod, PaymentMethodStats>,
    pub payments_by_provider: HashMap<PaymentProvider, PaymentProviderStats>,
    pub payments_by_status: HashMap<PaymentStatus, u64>,
    pub revenue_by_period: Vec<RevenuePeriod>,
    pub updated_at: DateTime<Utc>,
}

/// 支付方式统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodStats {
    pub total_count: u64,
    pub total_amount: f64,
    pub success_rate: f64,
    pub average_amount: f64,
}

/// 支付提供商统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProviderStats {
    pub total_count: u64,
    pub total_amount: f64,
    pub success_rate: f64,
    pub average_processing_time: f64, // in seconds
    pub total_fees: f64,
}

/// 时期收入统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenuePeriod {
    pub period: String, // "2024-01", "2024-01-15" etc.
    pub revenue: f64,
    pub payment_count: u64,
    pub refund_amount: f64,
    pub net_revenue: f64,
}

/// 支付响应模式（包含详细信息）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponseSchema {
    pub payment: PaymentSchema,
    pub events: Vec<PaymentEventSchema>,
    pub refunds: Vec<RefundSchema>,
    pub wallet_transactions: Vec<WalletTransactionSchema>,
}

/// 退款模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundSchema {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub refund_number: String,
    pub amount: f64,
    pub currency: String,
    pub reason: Option<String>,
    pub status: RefundStatus,
    pub provider_refund_id: Option<String>,
    pub processed_by: Option<Uuid>,
    pub processed_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 退款状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefundStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// 支付意图模式（用于需要确认的支付）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntentSchema {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub client_secret: Option<String>,
    pub confirmation_method: ConfirmationMethod,
    pub status: PaymentIntentStatus,
    pub next_action: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 确认方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfirmationMethod {
    Automatic,
    Manual,
}

/// 支付意图状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentIntentStatus {
    RequiresPaymentMethod,
    RequiresConfirmation,
    RequiresAction,
    Processing,
    Succeeded,
    Cancelled,
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