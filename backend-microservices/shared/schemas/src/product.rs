// 产品相关模式定义
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use std::collections::HashMap;

/// 商品实体模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSchema {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub category_id: Uuid,
    pub producer_id: Uuid,
    pub price: f64,
    pub currency: String,
    pub weight: Option<f64>,
    pub dimensions: Option<ProductDimensions>,
    pub images: Vec<ProductImage>,
    pub attributes: HashMap<String, String>,
    pub status: ProductStatus,
    pub is_digital: bool,
    pub min_order_quantity: u32,
    pub max_order_quantity: Option<u32>,
    pub tags: Vec<String>,
    pub seo: Option<ProductSEO>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 商品状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductStatus {
    Active,
    Inactive,
    Draft,
    OutOfStock,
    Discontinued,
}

/// 商品尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub unit: DimensionUnit,
}

/// 尺寸单位
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DimensionUnit {
    Cm,
    Inch,
    Mm,
    M,
}

/// 商品图片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductImage {
    pub id: Uuid,
    pub url: String,
    pub alt_text: Option<String>,
    pub is_primary: bool,
    pub sort_order: u32,
}

/// 商品SEO信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSEO {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub slug: String,
}

/// 商品分类模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCategorySchema {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub sort_order: u32,
    pub is_active: bool,
    pub image_url: Option<String>,
    pub seo: Option<CategorySEO>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 分类SEO信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySEO {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub slug: String,
}

/// 商品变体模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantSchema {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub attributes: HashMap<String, String>,
    pub images: Vec<ProductImage>,
    pub status: ProductStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建商品请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductSchema {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(length(max = 2000))]
    pub description: Option<String>,
    
    #[validate(length(min = 1, max = 100))]
    pub sku: String,
    
    pub category_id: Uuid,
    pub producer_id: Uuid,
    
    #[validate(range(min = 0.01))]
    pub price: f64,
    
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    
    #[validate(range(min = 0.0))]
    pub weight: Option<f64>,
    
    pub dimensions: Option<ProductDimensions>,
    pub images: Option<Vec<CreateProductImageSchema>>,
    pub attributes: Option<HashMap<String, String>>,
    pub status: Option<ProductStatus>,
    pub is_digital: Option<bool>,
    
    #[validate(range(min = 1))]
    pub min_order_quantity: Option<u32>,
    
    #[validate(range(min = 1))]
    pub max_order_quantity: Option<u32>,
    
    pub tags: Option<Vec<String>>,
    pub seo: Option<CreateProductSEOSchema>,
}

/// 创建商品图片请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductImageSchema {
    #[validate(url)]
    pub url: String,
    
    #[validate(length(max = 255))]
    pub alt_text: Option<String>,
    
    pub is_primary: Option<bool>,
    pub sort_order: Option<u32>,
}

/// 创建商品SEO请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductSEOSchema {
    #[validate(length(max = 100))]
    pub title: Option<String>,
    
    #[validate(length(max = 300))]
    pub description: Option<String>,
    
    pub keywords: Option<Vec<String>>,
    
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
}

/// 更新商品请求模式
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductSchema {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    
    #[validate(length(max = 2000))]
    pub description: Option<String>,
    
    #[validate(length(min = 1, max = 100))]
    pub sku: Option<String>,
    
    pub category_id: Option<Uuid>,
    
    #[validate(range(min = 0.01))]
    pub price: Option<f64>,
    
    #[validate(length(min = 3, max = 3))]
    pub currency: Option<String>,
    
    #[validate(range(min = 0.0))]
    pub weight: Option<f64>,
    
    pub dimensions: Option<ProductDimensions>,
    pub status: Option<ProductStatus>,
    pub is_digital: Option<bool>,
    
    #[validate(range(min = 1))]
    pub min_order_quantity: Option<u32>,
    
    #[validate(range(min = 1))]
    pub max_order_quantity: Option<u32>,
    
    pub tags: Option<Vec<String>>,
    pub seo: Option<CreateProductSEOSchema>,
}

