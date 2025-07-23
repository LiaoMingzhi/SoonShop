// 订单相关模式定义 - 基本实现
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use std::collections::HashMap;

/// 订单实体模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSchema {
    pub id: Uuid,
    pub order_number: String,
    pub user_id: Uuid,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub currency: String,
    pub shipping_address: AddressInfo,
    pub billing_address: Option<AddressInfo>,
    pub payment_method: Option<String>,
    pub payment_status: PaymentStatus,
    pub shipping_method: Option<String>,
    pub shipping_fee: f64,
    pub tax_amount: f64,
    pub discount_amount: f64,
    pub notes: Option<String>,
    pub metadata: HashMap<String, String>,
    pub ordered_at: DateTime<Utc>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 订单状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Returned,
    Refunded,
}

/// 支付状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaymentStatus {
    Pending,
    Authorized,
    Captured,
    Failed,
    Cancelled,
    Refunded,
    PartiallyRefunded,
}

/// 地址信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
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
    pub email: Option<String>,
}

/// 订单项模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemSchema {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub product_variant_id: Option<Uuid>,
    pub product_name: String,
    pub product_sku: String,
    pub unit_price: f64,
    pub quantity: u32,
    pub total_price: f64,
    pub product_snapshot: ProductSnapshot,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 商品快照（下单时的商品信息）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSnapshot {
    pub name: String,
    pub description: Option<String>,
    pub images: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub weight: Option<f64>,
    pub dimensions: Option<String>,
}

/// 订单状态历史模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusHistorySchema {
    pub id: Uuid,
    pub order_id: Uuid,
    pub from_status: Option<OrderStatus>,
    pub to_status: OrderStatus,
    pub changed_by: Option<Uuid>,
    pub reason: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// 退款模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundSchema {
    pub id: Uuid,
    pub order_id: Uuid,
    pub refund_number: String,
    pub amount: f64,
    pub currency: String,
    pub reason: Option<String>,
    pub status: RefundStatus,
    pub processed_by: Option<Uuid>,
    pub processed_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 退款状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefundStatus {
    Pending,
    Approved,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// 创建订单请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderSchema {
    pub items: Vec<CreateOrderItemSchema>,
    pub shipping_address: CreateAddressSchema,
    pub billing_address: Option<CreateAddressSchema>,
    pub payment_method: Option<String>,
    pub shipping_method: Option<String>,
    pub notes: Option<String>,
    pub coupon_code: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// 创建订单项请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderItemSchema {
    pub product_id: Uuid,
    pub product_variant_id: Option<Uuid>,
    
    #[validate(range(min = 1))]
    pub quantity: u32,
}

/// 创建地址请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateAddressSchema {
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
    
    #[validate(email)]
    pub email: Option<String>,
}

/// 更新订单状态请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderStatusSchema {
    pub status: OrderStatus,
    pub reason: Option<String>,
    pub tracking_number: Option<String>,
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub metadata: Option<HashMap<String, String>>,
}

/// 创建退款请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateRefundSchema {
    pub order_id: Uuid,
    
    #[validate(range(min = 0.01))]
    pub amount: f64,
    
    #[validate(length(max = 500))]
    pub reason: Option<String>,
    
    pub refund_items: Option<Vec<RefundItemSchema>>,
}

/// 退款项
#[derive(Debug, Deserialize, Validate)]
pub struct RefundItemSchema {
    pub order_item_id: Uuid,
    
    #[validate(range(min = 1))]
    pub quantity: u32,
    
    #[validate(range(min = 0.01))]
    pub amount: f64,
}

/// 订单搜索查询参数
#[derive(Debug, Deserialize, Validate)]
pub struct OrderSearchQuerySchema {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>, // 订单号或用户名搜索
    pub user_id: Option<Uuid>,
    pub status: Option<OrderStatus>,
    pub payment_status: Option<PaymentStatus>,
    
    #[validate(range(min = 0.0))]
    pub min_amount: Option<f64>,
    
    #[validate(range(min = 0.0))]
    pub max_amount: Option<f64>,
    
    pub currency: Option<String>,
    pub payment_method: Option<String>,
    pub shipping_method: Option<String>,
    pub sort_by: Option<OrderSortField>,
    pub sort_order: Option<SortOrder>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub shipped_after: Option<DateTime<Utc>>,
    pub shipped_before: Option<DateTime<Utc>>,
}

/// 订单排序字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderSortField {
    OrderNumber,
    TotalAmount,
    Status,
    OrderedAt,
    ShippedAt,
    DeliveredAt,
    CreatedAt,
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

/// 物流信息模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingInfoSchema {
    pub id: Uuid,
    pub order_id: Uuid,
    pub carrier: String,
    pub tracking_number: String,
    pub service_type: String,
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub tracking_url: Option<String>,
    pub status: ShippingStatus,
    pub tracking_events: Vec<TrackingEvent>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 物流状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShippingStatus {
    Created,
    PickedUp,
    InTransit,
    OutForDelivery,
    Delivered,
    Returned,
    Exception,
}

/// 物流跟踪事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingEvent {
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub description: String,
    pub location: Option<String>,
}

/// 优惠券模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponSchema {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub discount_value: f64,
    pub minimum_amount: Option<f64>,
    pub maximum_discount: Option<f64>,
    pub usage_limit: Option<u32>,
    pub used_count: u32,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub is_active: bool,
    pub applicable_products: Vec<Uuid>,
    pub applicable_categories: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 折扣类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscountType {
    Percentage,
    FixedAmount,
    FreeShipping,
}

/// 订单统计模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatsSchema {
    pub total_orders: u64,
    pub total_revenue: f64,
    pub average_order_value: f64,
    pub orders_by_status: HashMap<OrderStatus, u64>,
    pub orders_by_payment_status: HashMap<PaymentStatus, u64>,
    pub top_products: Vec<ProductSalesStats>,
    pub revenue_by_period: Vec<RevenuePeriod>,
    pub updated_at: DateTime<Utc>,
}

/// 商品销售统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSalesStats {
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity_sold: u32,
    pub revenue: f64,
}

/// 时期收入统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenuePeriod {
    pub period: String, // "2024-01", "2024-01-15" etc.
    pub revenue: f64,
    pub order_count: u64,
}

/// 订单响应模式（包含所有相关信息）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponseSchema {
    pub order: OrderSchema,
    pub items: Vec<OrderItemSchema>,
    pub status_history: Vec<OrderStatusHistorySchema>,
    pub shipping_info: Option<ShippingInfoSchema>,
    pub refunds: Vec<RefundSchema>,
    pub applied_coupons: Vec<CouponSchema>,
}

/// 购物车模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartSchema {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<CartItemSchema>,
    pub total_amount: f64,
    pub currency: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 购物车项模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemSchema {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub product_variant_id: Option<Uuid>,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_price: f64,
    pub added_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 添加到购物车请求
#[derive(Debug, Deserialize, Validate)]
pub struct AddToCartSchema {
    pub product_id: Uuid,
    pub product_variant_id: Option<Uuid>,
    
    #[validate(range(min = 1))]
    pub quantity: u32,
}

/// 更新购物车项请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCartItemSchema {
    pub cart_item_id: Uuid,
    
    #[validate(range(min = 1))]
    pub quantity: u32,
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