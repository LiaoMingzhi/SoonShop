use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

// ============ 购物车相关模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingCart {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<CartItem>,
    pub total_amount: f64,
    pub total_quantity: u32,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_image: Option<String>,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_price: f64,
    pub product_attributes: Option<serde_json::Value>, // 商品属性如颜色、尺寸等
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddToCartRequest {
    pub user_id: Uuid,
    pub product_id: Uuid,
    #[validate(range(min = 1, max = 100))]
    pub quantity: u32,
    pub product_attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateCartItemRequest {
    #[validate(range(min = 1, max = 100))]
    pub quantity: u32,
}

// ============ B2C订单相关模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2COrder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_number: String,
    pub status: OrderStatus,
    pub items: Vec<OrderItem>,
    pub shipping_address: ShippingAddress,
    pub billing_address: Option<BillingAddress>,
    pub payment_method: PaymentMethod,
    pub subtotal: f64,
    pub shipping_fee: f64,
    pub tax_amount: f64,
    pub discount_amount: f64,
    pub total_amount: f64,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_image: Option<String>,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_price: f64,
    pub product_attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,        // 待确认
    Confirmed,      // 已确认
    Paid,          // 已支付
    Processing,    // 处理中
    Shipped,       // 已发货
    Delivered,     // 已送达
    Cancelled,     // 已取消
    Refunded,      // 已退款
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub recipient_name: String,
    pub phone: String,
    pub country: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub street_address: String,
    pub postal_code: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
    pub phone: String,
    pub country: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub street_address: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    PayPal,
    BankTransfer,
    DigitalWallet,
    CashOnDelivery,
    Cryptocurrency,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOrderRequest {
    pub user_id: Uuid,
    pub cart_id: Uuid,
    pub shipping_address: ShippingAddress,
    pub billing_address: Option<BillingAddress>,
    pub payment_method: PaymentMethod,
    pub coupon_code: Option<String>,
    #[validate(length(max = 500))]
    pub notes: Option<String>,
}

// ============ 商品浏览相关模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBrowseHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_image: Option<String>,
    pub category_id: Uuid,
    pub browse_duration: Option<u32>, // 浏览时长（秒）
    pub browsed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRecommendation {
    pub product_id: Uuid,
    pub product_name: String,
    pub product_image: Option<String>,
    pub price: f64,
    pub currency: String,
    pub rating: f64,
    pub recommendation_score: f64,
    pub recommendation_reason: String, // 推荐原因
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProductSearchRequest {
    #[validate(length(min = 1, max = 100))]
    pub keyword: String,
    pub category_id: Option<Uuid>,
    pub price_range: Option<PriceRange>,
    pub sort_by: Option<SortOption>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRange {
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOption {
    Relevance,      // 相关度
    PriceLowToHigh, // 价格从低到高
    PriceHighToLow, // 价格从高到低
    Rating,         // 评分
    Popularity,     // 热度
    Newest,         // 最新
}

// ============ 用户偏好模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub preferred_categories: Vec<Uuid>,
    pub preferred_brands: Vec<String>,
    pub price_sensitivity: PriceSensitivity,
    pub shopping_frequency: ShoppingFrequency,
    pub preferred_payment_method: Option<PaymentMethod>,
    pub preferred_shipping_address: Option<Uuid>,
    pub marketing_preferences: MarketingPreferences,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceSensitivity {
    High,    // 价格敏感
    Medium,  // 中等
    Low,     // 不敏感
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShoppingFrequency {
    Daily,     // 每日
    Weekly,    // 每周
    Monthly,   // 每月
    Seasonal,  // 季节性
    Occasional,// 偶尔
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingPreferences {
    pub email_notifications: bool,
    pub sms_notifications: bool,
    pub push_notifications: bool,
    pub promotional_emails: bool,
    pub product_recommendations: bool,
}

// ============ 响应模型 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingCartResponse {
    pub cart: ShoppingCart,
    pub recommendations: Vec<ProductRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: B2COrder,
    pub estimated_delivery_date: Option<DateTime<Utc>>,
    pub tracking_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSearchResponse {
    pub products: Vec<ProductSearchResult>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSearchResult {
    pub product_id: Uuid,
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub price: f64,
    pub currency: String,
    pub rating: f64,
    pub review_count: u32,
    pub category_id: Uuid,
    pub category_name: String,
    pub brand: String,
    pub in_stock: bool,
    pub stock_quantity: u32,
    pub attributes: serde_json::Value,
}

// ============ 实现转换 ============

impl From<ShoppingCart> for ShoppingCartResponse {
    fn from(cart: ShoppingCart) -> Self {
        ShoppingCartResponse {
            cart,
            recommendations: Vec::new(), // 在服务层填充
        }
    }
}

impl From<B2COrder> for OrderResponse {
    fn from(order: B2COrder) -> Self {
        OrderResponse {
            order,
            estimated_delivery_date: None, // 在服务层计算
            tracking_number: None,          // 在发货后填充
        }
    }
} 