//! SoonShop 客户端类型定义

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// 创建用户参数
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct CreateUserParams {
    /// 用户名
    pub username: String,
    /// 电子邮件
    pub email: String,
    /// 用户类型
    pub user_type: UserType,
}

/// 用户类型
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum UserType {
    /// 普通用户
    Regular,
    /// 商家
    Merchant,
    /// 管理员
    Admin,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct UserInfo {
    /// 用户公钥
    pub user_pubkey: Pubkey,
    /// 用户名
    pub username: String,
    /// 电子邮件
    pub email: String,
    /// 用户类型
    pub user_type: UserType,
    /// 创建时间
    pub created_at: i64,
    /// 最后更新时间
    pub updated_at: i64,
}

/// 产品信息
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct ProductInfo {
    /// 产品ID
    pub product_id: u64,
    /// 产品名称
    pub name: String,
    /// 产品描述
    pub description: String,
    /// 价格（以最小单位计）
    pub price: u64,
    /// 库存数量
    pub stock_quantity: u64,
    /// 商家公钥
    pub merchant_pubkey: Pubkey,
    /// 创建时间
    pub created_at: i64,
}

/// 订单信息
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct OrderInfo {
    /// 订单ID
    pub order_id: u64,
    /// 买家公钥
    pub buyer_pubkey: Pubkey,
    /// 产品ID
    pub product_id: u64,
    /// 数量
    pub quantity: u64,
    /// 总价
    pub total_price: u64,
    /// 订单状态
    pub status: OrderStatus,
    /// 创建时间
    pub created_at: i64,
}

/// 订单状态
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum OrderStatus {
    /// 待付款
    Pending,
    /// 已付款
    Paid,
    /// 已发货
    Shipped,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

/// 支付信息
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct PaymentInfo {
    /// 支付ID
    pub payment_id: u64,
    /// 订单ID
    pub order_id: u64,
    /// 支付金额
    pub amount: u64,
    /// 支付方式
    pub payment_method: PaymentMethod,
    /// 支付状态
    pub status: PaymentStatus,
    /// 创建时间
    pub created_at: i64,
}

/// 支付方式
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum PaymentMethod {
    /// SOL 代币
    Sol,
    /// USDC 代币
    Usdc,
    /// 其他 SPL 代币
    SplToken { mint: Pubkey },
}

/// 支付状态
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum PaymentStatus {
    /// 待处理
    Pending,
    /// 已完成
    Completed,
    /// 已失败
    Failed,
    /// 已退款
    Refunded,
} 