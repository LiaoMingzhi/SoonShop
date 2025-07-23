/**
 * SoonShop核心智能合约钱包状态模块
 * 
 * 本模块定义了用户钱包相关的状态结构体，包括：
 * - 用户钱包信息
 * - 交易记录
 * - 余额管理
 */

use anchor_lang::prelude::*;
use crate::constants::*;

// ================================
// 用户钱包账户
// ================================

/**
 * 用户钱包信息
 * 
 * 存储用户的资产信息和交易历史统计
 */
#[account]
#[derive(Debug)]
pub struct UserWallet {
    /// 钱包所有者
    pub owner: Pubkey,
    
    /// 平台代币账户
    pub platform_token_account: Pubkey,
    
    /// 总收入
    pub total_income: u64,
    
    /// 总支出
    pub total_expense: u64,
    
    /// 总奖励
    pub total_rewards: u64,
    
    /// 交易次数
    pub transaction_count: u64,
    
    /// 最后交易时间
    pub last_transaction_at: i64,
    
    /// 钱包状态
    pub status: WalletStatus,
    
    /// 创建时间
    pub created_at: i64,
    
    /// 更新时间
    pub updated_at: i64,
    
    /// 钱包版本
    pub version: u8,
    
    /// 钱包权限设置
    pub permissions: WalletPermissions,
    
    /// 钱包统计信息
    pub statistics: WalletStatistics,
}

/**
 * 钱包状态枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum WalletStatus {
    /// 正常状态
    Active,
    /// 已冻结
    Frozen,
    /// 已暂停
    Suspended,
    /// 已关闭
    Closed,
}

impl Default for WalletStatus {
    fn default() -> Self {
        WalletStatus::Active
    }
}

/**
 * 钱包权限设置
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WalletPermissions {
    /// 是否可以接收代币
    pub can_receive: bool,
    
    /// 是否可以发送代币
    pub can_send: bool,
    
    /// 是否可以查看历史记录
    pub can_view_history: bool,
    
    /// 日转账限额
    pub daily_transfer_limit: u64,
    
    /// 已使用的日转账额度
    pub daily_transfer_used: u64,
    
    /// 上次重置日期
    pub last_reset_date: i64,
}

/**
 * 钱包统计信息
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WalletStatistics {
    /// 今日收入
    pub today_income: u64,
    
    /// 今日支出
    pub today_expense: u64,
    
    /// 今日奖励
    pub today_rewards: u64,
    
    /// 本月收入
    pub monthly_income: u64,
    
    /// 本月支出
    pub monthly_expense: u64,
    
    /// 本月奖励
    pub monthly_rewards: u64,
    
    /// 最大单笔收入
    pub max_single_income: u64,
    
    /// 最大单笔支出
    pub max_single_expense: u64,
    
    /// 平均交易金额
    pub avg_transaction_amount: u64,
    
    /// 最后统计更新时间
    pub last_stats_update: i64,
}

// ================================
// 交易记录账户
// ================================

/**
 * 交易记录信息
 * 
 * 存储用户的收入、支出、奖励等交易记录
 */
#[account]
#[derive(Debug)]
pub struct TransactionRecord {
    /// 交易唯一ID
    pub id: String,
    
    /// 用户公钥
    pub user: Pubkey,
    
    /// 交易类型
    pub transaction_type: TransactionType,
    
    /// 交易金额
    pub amount: u64,
    
    /// 代币类型
    pub token_mint: Pubkey,
    
    /// 交易对手方
    pub counterparty: Option<Pubkey>,
    
    /// 交易描述
    pub description: String,
    
    /// 元数据(JSON格式)
    pub metadata: String,
    
    /// 交易状态
    pub status: TransactionStatus,
    
    /// 交易时间
    pub timestamp: i64,
    
    /// 区块高度
    pub block_height: u64,
    
    /// 交易签名
    pub signature: String,
}

