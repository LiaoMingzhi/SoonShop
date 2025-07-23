/**
 * SoonShop核心智能合约提货券状态模块
 * 
 * 本模块定义了提货券相关的状态结构体，包括：
 * - 提货券信息
 * - 商品信息
 * - 消费记录
 */

use anchor_lang::prelude::*;
use crate::constants::*;

// ================================
// 提货券账户
// ================================

/**
 * 提货券信息
 * 
 * 存储提货券的详细信息和额度状态
 */
#[account]
#[derive(Debug)]
pub struct Voucher {
    /// 提货券唯一ID
    pub id: String,
    
    /// 生产者公钥
    pub producer: Pubkey,
    
    /// 商品信息
    pub product_info: ProductInfo,
    
    /// 总额度
    pub total_credits: u64,
    
    /// 已获取额度
    pub claimed_credits: u64,
    
    /// 已消费额度
    pub consumed_credits: u64,
    
    /// 状态
    pub status: VoucherStatus,
    
    /// 创建时间
    pub created_at: i64,
    
    /// 过期时间
    pub expires_at: Option<i64>,
    
    /// 最后更新时间
    pub updated_at: i64,
    
    /// 提货券配置
    pub config: VoucherConfig,
    
    /// 获取限制
    pub claim_restrictions: ClaimRestrictions,
    
    /// 提货券统计
    pub statistics: VoucherStatistics,
}

/**
 * 提货券状态枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum VoucherStatus {
    /// 活跃
    Active,
    /// 暂停
    Paused,
    /// 已结束
    Ended,
    /// 已过期
    Expired,
    /// 已取消
    Cancelled,
}

impl Default for VoucherStatus {
    fn default() -> Self {
        VoucherStatus::Active
    }
}

/**
 * 商品信息
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ProductInfo {
    /// 商品名称
    pub name: String,
    
    /// 商品描述
    pub description: String,
    
    /// 商品类别
    pub category: ProductCategory,
    
    /// 商品规格
    pub specifications: String,
    
    /// 商品单价
    pub unit_price: u64,
    
    /// 商品单位
    pub unit: String,
    
    /// 商品图片URL
    pub image_url: String,
    
    /// 生产地
    pub production_location: String,
    
    /// 生产日期
    pub production_date: Option<i64>,
    
    /// 保质期（天）
    pub shelf_life_days: Option<u32>,
    
    /// 质量等级
    pub quality_grade: QualityGrade,
    
    /// 认证信息
    pub certifications: Vec<String>,
}

/**
 * 商品类别枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ProductCategory {
    /// 食品
    Food,
    /// 服装
    Clothing,
    /// 电子产品
    Electronics,
    /// 家居用品
    Household,
    /// 书籍
    Books,
    /// 运动用品
    Sports,
    /// 美容护肤
    Beauty,
    /// 医疗保健
    Healthcare,
    /// 汽车用品
    Automotive,
    /// 工业用品
    Industrial,
    /// 服务类
    Services,
    /// 其他
    Other,
}

/**
 * 质量等级枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum QualityGrade {
    /// 优质
    Premium,
    /// 良好
    Good,
    /// 标准
    Standard,
    /// 基础
    Basic,
}

/**
 * 提货券配置
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VoucherConfig {
    /// 是否允许部分获取
    pub allow_partial_claim: bool,
    
    /// 是否允许转让
    pub allow_transfer: bool,
    
    /// 是否需要预约
    pub require_appointment: bool,
    
    /// 最小获取数量
    pub min_claim_amount: u64,
    
    /// 最大获取数量
    pub max_claim_amount: u64,
    
    /// 获取费用
    pub claim_fee: u64,
    
    /// 取消费用
    pub cancellation_fee: u64,
    
    /// 是否需要身份验证
    pub require_identity_verification: bool,
    
    /// 允许的用户类型
    pub allowed_user_types: Vec<UserType>,
    
    /// 地理限制
    pub geographic_restrictions: Vec<String>,
}

/**
 * 用户类型枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum UserType {
    /// 普通用户
    Regular,
    /// VIP用户
    VIP,
    /// 企业用户
    Enterprise,
    /// 政府用户
    Government,
    /// 学生用户
    Student,
    /// 老年用户
    Senior,
    /// 残疾人用户
    Disabled,
}

/**
 * 获取限制
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ClaimRestrictions {
    /// 每人获取限制
    pub per_user_limit: u64,
    
    /// 每日获取限制
    pub daily_limit: u64,
    
    /// 每月获取限制
    pub monthly_limit: u64,
    
    /// 总获取限制
    pub total_limit: u64,
    
    /// 当前已获取数量
    pub current_claimed: u64,
    
    /// 今日已获取数量
    pub today_claimed: u64,
    
    /// 本月已获取数量
    pub monthly_claimed: u64,
    
    /// 上次重置日期
    pub last_reset_date: i64,
    
    /// 获取开始时间
    pub claim_start_time: Option<i64>,
    
    /// 获取结束时间
    pub claim_end_time: Option<i64>,
}

/**
 * 提货券统计
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct VoucherStatistics {
    /// 获取次数
    pub claim_count: u64,
    
    /// 消费次数
    pub consumption_count: u64,
    
    /// 独立获取用户数
    pub unique_claimers: u64,
    
    /// 独立消费用户数
    pub unique_consumers: u64,
    
    /// 平均获取金额
    pub avg_claim_amount: u64,
    
    /// 平均消费金额
    pub avg_consumption_amount: u64,
    
    /// 总奖励分发
    pub total_rewards_distributed: u64,
    
    /// 平均质量评分
    pub avg_quality_score: f64,
    
    /// 最后统计更新时间
    pub last_stats_update: i64,
}

// ================================
// 消费记录账户
// ================================

/**
 * 消费记录信息
 * 
 * 存储提货券消费的详细记录
 */