/// 创建分类请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategorySchema {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(max = 500))]
    pub description: Option<String>,
    
    pub parent_id: Option<Uuid>,
    pub sort_order: Option<u32>,
    pub is_active: Option<bool>,
    
    #[validate(url)]
    pub image_url: Option<String>,
    
    pub seo: Option<CreateCategorySEOSchema>,
}

/// 创建分类SEO请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategorySEOSchema {
    #[validate(length(max = 100))]
    pub title: Option<String>,
    
    #[validate(length(max = 300))]
    pub description: Option<String>,
    
    pub keywords: Option<Vec<String>>,
    
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
}

/// 创建变体请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateVariantSchema {
    pub product_id: Uuid,
    
    #[validate(length(min = 1, max = 100))]
    pub sku: String,
    
    #[validate(length(max = 255))]
    pub name: Option<String>,
    
    #[validate(range(min = 0.01))]
    pub price: Option<f64>,
    
    pub attributes: Option<HashMap<String, String>>,
    pub images: Option<Vec<CreateProductImageSchema>>,
    pub status: Option<ProductStatus>,
}

/// 商品搜索查询模式
#[derive(Debug, Deserialize, Validate)]
pub struct ProductSearchQuerySchema {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    
    #[validate(length(min = 1))]
    pub search: Option<String>,
    
    pub category_id: Option<Uuid>,
    pub producer_id: Option<Uuid>,
    pub status: Option<ProductStatus>,
    pub is_digital: Option<bool>,
    
    #[validate(range(min = 0.0))]
    pub min_price: Option<f64>,
    
    #[validate(range(min = 0.0))]
    pub max_price: Option<f64>,
    
    pub currency: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sort_by: Option<ProductSortField>,
    pub sort_order: Option<SortOrder>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
}

/// 商品排序字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductSortField {
    Name,
    Price,
    CreatedAt,
    UpdatedAt,
    PopularityScore,
    Rating,
}

/// 排序顺序
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// 商品库存模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInventorySchema {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub warehouse_id: Uuid,
    pub quantity_available: u32,
    pub quantity_reserved: u32,
    pub quantity_on_order: u32,
    pub reorder_level: u32,
    pub max_stock_level: Option<u32>,
    pub location: Option<String>,
    pub cost: Option<f64>,
    pub last_stock_take: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 商品评价模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReviewSchema {
    pub id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: u8, // 1-5 stars
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_verified_purchase: bool,
    pub is_approved: bool,
    pub helpful_count: u32,
    pub images: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建评价请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateReviewSchema {
    pub product_id: Uuid,
    
    #[validate(range(min = 1, max = 5))]
    pub rating: u8,
    
    #[validate(length(max = 100))]
    pub title: Option<String>,
    
    #[validate(length(max = 2000))]
    pub content: Option<String>,
    
    pub images: Option<Vec<String>>,
}

/// 商品价格历史模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPriceHistorySchema {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub old_price: f64,
    pub new_price: f64,
    pub currency: String,
    pub change_reason: PriceChangeReason,
    pub changed_by: Uuid,
    pub effective_from: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// 价格变更原因
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PriceChangeReason {
    Promotion,
    CostChange,
    MarketAdjustment,
    Discount,
    RegularUpdate,
    Other,
}

/// 商品统计模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductStatsSchema {
    pub product_id: Uuid,
    pub view_count: u64,
    pub purchase_count: u64,
    pub wishlist_count: u64,
    pub average_rating: f64,
    pub review_count: u32,
    pub revenue_total: f64,
    pub last_purchased_at: Option<DateTime<Utc>>,
    pub popularity_score: f64,
    pub updated_at: DateTime<Utc>,
}

/// 商品响应模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponseSchema {
    pub product: ProductSchema,
    pub category: ProductCategorySchema,
    pub variants: Vec<ProductVariantSchema>,
    pub inventory: Vec<ProductInventorySchema>,
    pub reviews_summary: ProductReviewSummary,
    pub stats: ProductStatsSchema,
}

/// 商品评价摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReviewSummary {
    pub average_rating: f64,
    pub total_reviews: u32,
    pub rating_distribution: HashMap<u8, u32>, // rating -> count
    pub recent_reviews: Vec<ProductReviewSchema>,
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