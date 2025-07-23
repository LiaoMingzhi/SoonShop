/**
 * SoonShop核心智能合约常量定义
 * 
 * 本文件定义了合约中使用的所有常量，包括：
 * - 倍增系数配置
 * - 时间相关常量
 * - 权限配置
 * - 费用和限制
 * - 种子常量
 */

use anchor_lang::prelude::*;

// ================================
// 程序种子常量
// ================================

/// 平台配置种子
pub const PLATFORM_CONFIG_SEED: &[u8] = b"platform_config";

/// 用户账户种子
pub const USER_ACCOUNT_SEED: &[u8] = b"user_account";

/// 提货券种子
pub const VOUCHER_SEED: &[u8] = b"voucher";

/// 提货券获取记录种子
pub const VOUCHER_CLAIM_SEED: &[u8] = b"voucher_claim";

/// 消费记录种子
pub const CONSUMPTION_SEED: &[u8] = b"consumption";

/// 企业评估种子
pub const EVALUATION_SEED: &[u8] = b"evaluation";

/// 价格数据种子
pub const PRICE_DATA_SEED: &[u8] = b"price_data";

/// 奖励池种子
pub const REWARD_POOL_SEED: &[u8] = b"reward_pool";

/// 平台权限种子
pub const PLATFORM_AUTHORITY_SEED: &str = "platform_authority";

// ================================
// 倍增系数相关常量
// ================================

/// 默认基础倍增系数（2倍）
pub const DEFAULT_BASE_MULTIPLIER: u8 = 2;

/// 最小倍增系数（1倍，即无奖励）
pub const MIN_MULTIPLIER: u8 = 1;

/// 最大倍增系数（10倍）
pub const MAX_MULTIPLIER: u8 = 10;

/// 生产者奖励比例（70%）
pub const PRODUCER_REWARD_RATIO: u32 = 70;

/// 职工奖励比例（30%）
pub const WORKER_REWARD_RATIO: u32 = 30;

/// 比例基数（100%）
pub const RATIO_BASE: u32 = 100;

/// 链式倍增传递最大层级
pub const MAX_CHAIN_LEVELS: u8 = 5;

/// 链式倍增每层衰减比例（90%）
pub const CHAIN_DECAY_RATIO: u32 = 90;

// ================================
// 评估相关常量
// ================================

/// 评估分数最小值
pub const MIN_EVALUATION_SCORE: u8 = 0;

/// 评估分数最大值
pub const MAX_EVALUATION_SCORE: u8 = 100;

/// 评估分数及格线
pub const EVALUATION_PASS_THRESHOLD: u8 = 60;

/// 评估分数优秀线
pub const EVALUATION_EXCELLENT_THRESHOLD: u8 = 90;

/// 企业评估最小间隔（30天）
pub const MIN_EVALUATION_INTERVAL: i64 = 30 * 24 * 60 * 60;

/// 评估有效期（180天）
pub const EVALUATION_VALIDITY_PERIOD: i64 = 180 * 24 * 60 * 60;

// ================================
// 价格监控相关常量
// ================================

/// 价格异常波动阈值（20%）
pub const PRICE_VOLATILITY_THRESHOLD: u32 = 20;

/// 价格数据最大历史记录数量
pub const MAX_PRICE_HISTORY_RECORDS: usize = 100;

/// 价格更新最小间隔（5分钟）
pub const MIN_PRICE_UPDATE_INTERVAL: i64 = 5 * 60;

/// 价格预警阈值（10%）
pub const PRICE_WARNING_THRESHOLD: u32 = 10;

// ================================
// 提货券相关常量
// ================================

/// 提货券默认有效期（90天）
pub const DEFAULT_VOUCHER_VALIDITY: i64 = 90 * 24 * 60 * 60;

/// 提货券最大有效期（365天）
pub const MAX_VOUCHER_VALIDITY: i64 = 365 * 24 * 60 * 60;

/// 单次获取提货券最大数量
pub const MAX_VOUCHER_CLAIM_QUANTITY: u32 = 100;

/// 提货券描述最大长度
pub const MAX_VOUCHER_DESCRIPTION_LENGTH: usize = 500;

/// 商品名称最大长度
pub const MAX_PRODUCT_NAME_LENGTH: usize = 100;

// ================================
// 时间相关常量
// ================================

/// 一天的秒数
pub const SECONDS_PER_DAY: i64 = 24 * 60 * 60;

/// 一周的秒数
pub const SECONDS_PER_WEEK: i64 = 7 * SECONDS_PER_DAY;