/**
 * 交易类型枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TransactionType {
    /// 收入
    Income,
    /// 支出
    Expense,
    /// 奖励
    Reward,
    /// 转账发送
    TransferSent,
    /// 转账接收
    TransferReceived,
    /// 提货券获取
    VoucherClaim,
    /// 提货券消费
    VoucherConsumption,
    /// 代币铸造
    TokenMint,
    /// 代币销毁
    TokenBurn,
    /// 倍增奖励
    MultiplierReward,
    /// 平台费用
    PlatformFee,
}

/**
 * 交易状态枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TransactionStatus {
    /// 待处理
    Pending,
    /// 已完成
    Completed,
    /// 已失败
    Failed,
    /// 已取消
    Cancelled,
}

impl Default for TransactionStatus {
    fn default() -> Self {
        TransactionStatus::Pending
    }
}

// ================================
// 余额信息结构
// ================================

/**
 * 代币余额信息
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TokenBalance {
    /// 代币类型
    pub token_mint: Pubkey,
    
    /// 余额
    pub balance: u64,
    
    /// 冻结余额
    pub frozen_balance: u64,
    
    /// 可用余额
    pub available_balance: u64,
    
    /// 最后更新时间
    pub last_updated: i64,
}

/**
 * 收入记录
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct IncomeRecord {
    /// 收入金额
    pub amount: u64,
    
    /// 收入来源
    pub source: IncomeSource,
    
    /// 收入时间
    pub timestamp: i64,
    
    /// 交易ID
    pub transaction_id: String,
    
    /// 备注
    pub note: String,
}

/**
 * 收入来源枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum IncomeSource {
    /// 倍增奖励
    MultiplierReward,
    /// 转账接收
    Transfer,
    /// 代币铸造
    TokenMint,
    /// 提货券奖励
    VoucherReward,
    /// 推荐奖励
    ReferralReward,
    /// 其他收入
    Other,
}

/**
 * 支出记录
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ExpenseRecord {
    /// 支出金额
    pub amount: u64,
    
    /// 支出类别
    pub category: ExpenseCategory,
    
    /// 支出时间
    pub timestamp: i64,
    
    /// 交易ID
    pub transaction_id: String,
    
    /// 备注
    pub note: String,
}

/**
 * 支出类别枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ExpenseCategory {
    /// 提货券消费
    VoucherConsumption,
    /// 转账发送
    Transfer,
    /// 平台费用
    PlatformFee,
    /// 代币销毁
    TokenBurn,
    /// 其他支出
    Other,
}

/**
 * 奖励记录
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RewardRecord {
    /// 奖励金额
    pub amount: u64,
    
    /// 奖励类型
    pub reward_type: RewardType,
    
    /// 奖励时间
    pub timestamp: i64,
    
    /// 交易ID
    pub transaction_id: String,
    
    /// 奖励来源
    pub source: String,
    
    /// 备注
    pub note: String,
}

/**
 * 奖励类型枚举
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum RewardType {
    /// 倍增奖励
    MultiplierReward,
    /// 质量奖励
    QualityReward,
    /// 推荐奖励
    ReferralReward,
    /// 活跃奖励
    ActivityReward,
    /// 特殊奖励
    SpecialReward,
}

/**
 * 奖励分类统计
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RewardBreakdown {
    /// 奖励类型
    pub reward_type: RewardType,
    
    /// 奖励总额
    pub total_amount: u64,
    
    /// 奖励次数
    pub count: u64,
    
    /// 平均奖励
    pub average_amount: u64,
    
    /// 最后奖励时间
    pub last_reward_time: i64,
}

// ================================
// 实现
// ================================

impl UserWallet {
    /// 计算账户所需空间
    pub const SPACE: usize = ACCOUNT_DISCRIMINATOR_SIZE
        + PUBKEY_SIZE * 2  // owner, platform_token_account
        + U64_SIZE * 6     // total_income, total_expense, total_rewards, transaction_count, last_transaction_at
        + 1                // status enum
        + I64_SIZE * 2     // created_at, updated_at
        + U8_SIZE          // version
        + WalletPermissions::SPACE
        + WalletStatistics::SPACE;

    /// 初始化钱包
    pub fn initialize(
        &mut self,
        owner: Pubkey,
        platform_token_account: Pubkey,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        self.owner = owner;
        self.platform_token_account = platform_token_account;
        self.total_income = 0;
        self.total_expense = 0;
        self.total_rewards = 0;
        self.transaction_count = 0;
        self.last_transaction_at = current_time;
        self.status = WalletStatus::Active;
        self.created_at = current_time;
        self.updated_at = current_time;
        self.version = 1;
        self.permissions = WalletPermissions {
            can_receive: true,
            can_send: true,
            can_view_history: true,
            daily_transfer_limit: 1000000, // 1M tokens
            daily_transfer_used: 0,
            last_reset_date: current_time / SECONDS_PER_DAY,
        };
        self.statistics = WalletStatistics::default();
        
        Ok(())
    }

    /// 更新收入
    pub fn add_income(&mut self, amount: u64) -> Result<()> {
        self.total_income = self.total_income
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        self.transaction_count += 1;
        self.last_transaction_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        // 更新统计信息
        self.statistics.today_income = self.statistics.today_income
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        self.statistics.monthly_income = self.statistics.monthly_income
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        
        if amount > self.statistics.max_single_income {
            self.statistics.max_single_income = amount;
        }
        
        Ok(())
    }

    /// 更新支出
    pub fn add_expense(&mut self, amount: u64) -> Result<()> {
        self.total_expense = self.total_expense
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        self.transaction_count += 1;
        self.last_transaction_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        // 更新统计信息
        self.statistics.today_expense = self.statistics.today_expense
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        self.statistics.monthly_expense = self.statistics.monthly_expense
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        
        if amount > self.statistics.max_single_expense {
            self.statistics.max_single_expense = amount;
        }
        
        Ok(())
    }

    /// 更新奖励
    pub fn add_reward(&mut self, amount: u64) -> Result<()> {
        self.total_rewards = self.total_rewards
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        self.transaction_count += 1;
        self.last_transaction_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;
        
        // 更新统计信息
        self.statistics.today_rewards = self.statistics.today_rewards
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        self.statistics.monthly_rewards = self.statistics.monthly_rewards
            .checked_add(amount)
            .ok_or(crate::errors::SoonShopError::MathOverflow)?;
        
        Ok(())
    }

    /// 检查转账限额
    pub fn check_transfer_limit(&mut self, amount: u64) -> Result<()> {
        let current_day = Clock::get()?.unix_timestamp / SECONDS_PER_DAY;
        
        // 重置日限额
        if current_day > self.permissions.last_reset_date {
            self.permissions.daily_transfer_used = 0;
            self.permissions.last_reset_date = current_day;
        }
        
        // 检查是否超过限额
        if self.permissions.daily_transfer_used + amount > self.permissions.daily_transfer_limit {
            return Err(crate::errors::SoonShopError::InsufficientFunds.into());
        }
        
        self.permissions.daily_transfer_used += amount;
        Ok(())
    }
}

impl TransactionRecord {
    /// 计算账户所需空间
    pub const SPACE: usize = ACCOUNT_DISCRIMINATOR_SIZE
        + STRING_PREFIX_SIZE + 50  // id
        + PUBKEY_SIZE              // user
        + 1                        // transaction_type enum
        + U64_SIZE                 // amount
        + PUBKEY_SIZE              // token_mint
        + OPTION_FLAG_SIZE + PUBKEY_SIZE  // counterparty
        + STRING_PREFIX_SIZE + 200 // description
        + STRING_PREFIX_SIZE + 500 // metadata
        + 1                        // status enum
        + I64_SIZE                 // timestamp
        + U64_SIZE                 // block_height
        + STRING_PREFIX_SIZE + 100; // signature

    /// 初始化交易记录
    pub fn initialize(
        &mut self,
        id: String,
        user: Pubkey,
        transaction_type: TransactionType,
        amount: u64,
        token_mint: Pubkey,
        counterparty: Option<Pubkey>,
        description: String,
        metadata: String,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        
        self.id = id;
        self.user = user;
        self.transaction_type = transaction_type;
        self.amount = amount;
        self.token_mint = token_mint;
        self.counterparty = counterparty;
        self.description = description;
        self.metadata = metadata;
        self.status = TransactionStatus::Pending;
        self.timestamp = current_time;
        self.block_height = Clock::get()?.slot;
        self.signature = String::new();
        
        Ok(())
    }

    /// 完成交易
    pub fn complete(&mut self, signature: String) -> Result<()> {
        self.status = TransactionStatus::Completed;
        self.signature = signature;
        Ok(())
    }

    /// 失败交易
    pub fn fail(&mut self) -> Result<()> {
        self.status = TransactionStatus::Failed;
        Ok(())
    }
}

impl WalletPermissions {
    pub const SPACE: usize = BOOL_SIZE * 3 + U64_SIZE * 2 + I64_SIZE;
}

impl WalletStatistics {
    pub const SPACE: usize = U64_SIZE * 9 + I64_SIZE;
} 