use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    PaymentPending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::PaymentPending => write!(f, "payment_pending"),
            OrderStatus::Confirmed => write!(f, "confirmed"),
            OrderStatus::Processing => write!(f, "processing"),
            OrderStatus::Shipped => write!(f, "shipped"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
            OrderStatus::Refunded => write!(f, "refunded"),
        }
    }
}

impl From<String> for OrderStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "pending" => OrderStatus::Pending,
            "payment_pending" => OrderStatus::PaymentPending,
            "confirmed" => OrderStatus::Confirmed,
            "processing" => OrderStatus::Processing,
            "shipped" => OrderStatus::Shipped,
            "delivered" => OrderStatus::Delivered,
            "cancelled" => OrderStatus::Cancelled,
            "refunded" => OrderStatus::Refunded,
            _ => OrderStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Regular,
    Voucher,
    Subscription,
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Regular => write!(f, "regular"),
            OrderType::Voucher => write!(f, "voucher"),
            OrderType::Subscription => write!(f, "subscription"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub currency: String,
    pub items: Vec<OrderItem>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub payment_method: Option<String>,
    pub payment_id: Option<String>,
    pub voucher_id: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_sku: String,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_price: f64,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(length(min = 1))]
    pub items: Vec<CreateOrderItemRequest>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub payment_method: Option<String>,
    pub voucher_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderItemRequest {
    pub product_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: u32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderRequest {
    pub status: Option<OrderStatus>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub payment_method: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub currency: String,
    pub items: Vec<OrderItemResponse>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub payment_method: Option<String>,
    pub payment_id: Option<String>,
    pub voucher_id: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_sku: String,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_price: f64,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        OrderResponse {
            id: order.id,
            user_id: order.user_id,
            order_type: order.order_type,
            status: order.status,
            total_amount: order.total_amount,
            currency: order.currency,
            items: order.items.into_iter().map(|item| OrderItemResponse {
                id: item.id,
                product_id: item.product_id,
                product_name: item.product_name,
                product_sku: item.product_sku,
                quantity: item.quantity,
                unit_price: item.unit_price,
                total_price: item.total_price,
            }).collect(),
            shipping_address: order.shipping_address,
            billing_address: order.billing_address,
            payment_method: order.payment_method,
            payment_id: order.payment_id,
            voucher_id: order.voucher_id,
            notes: order.notes,
            created_at: order.created_at,
            updated_at: order.updated_at,
            expires_at: order.expires_at,
        }
    }
} 