#[account]
#[derive(Debug)]
pub struct ConsumptionRecord {
    /// 消费记录唯一ID
    pub id: String,
    
    /// 提货券ID
    pub voucher_id: String,
    
    /// 消费者公钥
    pub consumer: Pubkey,
    
    /// 商家公钥
    pub merchant: Pubkey,
    
    /// 消费金额
    pub amount: u64,
    
    /// 消费数量
    pub quantity: u64,
    
    /// 消费状态
    pub status: ConsumptionStatus,
    
    /// 消费时间
    pub consumed_at: i64,
    
    /// 确认时间
    pub confirmed_at: Option<i64>,
    
    /// 质量评分
    pub quality_score: Option<u8>,
    
    /// 消费地点
    pub location: String,
    
    /// 消费备注
    pub notes: String,
    
    /// 消费证明
    pub proof: ConsumptionProof,
    
    /// 奖励信息
    pub reward_info: RewardInfo,
}

/**
 * 消费状态枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ConsumptionStatus {
    /// 待确认
    Pending,
    /// 已确认
    Confirmed,
    /// 已完成
    Completed,
    /// 争议中
    Disputed,
    /// 已取消
    Cancelled,
}

impl Default for ConsumptionStatus {
    fn default() -> Self {
        ConsumptionStatus::Pending
    }
}

/**
 * 消费证明
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ConsumptionProof {
    /// 证明类型
    pub proof_type: ProofType,
    
    /// 证明数据
    pub proof_data: String,
    
    /// 证明时间
    pub proof_time: i64,
    
    /// 证明签名
    pub proof_signature: String,
    
    /// 是否已验证
    pub verified: bool,
}

/**
 * 证明类型枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ProofType {
    /// 照片证明
    Photo,
    /// 视频证明
    Video,
    /// 签名证明
    Signature,
    /// 位置证明
    Location,
    /// 时间证明
    Timestamp,
    /// 其他证明
    Other,
}

/**
 * 奖励信息
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RewardInfo {
    /// 基础奖励
    pub base_reward: u64,
    
    /// 质量奖励
    pub quality_reward: u64,
    
    /// 倍增奖励
    pub multiplier_reward: u64,
    
    /// 总奖励
    pub total_reward: u64,
    
    /// 奖励分发状态
    pub reward_status: RewardStatus,
    
    /// 奖励分发时间
    pub reward_distributed_at: Option<i64>,
    
    /// 奖励接收者
    pub reward_recipients: Vec<RewardRecipient>,
}

/**
 * 奖励状态枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum RewardStatus {
    /// 待计算
    Pending,
    /// 已计算
    Calculated,
    /// 已分发
    Distributed,
    /// 分发失败
    Failed,
}

/**
 * 奖励接收者
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RewardRecipient {
    /// 接收者公钥
    pub recipient: Pubkey,
    
    /// 奖励金额
    pub amount: u64,
    
    /// 奖励类型
    pub reward_type: String,
    
    /// 分发状态
    pub status: RewardStatus,
}

// ================================
// 实现
// ================================

impl Voucher {
    /// 计算账户所需空间
    pub const SPACE: usize = ACCOUNT_DISCRIMINATOR_SIZE
        + STRING_PREFIX_SIZE + 50    // id
        + PUBKEY_SIZE                // producer
        + ProductInfo::SPACE         // product_info
        + U64_SIZE * 3              // total_credits, claimed_credits, consumed_credits
        + 1                         // status enum
        + I64_SIZE                  // created_at
        + OPTION_FLAG_SIZE + I64_SIZE // expires_at
        + I64_SIZE                  // updated_at
        + VoucherConfig::SPACE      // config
        + ClaimRestrictions::SPACE  // claim_restrictions
        + VoucherStatistics::SPACE; // statistics

    /// 初始化提货券
    pub fn initialize(
        &mut self,
        id: String,
        producer: Pubkey,
        product_info: ProductInfo,
        total_credits: u64,
        expires_at: Option<i64>,
        config: VoucherConfig,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        self.id = id;
        self.producer = producer;
        self.product_info = product_info;
        self.total_credits = total_credits;
        self.claimed_credits = 0;
        self.consumed_credits = 0;
        self.status = VoucherStatus::Active;
        self.created_at = current_time;
        self.expires_at = expires_at;
        self.updated_at = current_time;
        self.config = config;
        self.claim_restrictions = ClaimRestrictions {
            per_user_limit: 0,
            daily_limit: 0,
            monthly_limit: 0,
            total_limit: total_credits,
            current_claimed: 0,
            today_claimed: 0,
            monthly_claimed: 0,
            last_reset_date: current_time / SECONDS_PER_DAY,
            claim_start_time: None,
            claim_end_time: expires_at,
        };
        self.statistics = VoucherStatistics::default();
        
        Ok(())
    }

    /// 获取提货券额度
    pub fn claim_credits(&mut self, amount: u64) -> Result<()> {
        // 检查状态
        if self.status != VoucherStatus::Active {
            return Err(crate::errors::SoonShopError::InvalidVoucherStatus.into());
        }
        
        // 检查过期时间
        if let Some(expires_at) = self.expires_at {
            if Clock::get()?.unix_timestamp > expires_at {
                self.status = VoucherStatus::Expired;
                return Err(crate::errors::SoonShopError::VoucherExpired.into());
            }
        }
        
        // 检查额度是否足够
        if self.claimed_credits + amount > self.total_credits {
            return Err(crate::errors::SoonShopError::InsufficientVoucherQuantity.into());
        }
        
        // 更新获取额度
        self.claimed_credits += amount;
        self.claim_restrictions.current_claimed += amount;
        self.statistics.claim_count += 1;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    /// 消费提货券额度
    pub fn consume_credits(&mut self, amount: u64) -> Result<()> {
        // 检查状态
        if self.status != VoucherStatus::Active {
            return Err(crate::errors::SoonShopError::InvalidVoucherStatus.into());
        }
        
        // 检查已获取额度是否足够
        if self.consumed_credits + amount > self.claimed_credits {
            return Err(crate::errors::SoonShopError::InsufficientVoucherQuantity.into());
        }
        
        // 更新消费额度
        self.consumed_credits += amount;
        self.statistics.consumption_count += 1;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    /// 检查是否可以获取
    pub fn can_claim(&self, amount: u64) -> bool {
        // 检查状态
        if self.status != VoucherStatus::Active {
            return false;
        }
        
        // 检查过期时间
        if let Some(expires_at) = self.expires_at {
            if Clock::get().unwrap().unix_timestamp > expires_at {
                return false;
            }
        }
        
        // 检查额度
        if self.claimed_credits + amount > self.total_credits {
            return false;
        }
        
        true
    }

    /// 获取可用额度
    pub fn available_credits(&self) -> u64 {
        self.total_credits - self.claimed_credits
    }

    /// 获取可消费额度
    pub fn consumable_credits(&self) -> u64 {
        self.claimed_credits - self.consumed_credits
    }
}

impl ConsumptionRecord {
    /// 计算账户所需空间
    pub const SPACE: usize = ACCOUNT_DISCRIMINATOR_SIZE
        + STRING_PREFIX_SIZE + 50    // id
        + STRING_PREFIX_SIZE + 50    // voucher_id
        + PUBKEY_SIZE * 2           // consumer, merchant
        + U64_SIZE * 2              // amount, quantity
        + 1                         // status enum
        + I64_SIZE                  // consumed_at
        + OPTION_FLAG_SIZE + I64_SIZE // confirmed_at
        + OPTION_FLAG_SIZE + U8_SIZE // quality_score
        + STRING_PREFIX_SIZE + 200  // location
        + STRING_PREFIX_SIZE + 500  // notes
        + ConsumptionProof::SPACE   // proof
        + RewardInfo::SPACE;        // reward_info

    /// 初始化消费记录
    pub fn initialize(
        &mut self,
        id: String,
        voucher_id: String,
        consumer: Pubkey,
        merchant: Pubkey,
        amount: u64,
        quantity: u64,
        location: String,
        notes: String,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        self.id = id;
        self.voucher_id = voucher_id;
        self.consumer = consumer;
        self.merchant = merchant;
        self.amount = amount;
        self.quantity = quantity;
        self.status = ConsumptionStatus::Pending;
        self.consumed_at = current_time;
        self.confirmed_at = None;
        self.quality_score = None;
        self.location = location;
        self.notes = notes;
        self.proof = ConsumptionProof {
            proof_type: ProofType::Timestamp,
            proof_data: current_time.to_string(),
            proof_time: current_time,
            proof_signature: String::new(),
            verified: false,
        };
        self.reward_info = RewardInfo {
            base_reward: 0,
            quality_reward: 0,
            multiplier_reward: 0,
            total_reward: 0,
            reward_status: RewardStatus::Pending,
            reward_distributed_at: None,
            reward_recipients: Vec::new(),
        };
        
        Ok(())
    }

    /// 确认消费
    pub fn confirm(&mut self, quality_score: u8) -> Result<()> {
        if self.status != ConsumptionStatus::Pending {
            return Err(crate::errors::SoonShopError::InvalidConsumptionStatus.into());
        }
        
        if quality_score < 1 || quality_score > 10 {
            return Err(crate::errors::SoonShopError::InvalidQualityScore.into());
        }
        
        self.status = ConsumptionStatus::Confirmed;
        self.confirmed_at = Some(Clock::get()?.unix_timestamp);
        self.quality_score = Some(quality_score);
        
        Ok(())
    }

    /// 完成消费
    pub fn complete(&mut self) -> Result<()> {
        if self.status != ConsumptionStatus::Confirmed {
            return Err(crate::errors::SoonShopError::InvalidConsumptionStatus.into());
        }
        
        self.status = ConsumptionStatus::Completed;
        Ok(())
    }
}

impl ProductInfo {
    pub const SPACE: usize = STRING_PREFIX_SIZE + 100  // name
        + STRING_PREFIX_SIZE + 500  // description
        + 1                         // category enum
        + STRING_PREFIX_SIZE + 200  // specifications
        + U64_SIZE                  // unit_price
        + STRING_PREFIX_SIZE + 20   // unit
        + STRING_PREFIX_SIZE + 200  // image_url
        + STRING_PREFIX_SIZE + 100  // production_location
        + OPTION_FLAG_SIZE + I64_SIZE // production_date
        + OPTION_FLAG_SIZE + U32_SIZE // shelf_life_days
        + 1                         // quality_grade enum
        + VEC_PREFIX_SIZE + 10 * (STRING_PREFIX_SIZE + 50); // certifications
}

impl VoucherConfig {
    pub const SPACE: usize = BOOL_SIZE * 4     // flags
        + U64_SIZE * 4                         // amounts and fees
        + VEC_PREFIX_SIZE + 10 * 1            // allowed_user_types
        + VEC_PREFIX_SIZE + 10 * (STRING_PREFIX_SIZE + 50); // geographic_restrictions
}

impl ClaimRestrictions {
    pub const SPACE: usize = U64_SIZE * 7 + I64_SIZE * 3 + OPTION_FLAG_SIZE * 2;
}

impl VoucherStatistics {
    pub const SPACE: usize = U64_SIZE * 7 + 8 + I64_SIZE; // f64 is 8 bytes
}

impl ConsumptionProof {
    pub const SPACE: usize = 1 + STRING_PREFIX_SIZE + 500 + I64_SIZE + STRING_PREFIX_SIZE + 100 + BOOL_SIZE;
}

impl RewardInfo {
    pub const SPACE: usize = U64_SIZE * 4 + 1 + OPTION_FLAG_SIZE + I64_SIZE + VEC_PREFIX_SIZE + 10 * RewardRecipient::SPACE;
}

impl RewardRecipient {
    pub const SPACE: usize = PUBKEY_SIZE + U64_SIZE + STRING_PREFIX_SIZE + 50 + 1;
} 