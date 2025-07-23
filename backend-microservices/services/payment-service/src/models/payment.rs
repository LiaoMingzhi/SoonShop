use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartiallyRefunded,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::Pending => write!(f, "pending"),
            PaymentStatus::Processing => write!(f, "processing"),
            PaymentStatus::Completed => write!(f, "completed"),
            PaymentStatus::Failed => write!(f, "failed"),
            PaymentStatus::Cancelled => write!(f, "cancelled"),
            PaymentStatus::Refunded => write!(f, "refunded"),
            PaymentStatus::PartiallyRefunded => write!(f, "partially_refunded"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentMethod {
    Solana,
    CreditCard,
    DebitCard,
    PayPal,
    BankTransfer,
    Voucher,
    Cryptocurrency,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethod::Solana => write!(f, "solana"),
            PaymentMethod::CreditCard => write!(f, "credit_card"),
            PaymentMethod::DebitCard => write!(f, "debit_card"),
            PaymentMethod::PayPal => write!(f, "paypal"),
            PaymentMethod::BankTransfer => write!(f, "bank_transfer"),
            PaymentMethod::Voucher => write!(f, "voucher"),
            PaymentMethod::Cryptocurrency => write!(f, "cryptocurrency"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Currency {
    USD,
    EUR,
    SOL,
    USDC,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::SOL => write!(f, "SOL"),
            Currency::USDC => write!(f, "USDC"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub amount: Decimal,
    pub currency: Currency,
    pub fee_amount: Decimal,
    pub net_amount: Decimal,
    pub provider_payment_id: Option<String>,
    pub provider_name: Option<String>,
    pub transaction_hash: Option<String>,
    pub failure_reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRefund {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub refund_amount: Decimal,
    pub reason: String,
    pub status: RefundStatus,
    pub provider_refund_id: Option<String>,
    pub transaction_hash: Option<String>,
    pub processed_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefundStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCard {
    pub id: Uuid,
    pub user_id: Uuid,
    pub encrypted_card_number: String,
    pub card_brand: String,
    pub last_four: String,
    pub expiry_month: u8,
    pub expiry_year: u16,
    pub cardholder_name: String,
    pub billing_address: BillingAddress,
    pub is_default: bool,
    pub is_active: bool,
    pub provider_card_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentRequest {
    pub order_id: Uuid,
    pub payment_method: PaymentMethod,
    pub amount: Decimal,
    pub currency: Currency,
    pub payment_details: PaymentDetails,
    pub return_url: Option<String>,
    pub cancel_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PaymentDetails {
    Solana {
        from_address: String,
        signature: Option<String>,
    },
    CreditCard {
        card_id: Option<Uuid>,
        card_token: Option<String>,
        save_card: bool,
    },
    PayPal {
        return_url: String,
        cancel_url: String,
    },
    BankTransfer {
        bank_account_id: Option<Uuid>,
    },
    Voucher {
        voucher_code: String,
    },
}

#[derive(Debug, Deserialize, Validate)]
pub struct ProcessPaymentRequest {
    pub payment_method_details: serde_json::Value,
    pub confirmation_token: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefundPaymentRequest {
    pub amount: Option<Decimal>,
    pub reason: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: PaymentStatus,
    pub amount: Decimal,
    pub currency: Currency,
    pub payment_method: PaymentMethod,
    pub fee_amount: Decimal,
    pub net_amount: Decimal,
    pub provider_payment_id: Option<String>,
    pub transaction_hash: Option<String>,
    pub payment_url: Option<String>,
    pub qr_code: Option<String>,
    pub instructions: Option<PaymentInstructions>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PaymentInstructions {
    pub title: String,
    pub description: String,
    pub steps: Vec<String>,
    pub additional_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct RefundResponse {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub refund_amount: Decimal,
    pub status: RefundStatus,
    pub reason: String,
    pub provider_refund_id: Option<String>,
    pub transaction_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub estimated_arrival: Option<DateTime<Utc>>,
}

impl From<Payment> for PaymentResponse {
    fn from(payment: Payment) -> Self {
        PaymentResponse {
            id: payment.id,
            order_id: payment.order_id,
            status: payment.status,
            amount: payment.amount,
            currency: payment.currency,
            payment_method: payment.payment_method,
            fee_amount: payment.fee_amount,
            net_amount: payment.net_amount,
            provider_payment_id: payment.provider_payment_id,
            transaction_hash: payment.transaction_hash,
            payment_url: None, // 需要根据支付方式生成
            qr_code: None,     // 需要根据支付方式生成
            instructions: None, // 需要根据支付方式生成
            created_at: payment.created_at,
            expires_at: payment.expires_at,
        }
    }
}

impl From<PaymentRefund> for RefundResponse {
    fn from(refund: PaymentRefund) -> Self {
        RefundResponse {
            id: refund.id,
            payment_id: refund.payment_id,
            refund_amount: refund.refund_amount,
            status: refund.status,
            reason: refund.reason,
            provider_refund_id: refund.provider_refund_id,
            transaction_hash: refund.transaction_hash,
            created_at: refund.created_at,
            estimated_arrival: None, // 需要根据支付方式计算
        }
    }
} 