/// 一个月的秒数（30天）
pub const SECONDS_PER_MONTH: i64 = 30 * SECONDS_PER_DAY;

/// 一年的秒数（365天）
pub const SECONDS_PER_YEAR: i64 = 365 * SECONDS_PER_DAY;

// ================================
// 费用相关常量
// ================================

/// 平台费率基点（100 = 1%）
pub const DEFAULT_PLATFORM_FEE_RATE: u16 = 50; // 0.5%

/// 最大平台费率基点（500 = 5%）
pub const MAX_PLATFORM_FEE_RATE: u16 = 500;

/// 基点基数（10000 = 100%）
pub const BASIS_POINTS_BASE: u16 = 10000;

// ================================
// 限制相关常量
// ================================

/// 字符串字段最大长度
pub const MAX_STRING_LENGTH: usize = 200;

/// 地址字段最大长度
pub const MAX_ADDRESS_LENGTH: usize = 300;

/// URL字段最大长度
pub const MAX_URL_LENGTH: usize = 500;

/// 评语最大长度
pub const MAX_COMMENT_LENGTH: usize = 1000;

/// 用户名最大长度
pub const MAX_USERNAME_LENGTH: usize = 50;

/// 用户名最小长度
pub const MIN_USERNAME_LENGTH: usize = 3;

// ================================
// 账户空间相关常量
// ================================

/// 账户数据头部大小（8字节用于锚点鉴别器）
pub const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;

/// Pubkey大小（32字节）
pub const PUBKEY_SIZE: usize = 32;

/// 布尔值大小（1字节）
pub const BOOL_SIZE: usize = 1;

/// u8大小（1字节）
pub const U8_SIZE: usize = 1;

/// u16大小（2字节）
pub const U16_SIZE: usize = 2;

/// u32大小（4字节）
pub const U32_SIZE: usize = 4;

/// u64大小（8字节）
pub const U64_SIZE: usize = 8;

/// i64大小（8字节）
pub const I64_SIZE: usize = 8;

/// f32大小（4字节）
pub const F32_SIZE: usize = 4;

/// f64大小（8字节）
pub const F64_SIZE: usize = 8;

// ================================
// 权限相关常量
// ================================

/// 超级管理员最大数量
pub const MAX_SUPER_ADMINS: usize = 3;

/// 管理员最大数量
pub const MAX_ADMINS: usize = 10;

/// 评估员最大数量
pub const MAX_EVALUATORS: usize = 100;

// ================================
// 业务逻辑相关常量
// ================================

/// 消费确认超时时间（7天）
pub const CONSUMPTION_CONFIRMATION_TIMEOUT: i64 = 7 * SECONDS_PER_DAY;

/// 争议处理超时时间（30天）
pub const DISPUTE_RESOLUTION_TIMEOUT: i64 = 30 * SECONDS_PER_DAY;

/// 自动结算延迟时间（1天）
pub const AUTO_SETTLEMENT_DELAY: i64 = SECONDS_PER_DAY;

/// 质量评分权重基数
pub const QUALITY_SCORE_WEIGHT_BASE: u32 = 100;

/// 最小质量评分
pub const MIN_QUALITY_SCORE: u8 = 1;

/// 最大质量评分
pub const MAX_QUALITY_SCORE: u8 = 10;

// ================================
// 紧急状态相关常量
// ================================

/// 紧急暂停自动恢复时间（24小时）
pub const EMERGENCY_AUTO_RESUME_TIME: i64 = 24 * 60 * 60;

/// 最大紧急暂停次数（防止滥用）
pub const MAX_EMERGENCY_PAUSES_PER_DAY: u8 = 3;

// ================================
// 数据结构相关常量
// ================================

/// Vec<u8>长度前缀大小（4字节）
pub const VEC_PREFIX_SIZE: usize = 4;

/// String长度前缀大小（4字节）
pub const STRING_PREFIX_SIZE: usize = 4;

/// Option<T>标志大小（1字节）
pub const OPTION_FLAG_SIZE: usize = 1;

// ================================
// 加密相关常量
// ================================

/// 哈希值大小（32字节）
pub const HASH_SIZE: usize = 32;

/// 签名大小（64字节）
pub const SIGNATURE_SIZE: usize = 64;

// ================================
// 预言机相关常量
// ================================

/// 价格精度（小数点后6位）
pub const PRICE_PRECISION: u32 = 1_000_000;

/// 最大价格延迟容忍度（5分钟）
pub const MAX_PRICE_STALENESS: i64 = 5 * 60;

/// 最小置信度要求（95%）
pub const MIN_CONFIDENCE_THRESHOLD: u32 = 95; 